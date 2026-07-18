//! 对齐: `cn.hutool.core.io.FileUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FileUtil.java
//!
//! Rust 版本提供文件操作的 idiomatic 实现。

use std::fs;
use std::path::{Path, PathBuf};

/// 对齐 Java: `cn.hutool.core.io.FileUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct FileUtil;

impl FileUtil {
    // ── 路径操作 ──

    /// 对齐 Java: `FileUtil.getName(File)`
    pub fn name(path: &Path) -> &str {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.getSuffix(File)`
    pub fn suffix(path: &Path) -> &str {
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.mainName(File)`
    pub fn main_name(path: &Path) -> &str {
        path.file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.getName(String)`
    pub fn name_from_str(path: &str) -> &str {
        Self::name(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.getSuffix(String)`
    pub fn suffix_from_str(path: &str) -> &str {
        Self::suffix(Path::new(path))
    }

    // ── 路径构建 ──

    /// 对齐 Java: `FileUtil.getPath(String...)`
    pub fn join_paths(parts: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(part);
        }
        path
    }

    /// 对齐 Java: `FileUtil.file(String...)`
    pub fn file(parts: &[&str]) -> PathBuf {
        Self::join_paths(parts)
    }

    // ── 文件判断 ──

    /// 对齐 Java: `FileUtil.exist(String)`
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// 对齐 Java: `FileUtil.isFile(String)`
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// 对齐 Java: `FileUtil.isDirectory(String)`
    pub fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    // ── 文件大小 ──

    /// 对齐 Java: `FileUtil.size(File)`
    pub fn size(path: &Path) -> u64 {
        fs::metadata(path).map(|m| m.len()).unwrap_or(0)
    }

    // ── 文件读取 ──

    /// 对齐 Java: `FileUtil.readUtf8String(File)`
    pub fn read_utf8_string(path: &str) -> std::io::Result<String> {
        fs::read_to_string(path)
    }

    /// 对齐 Java: `FileUtil.readBytes(File)`
    pub fn read_bytes(path: &str) -> std::io::Result<Vec<u8>> {
        fs::read(path)
    }

    // ── 文件写入 ──

    /// 对齐 Java: `FileUtil.writeUtf8String(String, File)`
    pub fn write_utf8_string(path: &str, content: &str) -> std::io::Result<()> {
        fs::write(path, content)
    }

    /// 对齐 Java: `FileUtil.writeBytes(byte[], File)`
    pub fn write_bytes(path: &str, content: &[u8]) -> std::io::Result<()> {
        fs::write(path, content)
    }

    // ── 文件操作 ──

    /// 对齐 Java: `FileUtil.copy(File, File)`
    pub fn copy(from: &str, to: &str) -> std::io::Result<u64> {
        fs::copy(from, to)
    }

    /// 对齐 Java: `FileUtil.del(File)`
    pub fn delete(path: &str) -> std::io::Result<()> {
        let p = Path::new(path);
        if p.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }

    /// 对齐 Java: `FileUtil.mkdir(File)`
    pub fn mkdir(path: &str) -> std::io::Result<()> {
        fs::create_dir_all(path)
    }

    /// 对齐 Java: `FileUtil.rename(File, String)`
    pub fn rename(from: &str, to: &str) -> std::io::Result<()> {
        fs::rename(from, to)
    }

    // ── 文件列表 ──

    /// 对齐 Java: `FileUtil.listFileNames(String)`
    pub fn list_file_names(path: &str) -> std::io::Result<Vec<String>> {
        let entries = fs::read_dir(path)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
        }
        Ok(names)
    }

    /// 列出目录下的所有目录名
    pub fn list_dir_names(path: &str) -> std::io::Result<Vec<String>> {
        let entries = fs::read_dir(path)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
        }
        Ok(names)
    }

    // ── 临时文件 ──

    /// 对齐 Java: `FileUtil.getTmpDirPath()`
    pub fn tmp_dir() -> &'static str {
        "/tmp"
    }
}
