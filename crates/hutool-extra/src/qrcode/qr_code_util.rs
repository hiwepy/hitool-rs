//! QR code SVG/PNG/ASCII rendering with Hutool-named facades.
//!
//! 对齐: `cn.hutool.extra.qrcode.QrCodeUtil`
//! 对齐: `cn.hutool.extra.qrcode.QrConfig`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/qrcode/

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

use super::error_correction::ErrorCorrection;
use super::qr_config::QrConfig;

/// Hutool `QrCodeUtil` facade over SVG/ASCII/(optional) PNG helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.qrcode.QrCodeUtil`
pub struct QrCodeUtil;

impl QrCodeUtil {
    /// SVG target type constant (Hutool `QR_TYPE_SVG`).
    pub const QR_TYPE_SVG: &'static str = "svg";
    /// ASCII-art target type constant (Hutool `QR_TYPE_TXT`).
    pub const QR_TYPE_TXT: &'static str = "txt";

    /// Generates SVG (Hutool `generateAsSvg`).
    pub fn generate_as_svg(content: impl AsRef<[u8]>, config: &QrConfig) -> Result<String> {
        render_svg(content.as_ref(), config)
    }

    /// Generates SVG with default 300×300 config (Hutool overload).
    pub fn generate_as_svg_default(content: impl AsRef<[u8]>) -> Result<String> {
        Self::generate_as_svg(content, &QrConfig::create())
    }

    /// Generates ASCII art (Hutool `generateAsAsciiArt`).
    pub fn generate_as_ascii_art(content: impl AsRef<[u8]>) -> Result<String> {
        to_ascii_art(content)
    }

    /// Generates ASCII art with config-driven error correction.
    pub fn generate_as_ascii_art_with_config(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
    ) -> Result<String> {
        to_ascii_art_with_level(content, config.error_correction)
    }

    /// Writes SVG to a file path (Hutool `generate(..., File)` SVG path).
    pub fn generate_to_file(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let svg = Self::generate_as_svg(content, config)?;
        std::fs::write(path, svg)?;
        Ok(())
    }

    /// Generates PNG bytes (Hutool `generatePng`) when `image` is enabled.
    #[cfg(feature = "image")]
    pub fn generate_png(content: impl AsRef<[u8]>, width: u32, height: u32) -> Result<Vec<u8>> {
        to_png(content, width, height)
    }

    /// Generates PNG from config (Hutool `generatePng(content, config)`).
    #[cfg(feature = "image")]
    pub fn generate_png_with_config(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
    ) -> Result<Vec<u8>> {
        to_png_with_config(content, config)
    }

    /// Base64 data-URL for SVG (Hutool `generateAsBase64` SVG subset).
    pub fn generate_as_base64_svg(content: impl AsRef<[u8]>, config: &QrConfig) -> Result<String> {
        let svg = Self::generate_as_svg(content, config)?;
        Ok(format!(
            "data:image/svg+xml;base64,{}",
            base64_encode(svg.as_bytes())
        ))
    }
}

pub(crate) fn to_ascii_art_with_level(

pub(crate) fn to_ascii_art(data: impl AsRef<[u8]>) -> Result<String> {
    to_ascii_art_with_level(data, ErrorCorrection::Medium)
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let mut buf = [0_u8; 3];
        for (i, b) in chunk.iter().enumerate() {
            buf[i] = *b;
        }
        let n = chunk.len();
        let x = (u32::from(buf[0]) << 16) | (u32::from(buf[1]) << 8) | u32::from(buf[2]);
        out.push(TABLE[((x >> 18) & 63) as usize] as char);
        out.push(TABLE[((x >> 12) & 63) as usize] as char);
        out.push(if n > 1 {
            TABLE[((x >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if n > 2 {
            TABLE[(x & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn render_svg(data: &[u8], config: &QrConfig) -> Result<String> {
    let code = QrCode::with_error_correction_level(data, config.error_correction.to_ec_level())?;
    Ok(code
        .render::<svg::Color<'_>>()
        .min_dimensions(config.min_dimension(), config.min_dimension())
        .dark_color(svg::Color(&config.fore_color))
        .light_color(svg::Color(&config.back_color))
        .build())
}

pub(crate) fn to_png_with_config(data: impl AsRef<[u8]>, config: &QrConfig) -> Result<Vec<u8>> {
    use ::image::{ImageBuffer, ImageFormat, Luma};
    use std::io::Cursor;

    let code = QrCode::with_error_correction_level(data.as_ref(), config.error_correction.to_ec_level())?;
    let colors = code.to_colors();
    let modules = code.width();
    let dim = config.min_dimension().max(modules as u32);
    let scale = (dim / modules as u32).max(1);
    let side = modules as u32 * scale;
    let mut buffer = ImageBuffer::from_pixel(side, side, Luma([255_u8]));
    for y in 0..modules {
        for x in 0..modules {
            if colors[y * modules + x] == ::qrcode::Color::Dark {
                for dy in 0..scale {
                    for dx in 0..scale {
                        buffer.put_pixel(x as u32 * scale + dx, y as u32 * scale + dy, Luma([0]));
                    }
                }
            }
        }
    }
    let mut bytes = Vec::new();
    ::image::DynamicImage::ImageLuma8(buffer)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    Ok(bytes)
}

pub(crate) fn to_png(data: impl AsRef<[u8]>, width: u32, height: u32) -> Result<Vec<u8>> {
    to_png_with_config(data, &QrConfig::new(width, height))
}
