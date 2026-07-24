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

/// Shared state used by Hutool-style random generators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGenerator {
    alphabet: Vec<u8>,
    length: usize,
}

impl AbstractGenerator {
    /// Creates a generator configuration using Hutool's alphanumeric alphabet.
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        Self::with_alphabet(HUTOOL_ALPHABET, length)
    }

    /// Creates a generator configuration with a caller-supplied ASCII alphabet.
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        if length == 0 {
            return Err(CaptchaError::InvalidLength);
        }
        if alphabet.is_empty() || !alphabet.is_ascii() {
            return Err(CaptchaError::InvalidAlphabet);
        }
        Ok(Self {
            alphabet: alphabet.to_vec(),
            length,
        })
    }

    /// Returns the generated code length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }
}

const HUTOOL_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
