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

mod abstract_generator;
mod random_generator;
mod math_generator;
mod captcha_color;
mod captcha_font;
mod captcha_stroke;
mod abstract_captcha;
mod i_captcha;
mod captcha_util;

pub use abstract_generator::AbstractGenerator;
pub use random_generator::RandomGenerator;
pub use math_generator::MathGenerator;
pub use captcha_color::CaptchaColor;
pub use captcha_font::CaptchaFont;
pub use captcha_stroke::CaptchaStroke;
pub use abstract_captcha::AbstractCaptcha;
pub use i_captcha::ICaptcha;
pub use captcha_util::CaptchaUtil;
