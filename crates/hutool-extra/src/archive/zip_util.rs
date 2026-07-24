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

/// Hutool-aligned ZIP helpers mirroring core `ZipUtil` names for extra users.
///
/// 对齐 Java 类: `cn.hutool.core.util.ZipUtil`（内存条目 / 安全解压子集）
pub struct ZipUtil;

impl ZipUtil {
    /// Zips in-memory named entries (Hutool `zip(paths, streams)` subset).
    pub fn zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
        create_zip(entries)
    }

    /// Unzips bytes into a directory with default limits (Hutool `unzip`).
    pub fn unzip(bytes: &[u8], destination: impl AsRef<Path>) -> Result<()> {
        extract_zip(Cursor::new(bytes), destination, ExtractionLimits::default())
    }

    /// Unzips with explicit resource limits.
    pub fn unzip_with_limits(
        bytes: &[u8],
        destination: impl AsRef<Path>,
        limits: ExtractionLimits,
    ) -> Result<()> {
        extract_zip(Cursor::new(bytes), destination, limits)
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
