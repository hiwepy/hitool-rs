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

use super::abstract_captcha::AbstractCaptcha;
use super::captcha_color::CaptchaColor;

/// Common operations exposed by all Hutool-style CAPTCHA variants.
pub trait ICaptcha {
    /// Generates a fresh challenge.
    fn create_code(&mut self) -> Result<(), CaptchaError>;
    /// Returns the generated challenge text.
    fn code(&mut self) -> Result<&str, CaptchaError>;
    /// Verifies user input.
    fn verify(&self, input: &str) -> bool;
    /// Returns encoded image bytes.
    fn image_bytes(&mut self) -> Result<&[u8], CaptchaError>;
}

impl ICaptcha for AbstractCaptcha {
    fn create_code(&mut self) -> Result<(), CaptchaError> {
        AbstractCaptcha::create_code(self)
    }

    fn code(&mut self) -> Result<&str, CaptchaError> {
        AbstractCaptcha::code(self)
    }

    fn verify(&self, input: &str) -> bool {
        AbstractCaptcha::verify(self, input)
    }

    fn image_bytes(&mut self) -> Result<&[u8], CaptchaError> {
        AbstractCaptcha::image_bytes(self)
    }
}

impl ShearCaptcha {
    /// Creates a shear CAPTCHA with the default line thickness.
    pub fn with_count(width: u16, height: u16, code_count: usize) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, code_count, 4)
    }
}

impl GifCaptcha {
    /// Sets GIF color-quantization speed/quality.
    ///
    /// Values below 1 are clamped to 1 (Hutool `GifCaptcha.setQuality`).
    #[must_use]
    pub fn set_quality(mut self, quality: u8) -> Self {
        self.0.gif_quality = quality.clamp(1, 30);
        self.0.invalidate();
        self
    }

    /// Returns the GIF quantization quality (Hutool `quality` field).
    #[must_use]
    pub const fn quality(&self) -> u8 {
        self.0.gif_quality
    }

    /// Sets GIF repetition count; zero means infinite.
    ///
    /// Negative values are clamped to `0` (Hutool `GifCaptcha.setRepeat` / `Math.max`).
    #[must_use]
    pub fn set_repeat(mut self, repeat: i32) -> Self {
        self.0.gif_repeat = if repeat < 0 {
            0
        } else {
            u16::try_from(repeat).unwrap_or(u16::MAX)
        };
        self.0.invalidate();
        self
    }

    /// Returns the GIF frame repeat count (Hutool `repeat` field).
    #[must_use]
    pub const fn repeat(&self) -> u16 {
        self.0.gif_repeat
    }

    /// Sets the maximum random text color component.
    #[must_use]
    pub fn set_max_color(mut self, maximum: u8) -> Self {
        self.0.max_color = maximum;
        self.0.invalidate();
        self
    }

    /// Returns the maximum random text color component (Hutool `maxColor`).
    #[must_use]
    pub const fn max_color(&self) -> u8 {
        self.0.max_color
    }

    /// Sets the minimum random text color component.
    #[must_use]
    pub fn set_min_color(mut self, minimum: u8) -> Self {
        self.0.min_color = minimum;
        self.0.invalidate();
        self
    }

    /// Returns the minimum random text color component (Hutool `minColor`).
    #[must_use]
    pub const fn min_color(&self) -> u8 {
        self.0.min_color
    }

    /// Samples a random RGB color in `[min, max]` (Hutool `GifCaptcha.getRandomColor`).
    #[must_use]
    pub fn random_color(min: u8, max: u8) -> CaptchaColor {
        let (lo, hi) = if min <= max { (min, max) } else { (0, 255) };
        let mut rng = rand::rng();
        CaptchaColor([
            rng.random_range(lo..=hi),
            rng.random_range(lo..=hi),
            rng.random_range(lo..=hi),
            255,
        ])
    }
}

macro_rules! captcha_type {
    ($name:ident, $kind:expr, $default_interference:expr) => {
        #[doc = concat!("Hutool-compatible `", stringify!($name), "`.")]
        #[derive(Debug)]
        pub struct $name(AbstractCaptcha);

        impl $name {
            /// Creates a five-character CAPTCHA with default interference.
            pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
                Self::with_code_count(width, height, 5, $default_interference)
            }

            /// Creates a CAPTCHA with an explicit code and interference count.
            pub fn with_code_count(
                width: u16,
                height: u16,
                code_count: usize,
                interference: u16,
            ) -> Result<Self, CaptchaError> {
                let generator = Arc::new(RandomGenerator::new(code_count)?);
                Self::with_generator(width, height, generator, interference)
            }

            /// Creates a CAPTCHA with an injected generator.
            pub fn with_generator(
                width: u16,
                height: u16,
                generator: Arc<dyn CodeGenerator>,
                interference: u16,
            ) -> Result<Self, CaptchaError> {
                AbstractCaptcha::new(width, height, generator, interference, 0.75, $kind).map(Self)
            }

            /// Creates a CAPTCHA with a Hutool-style font-size multiplier.
            pub fn with_size(
                width: u16,
                height: u16,
                code_count: usize,
                interference: u16,
                size: f32,
            ) -> Result<Self, CaptchaError> {
                let generator = Arc::new(RandomGenerator::new(code_count)?);
                AbstractCaptcha::new(width, height, generator, interference, size, $kind).map(Self)
            }

            /// Renders a supplied code without mutating challenge state.
            pub fn create_image(&self, code: &str) -> Result<DynamicImage, CaptchaError> {
                let rendered = self.0.render(code)?;
                Ok(image::load_from_memory(rendered.bytes())
                    .expect("the compatibility renderer emits valid PNG or GIF bytes"))
            }
        }

        impl Deref for $name {
            type Target = AbstractCaptcha;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
