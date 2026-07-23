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

/// Limits applied before and during archive extraction.
#[derive(Debug, Clone, Copy)]
pub struct ExtractionLimits {
    /// Maximum entry count.
    pub max_entries: usize,
    /// Maximum sum of declared uncompressed bytes.
    pub max_uncompressed_bytes: u64,
}

impl Default for ExtractionLimits {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
            max_uncompressed_bytes: 1_073_741_824,
        }
    }
}

/// Creates a ZIP archive from `(relative name, bytes)` entries.
pub fn create_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
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

/// Extracts a ZIP archive beneath `destination` without following archive
/// paths outside that root or creating symbolic links.
pub fn extract_zip<R: Read + Seek>(
    reader: R,
    destination: impl AsRef<Path>,
    limits: ExtractionLimits,
) -> Result<()> {
    let mut archive = ZipArchive::new(reader)?;
    if archive.len() > limits.max_entries {
        return Err(ExtraError::ArchiveLimit("entry count"));
    }

    fs::create_dir_all(destination.as_ref())?;
    let root = destination.as_ref().canonicalize()?;
    let mut declared_total = 0_u64;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry.name().to_owned();
        let relative = entry
            .enclosed_name()
            .ok_or_else(|| ExtraError::UnsafeArchivePath(name.clone()))?;
        validate_relative_path(&relative)?;
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170_000 == 0o120_000)
        {
            return Err(ExtraError::SymlinkEntry(name));
        }

        declared_total = declared_total
            .checked_add(entry.size())
            .ok_or(ExtraError::ArchiveLimit("uncompressed byte count"))?;
        if declared_total > limits.max_uncompressed_bytes {
            return Err(ExtraError::ArchiveLimit("uncompressed byte count"));
        }

        let output_path = root.join(&relative);
        if entry.is_dir() {
            fs::create_dir_all(&output_path)?;
            ensure_beneath_root(&root, &output_path)?;
            continue;
        }
        let parent = output_path
            .parent()
            .ok_or_else(|| ExtraError::UnsafeArchivePath(name.clone()))?;
        fs::create_dir_all(parent)?;
        ensure_beneath_root(&root, parent)?;
        let mut output = File::create(&output_path)?;
        let copied = std::io::copy(&mut entry, &mut output)?;
        if copied > limits.max_uncompressed_bytes {
            return Err(ExtraError::ArchiveLimit("single entry byte count"));
        }
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(ExtraError::UnsafeArchivePath(
            path.to_string_lossy().into_owned(),
        ));
    }
    Ok(())
}

fn ensure_beneath_root(root: &Path, path: &Path) -> Result<()> {
    let canonical = path.canonicalize()?;
    if !canonical.starts_with(root) {
        return Err(ExtraError::UnsafeArchivePath(
            path.to_string_lossy().into_owned(),
        ));
    }
    Ok(())
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_extracts_bounded_archive() {
        let bytes = create_zip(&[("nested/a.txt", b"hello"), ("b.txt", b"world")]).unwrap();
        let directory = tempfile::tempdir().unwrap();
        extract_zip(
            Cursor::new(bytes),
            directory.path(),
            ExtractionLimits::default(),
        )
        .unwrap();
        assert_eq!(
            fs::read(directory.path().join("nested/a.txt")).unwrap(),
            b"hello"
        );
        assert_eq!(fs::read(directory.path().join("b.txt")).unwrap(), b"world");
    }

    #[test]
    fn rejects_parent_directory_entries_and_limits() {
        assert!(matches!(
            create_zip(&[("../escape.txt", b"bad")]),
            Err(ExtraError::UnsafeArchivePath(_))
        ));
        let bytes = create_zip(&[("large.txt", b"12345")]).unwrap();
        let directory = tempfile::tempdir().unwrap();
        assert!(matches!(
            extract_zip(
                Cursor::new(bytes),
                directory.path(),
                ExtractionLimits {
                    max_entries: 1,
                    max_uncompressed_bytes: 4
                }
            ),
            Err(ExtraError::ArchiveLimit(_))
        ));
    }

    #[test]
    fn compress_and_zip_util_roundtrip() {
        let bytes = CompressUtil::create_archiver(&[("facade.txt", b"ok")]).unwrap();
        let directory = tempfile::tempdir().unwrap();
        ZipUtil::unzip(&bytes, directory.path()).unwrap();
        assert_eq!(
            fs::read(directory.path().join("facade.txt")).unwrap(),
            b"ok"
        );
    }
}
