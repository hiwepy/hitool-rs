//! QR code SVG/PNG/ASCII rendering with Hutool-named facades.
//!
//! 对齐: `cn.hutool.extra.qrcode.QrCodeUtil`
//! 对齐: `cn.hutool.extra.qrcode.QrConfig`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/qrcode/

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

/// QR error-correction strength.
///
/// 对齐 Java: `com.google.zxing.qrcode.decoder.ErrorCorrectionLevel`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// About 7% restoration capability (`L`).
    Low,
    /// About 15% restoration capability (`M`).
    #[default]
    Medium,
    /// About 25% restoration capability (`Q`).
    Quartile,
    /// About 30% restoration capability (`H`).
    High,
}

impl ErrorCorrection {
    fn to_ec_level(self) -> EcLevel {
        match self {
            Self::Low => EcLevel::L,
            Self::Medium => EcLevel::M,
            Self::Quartile => EcLevel::Q,
            Self::High => EcLevel::H,
        }
    }
}
