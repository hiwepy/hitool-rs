//! QR code SVG/PNG/ASCII rendering with Hutool-named facades.
//!
//! 对齐: `cn.hutool.extra.qrcode.QrCodeUtil`
//! 对齐: `cn.hutool.extra.qrcode.QrConfig`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/qrcode/

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

use super::error_correction::ErrorCorrection;

/// Hutool `QrConfig` — size and error-correction for generation helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.qrcode.QrConfig`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QrConfig {
    /// Width in pixels (SVG min dimension).
    pub width: u32,
    /// Height in pixels (SVG uses `max(width, height)` for square modules).
    pub height: u32,
    /// Error correction level.
    pub error_correction: ErrorCorrection,
    /// Foreground CSS color (SVG dark modules).
    pub fore_color: String,
    /// Background CSS color (SVG light modules).
    pub back_color: String,
}

impl Default for QrConfig {
    fn default() -> Self {
        Self::new(300, 300)
    }
}

impl QrConfig {
    /// Creates a config with Hutool's default 300×300 size (Hutool `QrConfig.create()`).
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Creates a config with explicit dimensions (Hutool `QrConfig(width, height)`).
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            error_correction: ErrorCorrection::Medium,
            fore_color: "#000000".into(),
            back_color: "#ffffff".into(),
        }
    }

    /// Sets width (Hutool `setWidth`).
    #[must_use]
    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width.max(1);
        self
    }

    /// Sets height (Hutool `setHeight`).
    #[must_use]
    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height.max(1);
        self
    }

    /// Sets error correction (Hutool `setErrorCorrection`).
    #[must_use]
    pub fn set_error_correction(mut self, level: ErrorCorrection) -> Self {
        self.error_correction = level;
        self
    }

    /// Sets foreground color hex (Hutool `setForeColor`).
    #[must_use]
    pub fn set_fore_color(mut self, color: impl Into<String>) -> Self {
        self.fore_color = color.into();
        self
    }

    /// Sets background color hex (Hutool `setBackColor`).
    #[must_use]
    pub fn set_back_color(mut self, color: impl Into<String>) -> Self {
        self.back_color = color.into();
        self
    }

    fn min_dimension(&self) -> u32 {
        self.width.min(self.height).max(1)
    }
}
