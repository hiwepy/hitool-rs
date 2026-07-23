//! 对齐: `cn.hutool.core.text.csv.CsvUtil`

use super::csv_data::CsvData;
use super::csv_read_config::CsvReadConfig;
use super::csv_reader::CsvReader;
use super::csv_write_config::CsvWriteConfig;
use super::csv_writer::CsvWriter;
use crate::Result;
use std::path::Path;

/// 对齐 Java: `CsvUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct CsvUtil;

impl CsvUtil {
    /// 对齐 Java: `CsvUtil.getReader()`
    pub fn get_reader() -> CsvReader {
        CsvReader::new()
    }

    /// 对齐 Java: `CsvUtil.getReader(CsvReadConfig)`
    pub fn get_reader_with(config: CsvReadConfig) -> CsvReader {
        CsvReader::with_config(config)
    }

    /// 对齐 Java: `CsvUtil.getWriter(File, Charset)`
    pub fn get_writer(path: impl AsRef<Path>) -> CsvWriter {
        CsvWriter::for_path(path, false)
    }

    /// 对齐 Java: `CsvUtil.getWriter(File, Charset, boolean append)`
    pub fn get_writer_append(path: impl AsRef<Path>, append: bool) -> CsvWriter {
        CsvWriter::for_path(path, append)
    }

    /// 便捷：读字符串
    pub fn read_from_str(text: &str) -> Result<CsvData> {
        Self::get_reader().read_from_str(text)
    }

    /// 便捷：读文件
    pub fn read_file(path: impl AsRef<Path>) -> Result<CsvData> {
        Self::get_reader().read_file(path)
    }

    /// 便捷写配置
    pub fn writer_with(path: impl AsRef<Path>, config: CsvWriteConfig) -> CsvWriter {
        CsvWriter::for_path(path, false).with_config(config)
    }
}
