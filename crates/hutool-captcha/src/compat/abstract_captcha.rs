//! Hutool-named raster CAPTCHA facade.

use std::fmt;
use std::fs;
use std::io::{Cursor, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::Arc;

use base64::Engine as _;
use font8x8::{BASIC_FONTS, UnicodeFonts as _};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, DynamicImage, Frame, ImageFormat, Rgba, RgbaImage};
use rand::Rng;

use crate::{CaptchaError, CodeGenerator, RenderedCaptcha, constant_time_ascii_case_eq};

use super::captcha_color::CaptchaColor;
use super::captcha_font::CaptchaFont;
use super::captcha_stroke::CaptchaStroke;

/// Common mutable CAPTCHA state corresponding to Hutool's `AbstractCaptcha`.
pub struct AbstractCaptcha {
    width: u16,
    height: u16,
    generator: Arc<dyn CodeGenerator>,
    interfere_count: u16,
    font: CaptchaFont,
    background: CaptchaColor,
    text_alpha: u8,
    stroke: CaptchaStroke,
    kind: CaptchaKind,
    code: Option<String>,
    rendered: Option<RenderedCaptcha>,
    gif_quality: u8,
    gif_repeat: u16,
    min_color: u8,
    max_color: u8,
}

impl fmt::Debug for AbstractCaptcha {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractCaptcha")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("interfere_count", &self.interfere_count)
            .field("kind", &self.kind)
            .field("generated", &self.code.is_some())
            .finish_non_exhaustive()
    }
}

impl AbstractCaptcha {
    fn new(
        width: u16,
        height: u16,
        generator: Arc<dyn CodeGenerator>,
        interfere_count: u16,
        font_scale: f32,
        kind: CaptchaKind,
    ) -> Result<Self, CaptchaError> {
        let pixels = u32::from(width).saturating_mul(u32::from(height));
        if width < 32 || height < 16 || pixels > 4_000_000 {
            return Err(CaptchaError::InvalidDimensions);
        }
        let requested_scale = (f32::from(height) * font_scale / 8.0)
            .round()
            .clamp(1.0, 12.0);
        let scale = (1_u8..=12)
            .find(|candidate| f32::from(*candidate) >= requested_scale)
            .unwrap_or(12);
        Ok(Self {
            width,
            height,
            generator,
            interfere_count,
            font: CaptchaFont { scale },
            background: CaptchaColor([255, 255, 255, 255]),
            text_alpha: 255,
            stroke: CaptchaStroke { width: 1 },
            kind,
            code: None,
            rendered: None,
            gif_quality: 10,
            gif_repeat: 0,
            min_color: 0,
            max_color: 255,
        })
    }

    /// Generates a new code and encoded image.
    pub fn create_code(&mut self) -> Result<(), CaptchaError> {
        let code = self.generator.generate();
        let rendered = self.render(&code)?;
        self.code = Some(code);
        self.rendered = Some(rendered);
        Ok(())
    }

    /// Returns the generated code, generating it lazily when necessary.
    pub fn code(&mut self) -> Result<&str, CaptchaError> {
        self.ensure_generated()?;
        self.code.as_deref().ok_or(CaptchaError::InvalidRenderCode)
    }

    /// Verifies user input against the current generated code.
    pub fn verify(&self, input: &str) -> bool {
        self.code
            .as_deref()
            .is_some_and(|code| self.generator.verify(code, input))
    }

    /// Writes encoded image bytes to a filesystem path.
    pub fn write_to_path(&mut self, path: &Path) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        fs::write(path, bytes).map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// Writes encoded image bytes to a stream.
    pub fn write_to(&mut self, output: &mut dyn Write) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        output
            .write_all(bytes)
            .map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// Returns encoded PNG or GIF bytes.
    pub fn image_bytes(&mut self) -> Result<&[u8], CaptchaError> {
        self.ensure_generated()?;
        self.rendered
            .as_ref()
            .map(RenderedCaptcha::bytes)
            .ok_or(CaptchaError::InvalidRenderCode)
    }

