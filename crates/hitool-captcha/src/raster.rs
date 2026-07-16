use std::io::Cursor;

use font8x8::{BASIC_FONTS, UnicodeFonts as _};
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use rand::Rng;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

/// Static PNG CAPTCHA renderer with bitmap glyphs and randomized interference.
#[derive(Debug, Clone)]
pub struct PngRenderer {
    width: u16,
    height: u16,
    noise_lines: u8,
    max_pixels: u32,
}

impl PngRenderer {
    /// Creates a bounded PNG renderer.
    pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
        let pixels = u32::from(width).saturating_mul(u32::from(height));
        if width < 80 || height < 30 || pixels > 4_000_000 {
            return Err(CaptchaError::InvalidDimensions);
        }
        Ok(Self {
            width,
            height,
            noise_lines: 8,
            max_pixels: 4_000_000,
        })
    }

    /// Sets the number of interference lines.
    #[must_use]
    pub const fn with_noise_lines(mut self, noise_lines: u8) -> Self {
        self.noise_lines = noise_lines;
        self
    }
}

impl Default for PngRenderer {
    fn default() -> Self {
        Self {
            width: 180,
            height: 60,
            noise_lines: 8,
            max_pixels: 4_000_000,
        }
    }
}

impl CaptchaRenderer for PngRenderer {
    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
        let glyph_count = code.chars().count();
        if glyph_count == 0 || glyph_count > 32 {
            return Err(CaptchaError::InvalidRenderCode);
        }
        let pixels = u32::from(self.width).saturating_mul(u32::from(self.height));
        if pixels > self.max_pixels {
            return Err(CaptchaError::MediaLimit("raster pixel count"));
        }
        let mut image = RgbaImage::from_pixel(
            u32::from(self.width),
            u32::from(self.height),
            Rgba([245, 247, 249, 255]),
        );
        let mut rng = rand::rng();
        for _ in 0..self.noise_lines {
            let start = (
                rng.random_range(0..i32::from(self.width)),
                rng.random_range(0..i32::from(self.height)),
            );
            let end = (
                rng.random_range(0..i32::from(self.width)),
                rng.random_range(0..i32::from(self.height)),
            );
            let color = random_color(&mut rng, 90);
            draw_line(&mut image, start, end, color);
        }

        let glyph_count =
            u32::try_from(glyph_count).map_err(|_| CaptchaError::InvalidRenderCode)?;
        let slot = u32::from(self.width) / (glyph_count + 1);
        let scale = (u32::from(self.height) / 11).clamp(2, 7);
        for (index, character) in code.chars().enumerate() {
            let bitmap = BASIC_FONTS
                .get(character)
                .or_else(|| BASIC_FONTS.get('?'))
                .ok_or(CaptchaError::InvalidRenderCode)?;
            let index = u32::try_from(index).map_err(|_| CaptchaError::InvalidRenderCode)?;
            let glyph_width = 8 * scale;
            let base_x = slot
                .saturating_mul(index + 1)
                .saturating_sub(glyph_width / 2);
            let max_y = u32::from(self.height).saturating_sub(8 * scale);
            let y = if max_y == 0 {
                0
            } else {
                rng.random_range(0..=max_y)
            };
            draw_glyph(
                &mut image,
                bitmap,
                base_x,
                y,
                scale,
                random_color(&mut rng, 35),
            );
        }

        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image).write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
        Ok(RenderedCaptcha::new("image/png", bytes))
    }
}

fn random_color(rng: &mut impl Rng, maximum: u8) -> Rgba<u8> {
    Rgba([
        rng.random_range(0..=maximum),
        rng.random_range(0..=maximum),
        rng.random_range(0..=maximum),
        255,
    ])
}

fn draw_glyph(image: &mut RgbaImage, bitmap: [u8; 8], x: u32, y: u32, scale: u32, color: Rgba<u8>) {
    for (row, bits) in bitmap.iter().enumerate() {
        for column in 0..8_u32 {
            if bits & (1 << column) == 0 {
                continue;
            }
            let row = u32::try_from(row).expect("bitmap has exactly eight rows");
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

fn draw_line(image: &mut RgbaImage, start: (i32, i32), end: (i32, i32), color: Rgba<u8>) {
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        if let (Ok(x), Ok(y)) = (u32::try_from(x0), u32::try_from(y0)) {
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, color);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let twice = 2 * error;
        if twice >= dy {
            error += dy;
            x0 += sx;
        }
        if twice <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_decodable_png_with_expected_dimensions() {
        let artifact = PngRenderer::default().render("A2B9").unwrap();
        assert_eq!(artifact.mime_type(), "image/png");
        assert!(artifact.bytes().starts_with(b"\x89PNG\r\n\x1a\n"));
        let decoded = image::load_from_memory(artifact.bytes()).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (180, 60));
    }
}
