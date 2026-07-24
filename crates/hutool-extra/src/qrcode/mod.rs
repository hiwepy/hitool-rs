//! QR code SVG/PNG/ASCII rendering with Hutool-named facades.
//!
//! 对齐: `cn.hutool.extra.qrcode.QrCodeUtil`
//! 对齐: `cn.hutool.extra.qrcode.QrConfig`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/qrcode/

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

mod error_correction;
mod qr_config;
mod qr_code_util;

pub use error_correction::ErrorCorrection;
pub use qr_config::QrConfig;
pub use qr_code_util::QrCodeUtil;
pub use error_correction::to_svg;
pub use error_correction::to_svg_with_level;
pub use qr_code_util::to_ascii_art;
pub use qr_code_util::to_ascii_art_with_level;
pub use qr_code_util::to_png;
pub use qr_code_util::to_png_with_config;
