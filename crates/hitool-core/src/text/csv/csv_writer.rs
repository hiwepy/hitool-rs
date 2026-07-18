//! 对齐: `cn.hutool.core.text.csv.CsvWriter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvWriter.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvWriter#`
#[derive(Debug, Clone)]
pub struct CsvWriter;

impl CsvWriter {
    /// 对齐 Java: `CsvWriter::write#void (CsvRow)`
    pub fn write(&mut self, _row: ()) -> Result<()> {
        Err(CoreError::PendingEngine("CsvWriter::write"))
    }

    /// 对齐 Java: `CsvWriter::flush#void ()`
    pub fn flush(&mut self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvWriter::flush"))
    }

    /// 对齐 Java: `CsvWriter::close#void ()`
    pub fn close(&mut self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvWriter::close"))
    }
}