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

/// Hutool-named factory facade.
pub struct CaptchaUtil;

impl CaptchaUtil {
    /// Creates a line CAPTCHA.
    pub fn create_line_captcha(width: u16, height: u16) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::new(width, height)
    }

    /// Creates a line CAPTCHA with explicit code and interference counts.
    pub fn create_line_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_code_count(width, height, code_count, line_count)
    }

    /// Creates a line CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_line_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
        size: f32,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_size(width, height, code_count, line_count, size)
    }

    /// Creates a circle CAPTCHA.
    pub fn create_circle_captcha(width: u16, height: u16) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::new(width, height)
    }

    /// Creates a circle CAPTCHA with explicit code and interference counts.
    pub fn create_circle_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_code_count(width, height, code_count, circle_count)
    }

    /// Creates a circle CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_circle_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
        size: f32,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_size(width, height, code_count, circle_count, size)
    }

    /// Creates a shear CAPTCHA.
    pub fn create_shear_captcha(width: u16, height: u16) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::new(width, height)
    }

    /// Creates a shear CAPTCHA with explicit code and thickness.
    pub fn create_shear_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_code_count(width, height, code_count, thickness)
    }

    /// Creates a shear CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_shear_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
        size: f32,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_size(width, height, code_count, thickness, size)
    }

    /// Creates an animated GIF CAPTCHA.
    pub fn create_gif_captcha(width: u16, height: u16) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::new(width, height)
    }

    /// Creates a GIF CAPTCHA with an explicit code count (default interference 10).
    pub fn create_gif_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
    ) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::with_code_count(width, height, code_count, 10)
    }

    /// Creates a GIF CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_gif_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
        size: f32,
    ) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::with_size(width, height, code_count, thickness, size)
    }
}
