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

/// Arithmetic CAPTCHA generator using addition, subtraction, and multiplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathGenerator {
    number_length: u8,
    allow_negative: bool,
}

impl MathGenerator {
    /// Creates an arithmetic generator.
    pub fn new(number_length: u8, allow_negative: bool) -> Result<Self, CaptchaError> {
        if number_length == 0 || number_length > 8 {
            return Err(CaptchaError::InvalidLength);
        }
        Ok(Self {
            number_length,
            allow_negative,
        })
    }

    /// Creates a generator with Hutool's default operand width.
    pub fn with_negative_results(allow_negative: bool) -> Self {
        Self {
            number_length: 2,
            allow_negative,
        }
    }

    /// Returns the formatted challenge length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.number_length as usize * 2 + 2
    }

    /// Evaluates an arithmetic CAPTCHA expression produced by [`Self::generate`].
    ///
    /// Mirrors Hutool's `Calculator.conversion(code)` used by `MathGenerator.verify`.
    #[must_use]
    pub fn evaluate(code: &str) -> Option<i64> {
        let expression = code.strip_suffix('=')?;
        for operator in ['+', '-', '*'] {
            if let Some((left, right)) = expression.split_once(operator) {
                let left = left.trim().parse::<i64>().ok()?;
                let right = right.trim().parse::<i64>().ok()?;
                return if operator == '+' {
                    left.checked_add(right)
                } else if operator == '-' {
                    left.checked_sub(right)
                } else {
                    left.checked_mul(right)
                };
            }
        }
        None
    }
}

impl Default for MathGenerator {
    fn default() -> Self {
        Self::with_negative_results(true)
    }
}

impl CodeGenerator for MathGenerator {
    fn generate(&self) -> String {
        let limit = 10_i64.pow(u32::from(self.number_length));
        let mut rng = rand::rng();
        let left = rng.random_range(0..limit);
        let operator = ['+', '-', '*'][rng.random_range(0..3)];
        let right = if !self.allow_negative && operator == '-' {
            rng.random_range(0..=left)
        } else {
            rng.random_range(0..limit)
        };
        format!(
            "{left:<width$}{operator}{right:<width$}=",
            width = usize::from(self.number_length)
        )
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        input.trim().parse::<i64>().ok() == Self::evaluate(generated)
    }
}
