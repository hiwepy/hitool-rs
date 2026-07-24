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

use super::abstract_generator::AbstractGenerator;

/// Hutool-compatible random-character generator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomGenerator(AbstractGenerator);

impl RandomGenerator {
    /// Creates an alphanumeric generator.
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::new(length).map(Self)
    }

    /// Creates a generator using a custom ASCII alphabet.
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::with_alphabet(alphabet, length).map(Self)
    }

    /// Returns the generated code length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.0.length()
    }
}

impl CodeGenerator for RandomGenerator {
    fn generate(&self) -> String {
        let mut rng = rand::rng();
        (0..self.0.length)
            .map(|_| {
                let index = rng.random_range(0..self.0.alphabet.len());
                char::from(self.0.alphabet[index])
            })
            .collect()
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        !input.trim().is_empty()
            && constant_time_ascii_case_eq(generated.as_bytes(), input.trim().as_bytes())
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new(5).expect("the default CAPTCHA length is valid")
    }
}
