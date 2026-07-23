//! 对齐: `cn.hutool.core.io.file.FileWriter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileWriter.java
//!
//! 文件写入门面；委托 [`crate::FileUtil`] / [`crate::IoUtil`]。

use crate::{FileUtil, IoUtil};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// 对齐 Java 类: `cn.hutool.core.io.file.FileWriter`
#[derive(Debug, Clone)]
pub struct FileWriter {
    path: PathBuf,
}

impl FileWriter {
    /// 对齐 Java: `FileWriter.create(File)` / `create(String)`
    pub fn create(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// 对齐 Java: `FileWriter(File)` 等构造。
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self::create(path)
    }

    /// 对齐 Java: `FileWriter.write(String)` / `write(String, boolean)`
    pub fn write_str(&self, content: &str) -> io::Result<()> {
        FileUtil::write_utf8_string(&self.path.to_string_lossy(), content)
    }

    /// 对齐 Java: `FileWriter.append(String)`
    pub fn append_str(&self, content: &str) -> io::Result<()> {
        FileUtil::append_utf8_string(&self.path.to_string_lossy(), content)
    }

    /// 对齐 Java: `FileWriter.writeLines(Collection)` / `appendLines`
    pub fn write_lines(&self, lines: &[impl AsRef<str>]) -> io::Result<()> {
        FileUtil::write_utf8_lines(&self.path.to_string_lossy(), lines)
    }

    /// 对齐 Java: `FileWriter.appendLines`
    pub fn append_lines(&self, lines: &[impl AsRef<str>]) -> io::Result<()> {
        FileUtil::append_utf8_lines(&self.path.to_string_lossy(), lines)
    }

    /// 对齐 Java: `FileWriter.writeMap(Map, String, boolean)` — `k=v` 行。
    pub fn write_map(&self, entries: &[(String, String)], kv_separator: &str) -> io::Result<()> {
        let lines: Vec<String> = entries
            .iter()
            .map(|(k, v)| format!("{k}{kv_separator}{v}"))
            .collect();
        self.write_lines(&lines)
    }

    /// 对齐 Java: `FileWriter.write(byte[])` / `append(byte[])`
    pub fn write_bytes(&self, data: &[u8]) -> io::Result<()> {
        FileUtil::write_bytes(&self.path.to_string_lossy(), data)
    }

    /// 对齐 Java: `FileWriter.append(byte[])`
    pub fn append_bytes(&self, data: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(data)
    }

    /// 对齐 Java: `FileWriter.writeFromStream(InputStream)`
    pub fn write_from_stream<R: Read>(&self, reader: &mut R) -> io::Result<u64> {
        FileUtil::mk_parent_dirs(&self.path.to_string_lossy())?;
        let mut file = File::create(&self.path)?;
        IoUtil::copy(reader, &mut file)
    }

    /// 对齐 Java: `FileWriter.getOutputStream()`
    pub fn get_output_stream(&self) -> io::Result<File> {
        FileUtil::mk_parent_dirs(&self.path.to_string_lossy())?;
        File::create(&self.path)
    }

    /// 对齐 Java: `FileWriter.getWriter()` — BufWriter。
    pub fn get_writer(&self) -> io::Result<BufWriter<File>> {
        Ok(BufWriter::new(self.get_output_stream()?))
    }

    /// 对齐 Java: `FileWriter.getPrintWriter()` — 同 getWriter（Rust 无 PrintWriter）。
    pub fn get_print_writer(&self) -> io::Result<BufWriter<File>> {
        self.get_writer()
    }

    /// 底层路径。
    pub fn path(&self) -> &Path {
        &self.path
    }
}
