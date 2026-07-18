//! 对齐: `cn.hutool.core.text.csv.CsvBaseReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvBaseReader.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvBaseReader#`
#[derive(Debug, Clone)]
pub struct CsvBaseReader;

impl CsvBaseReader {
    /// 对齐 Java: `CsvBaseReader::read#CsvData (Reader, Charset)` 概览桩
    pub fn read(_reader: (), _charset: &str) -> Result<()> {
        Err(CoreError::PendingEngine("CsvBaseReader::read"))
    }
}