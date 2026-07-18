//! 对齐: `cn.hutool.core.text.csv.CsvData`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvData.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvData#`
#[derive(Debug, Clone)]
pub struct CsvData;

impl CsvData {
    /// 对齐 Java: `CsvData::getRows#List<CsvRow> ()`
    pub fn get_rows(&self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvData::get_rows"))
    }

    /// 对齐 Java: `CsvData::iterator#Iterator<CsvRow> ()`
    pub fn iter(&self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvData::iter"))
    }
}