    /// Decodes and returns the current raster image.
    pub fn image(&mut self) -> Result<DynamicImage, CaptchaError> {
        image::load_from_memory(self.image_bytes()?).map_err(CaptchaError::from)
    }

    /// Returns the image bytes as standard Base64.
    pub fn image_base64(&mut self) -> Result<String, CaptchaError> {
        Ok(base64::engine::general_purpose::STANDARD.encode(self.image_bytes()?))
    }

    /// Returns a browser-ready image data URI.
    pub fn image_base64_data(&mut self) -> Result<String, CaptchaError> {
        self.ensure_generated()?;
        let mime = if self.kind == CaptchaKind::Gif {
            "image/gif"
        } else {
            "image/png"
        };
        let bytes = self.rendered_bytes_invariant();
        Ok(format!(
            "data:{mime};base64,{}",
            base64::engine::general_purpose::STANDARD.encode(bytes)
        ))
    }

    /// Replaces the bitmap font configuration and invalidates rendered media.
    pub fn set_font(&mut self, font: CaptchaFont) -> &mut Self {
        self.font = CaptchaFont {
            scale: font.scale.clamp(1, 12),
        };
        self.invalidate();
        self
    }

    /// Returns the active code generator.
    #[must_use]
    pub fn generator(&self) -> &dyn CodeGenerator {
        self.generator.as_ref()
    }

    /// Replaces the code generator and invalidates the existing challenge.
    pub fn set_generator(&mut self, generator: Arc<dyn CodeGenerator>) -> &mut Self {
        self.generator = generator;
        self.invalidate();
        self
    }

    /// Sets the background color.
    pub fn set_background(&mut self, background: CaptchaColor) -> &mut Self {
        self.background = background;
        self.invalidate();
        self
    }

    /// Sets text alpha in the inclusive range 0.0..=1.0.
    pub fn set_text_alpha(&mut self, alpha: f32) -> &mut Self {
        let requested_alpha = (alpha.clamp(0.0, 1.0) * 255.0).round();
        self.text_alpha = (0_u8..=u8::MAX)
            .find(|candidate| f32::from(*candidate) >= requested_alpha)
            .unwrap_or(u8::MAX);
        self.invalidate();
        self
    }

    /// Sets interference stroke width.
    pub fn set_stroke(&mut self, stroke: CaptchaStroke) -> &mut Self {
        self.stroke = CaptchaStroke {
            width: stroke.width.clamp(1, 16),
        };
        self.invalidate();
        self
    }

    fn invalidate(&mut self) {
        self.code = None;
        self.rendered = None;
    }

    fn rendered_bytes_invariant(&self) -> &[u8] {
        self.rendered
            .as_ref()
            .expect("generation stores rendered media")
            .bytes()
    }

