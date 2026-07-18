//! 对齐: `cn.hutool.core.text.csv.CsvReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvReader.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvReader#`
#[derive(Debug, Clone)]
pub struct CsvReader;

impl CsvReader {
    /// 对齐 Java: `CsvReader::read#CsvData (Resource, Charset)`
    pub fn read(_resource: (), _charset: &str) -> Result<()> {
        Err(CoreError::PendingEngine("CsvReader::read"))
    }

    /// 对齐 Java: `CsvReader::iterator#Iterator<CsvRow> ()`
    pub fn iter(&self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvReader::iter"))
    }

    /// 对齐 Java: `CsvReader::close#void ()`
    pub fn close(&mut self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvReader::close"))
    }
}