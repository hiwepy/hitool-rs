//! Bounded image decoding, resizing, cropping, and encoding.

use std::io::Cursor;

use ::image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{ExtraError, Result};

/// Defensive limits applied to encoded and decoded images.
#[derive(Debug, Clone, Copy)]
pub struct ImageLimits {
    /// Maximum encoded input bytes.
    pub max_input_bytes: usize,
    /// Maximum decoded width.
    pub max_width: u32,
    /// Maximum decoded height.
    pub max_height: u32,
    /// Maximum decoded pixel count.
    pub max_pixels: u64,
    /// Maximum encoded output bytes.
    pub max_output_bytes: usize,
}

impl Default for ImageLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: 16 * 1024 * 1024,
            max_width: 8_192,
            max_height: 8_192,
            max_pixels: 40_000_000,
            max_output_bytes: 32 * 1024 * 1024,
        }
    }
}

/// Geometry strategy used by [`resize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    /// Preserve aspect ratio and fit entirely inside the target box.
    Fit,
    /// Preserve aspect ratio and crop overflow to fill the target box.
    FillCrop,
    /// Ignore aspect ratio and force exact dimensions.
    Exact,
}

/// Encoded image output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Portable Network Graphics.
    Png,
    /// JPEG with quality from 1 through 100.
    Jpeg(u8),
    /// Lossless WebP.
    WebP,
}

/// Returns image dimensions without retaining decoded pixels.
pub fn dimensions(bytes: &[u8], limits: ImageLimits) -> Result<(u32, u32)> {
    let image = decode(bytes, limits)?;
    Ok(image.dimensions())
}

/// Decodes and resizes an image, returning an encoded payload.
pub fn resize(
    bytes: &[u8],
    width: u32,
    height: u32,
    mode: ResizeMode,
    output: OutputFormat,
    limits: ImageLimits,
) -> Result<Vec<u8>> {
    validate_target(width, height, limits)?;
    let image = decode(bytes, limits)?;
    let resized = match mode {
        ResizeMode::Fit => image.resize(width, height, FilterType::Lanczos3),
        ResizeMode::FillCrop => image.resize_to_fill(width, height, FilterType::Lanczos3),
        ResizeMode::Exact => image.resize_exact(width, height, FilterType::Lanczos3),
    };
    encode(&resized, output, limits)
}

/// Crops an image after validating the rectangle against decoded dimensions.
pub fn crop(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    output: OutputFormat,
    limits: ImageLimits,
) -> Result<Vec<u8>> {
    validate_target(width, height, limits)?;
    let image = decode(bytes, limits)?;
    let end_x = x
        .checked_add(width)
        .ok_or(ExtraError::ImageLimit("crop rectangle"))?;
    let end_y = y
        .checked_add(height)
        .ok_or(ExtraError::ImageLimit("crop rectangle"))?;
    if end_x > image.width() || end_y > image.height() {
        return Err(ExtraError::ImageLimit("crop rectangle"));
    }
    encode(&image.crop_imm(x, y, width, height), output, limits)
}

fn decode(bytes: &[u8], limits: ImageLimits) -> Result<DynamicImage> {
    if bytes.len() > limits.max_input_bytes {
        return Err(ExtraError::ImageLimit("encoded input bytes"));
    }
    let reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
    let image = reader.decode()?;
    validate_target(image.width(), image.height(), limits)?;
    Ok(image)
}

fn validate_target(width: u32, height: u32, limits: ImageLimits) -> Result<()> {
    if width == 0 || height == 0 || width > limits.max_width || height > limits.max_height {
        return Err(ExtraError::ImageLimit("dimensions"));
    }
    if u64::from(width).saturating_mul(u64::from(height)) > limits.max_pixels {
        return Err(ExtraError::ImageLimit("pixel count"));
    }
    Ok(())
}

fn encode(image: &DynamicImage, output: OutputFormat, limits: ImageLimits) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    match output {
        OutputFormat::Png => image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?,
        OutputFormat::WebP => image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::WebP)?,
        OutputFormat::Jpeg(quality) => {
            if !(1..=100).contains(&quality) {
                return Err(ExtraError::ImageLimit("JPEG quality"));
            }
            let mut encoder =
                ::image::codecs::jpeg::JpegEncoder::new_with_quality(&mut bytes, quality);
            encoder.encode_image(image)?;
        }
    }
    if bytes.len() > limits.max_output_bytes {
        return Err(ExtraError::ImageLimit("encoded output bytes"));
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::image::{ImageBuffer, Rgb};

    fn fixture() -> Vec<u8> {
        let image = DynamicImage::ImageRgb8(ImageBuffer::from_pixel(20, 10, Rgb([10, 20, 30])));
        let mut bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn resizes_and_crops_with_bounded_output() {
        let limits = ImageLimits::default();
        let resized = resize(&fixture(), 5, 5, ResizeMode::Fit, OutputFormat::Png, limits).unwrap();
        assert_eq!(dimensions(&resized, limits).unwrap(), (5, 3));
        let cropped = crop(&fixture(), 5, 2, 4, 3, OutputFormat::Jpeg(85), limits).unwrap();
        assert_eq!(dimensions(&cropped, limits).unwrap(), (4, 3));
    }

    #[test]
    fn rejects_pixel_and_crop_amplification() {
        let limits = ImageLimits {
            max_pixels: 10,
            ..ImageLimits::default()
        };
        assert!(matches!(
            dimensions(&fixture(), limits),
            Err(ExtraError::ImageLimit("pixel count"))
        ));
        assert!(matches!(
            crop(
                &fixture(),
                19,
                9,
                2,
                2,
                OutputFormat::Png,
                ImageLimits::default()
            ),
            Err(ExtraError::ImageLimit("crop rectangle"))
        ));
    }
}