    fn ensure_generated(&mut self) -> Result<(), CaptchaError> {
        if self.rendered.is_none() {
            self.create_code()?;
        }
        Ok(())
    }

    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
        if code.is_empty() || code.chars().count() > 32 {
            return Err(CaptchaError::InvalidRenderCode);
        }
        if self.kind == CaptchaKind::Gif {
            return Ok(self.render_gif(code));
        }
        let image = self.render_frame(code, 255);
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .expect("encoding an in-memory RGBA image as PNG is infallible");
        Ok(RenderedCaptcha::new("image/png", bytes))
    }

    fn render_gif(&self, code: &str) -> RenderedCaptcha {
        let mut bytes = Vec::new();
        {
            let mut encoder =
                GifEncoder::new_with_speed(&mut bytes, i32::from(self.gif_quality.clamp(1, 30)));
            let repeat = if self.gif_repeat == 0 {
                Repeat::Infinite
            } else {
                Repeat::Finite(self.gif_repeat)
            };
            encoder
                .set_repeat(repeat)
                .expect("writing GIF metadata to a Vec is infallible");
            let count = code.chars().count().max(1);
            for index in 0..count {
                let alpha = u8::try_from(((index + 1) * 255) / count).unwrap_or(255);
                let frame = Frame::from_parts(
                    self.render_frame(code, alpha),
                    0,
                    0,
                    Delay::from_numer_denom_ms(100, 1),
                );
                encoder
                    .encode_frame(frame)
                    .expect("writing a bounded RGBA frame to a Vec is infallible");
            }
        }
        RenderedCaptcha::new("image/gif", bytes)
    }

    fn render_frame(&self, code: &str, frame_alpha: u8) -> RgbaImage {
        let mut image = RgbaImage::from_pixel(
            u32::from(self.width),
            u32::from(self.height),
            Rgba(self.background.0),
        );
        let mut rng = rand::rng();
        self.draw_interference(&mut image, &mut rng);
        let glyph_count = u32::try_from(code.chars().count()).unwrap_or(1);
        let slot = u32::from(self.width) / (glyph_count + 1);
        for (index, character) in code.chars().enumerate() {
            let bitmap = BASIC_FONTS
                .get(character)
                .or_else(|| BASIC_FONTS.get('?'))
                .unwrap_or([0; 8]);
            let index = u32::try_from(index).unwrap_or(0);
            let scale = u32::from(self.font.scale);
            let x = slot.saturating_mul(index + 1).saturating_sub(4 * scale);
            let y = u32::from(self.height).saturating_sub(8 * scale) / 2;
            let color = self.random_text_color(&mut rng, frame_alpha);
            draw_glyph(&mut image, bitmap, x, y, scale, color);
        }
        image
    }

    fn random_text_color(&self, rng: &mut impl Rng, frame_alpha: u8) -> Rgba<u8> {
        let (min, max) = if self.min_color <= self.max_color {
            (self.min_color, self.max_color)
        } else {
            (0, 255)
        };
        Rgba([
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            self.text_alpha.min(frame_alpha),
        ])
    }

    fn draw_interference(&self, image: &mut RgbaImage, rng: &mut impl Rng) {
        let width = i32::from(self.width);
        let height = i32::from(self.height);
        for _ in 0..self.interfere_count {
            let color = Rgba([
                rng.random_range(0..=180),
                rng.random_range(0..=180),
                rng.random_range(0..=180),
                180,
            ]);
            match self.kind {
                CaptchaKind::Circle => {
                    let radius = rng.random_range(1..=(height / 4).max(1));
                    draw_circle(
                        image,
                        rng.random_range(0..width),
                        rng.random_range(0..height),
                        radius,
                        color,
                    );
                }
                CaptchaKind::Line | CaptchaKind::Shear | CaptchaKind::Gif => {
                    let thickness = if self.kind == CaptchaKind::Shear {
                        self.interfere_count.clamp(1, 16) as u8
                    } else {
                        self.stroke.width
                    };
                    draw_line(
                        image,
                        (rng.random_range(0..width), rng.random_range(0..height)),
                        (rng.random_range(0..width), rng.random_range(0..height)),
                        thickness,
                        color,
                    );
                }
            }
        }
    }
}

fn draw_circle(image: &mut RgbaImage, center_x: i32, center_y: i32, radius: i32, color: Rgba<u8>) {
    for offset_x in -radius..=radius {
        let target = radius * radius - offset_x * offset_x;
        let mut offset_y = 0;
        while (offset_y + 1) * (offset_y + 1) <= target {
            offset_y += 1;
        }
        put_pixel_checked(image, center_x + offset_x, center_y + offset_y, color);
        put_pixel_checked(image, center_x + offset_x, center_y - offset_y, color);
    }
}

fn draw_line(

fn draw_glyph(image: &mut RgbaImage, bitmap: [u8; 8], x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    for (row, bits) in bitmap.iter().enumerate() {
        for column in 0..8_u32 {
            if bits & (1 << column) == 0 {
                continue;
            }
            let row = u32::try_from(row).expect("bitmap has eight rows");
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    let pixel_x = x + column * scale + offset_x;
                    let pixel_y = y + row * scale + offset_y;
                    if pixel_x < image.width() && pixel_y < image.height() {
                        image.put_pixel(pixel_x, pixel_y, color);
                    }
                }
            }
        }
    }
}

enum CaptchaKind {
    Line,
    Circle,
    Shear,
    Gif,
}
