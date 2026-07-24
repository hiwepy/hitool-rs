//! ZIP creation and bounded, path-safe extraction.
//!
//! 对齐: `cn.hutool.extra.compress.CompressUtil`
//! 对齐: `cn.hutool.core.util.ZipUtil`（extra 侧安全 ZIP 子集）

use crate::{ExtraError, Result};
use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, Write},
    path::{Component, Path},
};
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

use super::extraction_limits::ExtractionLimits;

/// Hutool `CompressUtil` — ZIP archiver/extractor factory over safe helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.compress.CompressUtil`
///
/// 7z/tar engines remain planned; ZIP maps to [`create_zip`] / [`extract_zip`].
pub struct CompressUtil;

impl CompressUtil {
    /// Creates a ZIP byte archive (Hutool `createArchiver` ZIP path).
    pub fn create_archiver(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
        create_zip(entries)
    }

    /// Extracts a ZIP beneath `destination` (Hutool `createExtractor` ZIP path).
    pub fn create_extractor<R: Read + Seek>(
        reader: R,
        destination: impl AsRef<Path>,
        limits: ExtractionLimits,
    ) -> Result<()> {
        extract_zip(reader, destination, limits)
    }
}

pub(crate) fn extract_zip<R: Read + Seek>(

pub(crate) fn create_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    for (name, bytes) in entries {
        validate_relative_path(Path::new(name))?;
        writer.start_file(*name, options)?;
        writer.write_all(bytes)?;
    }
    Ok(writer.finish()?.into_inner())
}
