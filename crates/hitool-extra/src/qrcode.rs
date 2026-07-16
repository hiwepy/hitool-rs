//! QR code SVG rendering.

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

/// QR error-correction strength.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// About 7% restoration capability.
    Low,
    /// About 15% restoration capability.
    #[default]
    Medium,
    /// About 25% restoration capability.
    Quartile,
    /// About 30% restoration capability.
    High,
}

/// Renders data as a standalone SVG document.
pub fn to_svg(data: impl AsRef<[u8]>, minimum_width: u32) -> Result<String> {
    to_svg_with_level(data, minimum_width, ErrorCorrection::Medium)
}

/// Renders data as SVG with an explicit correction level.
pub fn to_svg_with_level(
    data: impl AsRef<[u8]>,
    minimum_width: u32,
    level: ErrorCorrection,
) -> Result<String> {
    let level = match level {
        ErrorCorrection::Low => EcLevel::L,
        ErrorCorrection::Medium => EcLevel::M,
        ErrorCorrection::Quartile => EcLevel::Q,
        ErrorCorrection::High => EcLevel::H,
    };
    let code = QrCode::with_error_correction_level(data, level)?;
    Ok(code
        .render::<svg::Color<'_>>()
        .min_dimensions(minimum_width, minimum_width)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_standalone_svg() {
        let svg = to_svg("https://example.com", 256).unwrap();
        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("width=\""));
    }
}
