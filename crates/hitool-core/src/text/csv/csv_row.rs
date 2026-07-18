//! 对齐: `cn.hutool.core.text.csv.CsvRow`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvRow.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvRow#`
#[derive(Debug, Clone)]
pub struct CsvRow;

impl CsvRow {
    /// 对齐 Java: `CsvRow::getRawList#List<String> ()`
    pub fn get_raw_list(&self) -> Result<Vec<String>> {
        Err(CoreError::PendingEngine("CsvRow::get_raw_list"))
    }

    /// 对齐 Java: `CsvRow::getByName#String (String)`
    pub fn get_by_name(&self, _name: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CsvRow::get_by_name"))
    }

    /// 对齐 Java: `CsvRow::getFieldMap#Map<String,String> ()`
    pub fn get_field_map(&self) -> Result<Vec<(String, String)>> {
        Err(CoreError::PendingEngine("CsvRow::get_field_map"))
    }
}