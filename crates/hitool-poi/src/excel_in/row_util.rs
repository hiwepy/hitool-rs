//! Row utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.RowUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/RowUtil.java
//!
//! 提供 `readRow` / `writeRow` 等基于行集合的工具方法。

use crate::{PoiError, Result};

/// Row utility facade.
///
/// 对齐 Java: `cn.hutool.poi.excel.RowUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct RowUtil;

impl RowUtil {
    /// 对齐 Java: `RowUtil.readRow(Row, int)`
    pub fn read_row(_row_index: i32) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "RowUtil::readRow (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `RowUtil.writeRow(Row, Object...)`
    pub fn write_row(_values: &[&str]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowUtil::writeRow (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `RowUtil.writeRow(Row, Iterable<?>)`
    pub fn write_row_iter(_values: &[&str]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowUtil::writeRow (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `RowUtil.writeRow(Row, Map<?,?>, boolean)`
    pub fn write_row_map(_map: &[(&str, &str)], _key_as_head: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowUtil::writeRow (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `RowUtil.getOrCreateRow(Sheet, int)`
    pub fn get_or_create_row(_sheet: (), _index: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowUtil::getOrCreateRow (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `RowUtil.getOrCreateCell(Row, int)`
    pub fn get_or_create_cell(_row: (), _index: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowUtil::getOrCreateCell (waiting for easyexcel-rs)",
        ))
    }
}