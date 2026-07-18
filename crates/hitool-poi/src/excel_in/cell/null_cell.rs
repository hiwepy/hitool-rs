//! Null-cell sentinel aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.NullCell`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/NullCell.java
//!
//! `NullCell` 是 Hutool 对"空单元格"的占位实现,避免在链式 `setValue` 中
//! 出现 NPE。本文件保留类型与方法名对齐。

use crate::{PoiError, Result};

/// Null-cell sentinel.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.NullCell`
#[derive(Debug, Clone, Copy, Default)]
pub struct NullCell;

impl NullCell {
    /// 对齐 Java: `NullCell.INSTANCE`
    pub const INSTANCE: Self = Self;

    /// 对齐 Java: `NullCell.getValue()`
    pub fn get_value(&self) -> Option<String> {
        None
    }
    /// 对齐 Java: `NullCell.setValue(Object)`
    pub fn set_value(&mut self, _value: &str) {}
    /// 对齐 Java: `NullCell.getCellType()`
    pub fn get_cell_type(&self) -> i32 {
        -1
    }
    /// 对齐 Java: `NullCell.isNull()`
    pub fn is_null(&self) -> bool {
        true
    }

    /// 对齐 Java: `NullCell.setCellValue(...)` 系列便捷方法
    pub fn set_cell_value_string(_s: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellValue(boolean)`
    pub fn set_cell_value_bool(_b: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellValue(double)`
    pub fn set_cell_value_double(_d: f64) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellValue(Date)`
    pub fn set_cell_value_date(_value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setBlank()`
    pub fn set_blank() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setBlank (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellFormula(String)`
    pub fn set_cell_formula(_formula: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellFormula (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.removeFormula()`
    pub fn remove_formula() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::removeFormula (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getStringCellValue()`
    pub fn get_string_cell_value() -> Result<String> {
        Err(PoiError::PendingEngine(
            "NullCell::getStringCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getNumericCellValue()`
    pub fn get_numeric_cell_value() -> Result<f64> {
        Err(PoiError::PendingEngine(
            "NullCell::getNumericCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getBooleanCellValue()`
    pub fn get_boolean_cell_value() -> Result<bool> {
        Err(PoiError::PendingEngine(
            "NullCell::getBooleanCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getDateCellValue()`
    pub fn get_date_cell_value() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::getDateCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getCellFormula()`
    pub fn get_cell_formula() -> Result<String> {
        Err(PoiError::PendingEngine(
            "NullCell::getCellFormula (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getErrorCellValue()`
    pub fn get_error_cell_value() -> Result<i32> {
        Err(PoiError::PendingEngine(
            "NullCell::getErrorCellValue (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getErrorCellString()`
    pub fn get_error_cell_string() -> Result<String> {
        Err(PoiError::PendingEngine(
            "NullCell::getErrorCellString (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getHyperlink()`
    pub fn get_hyperlink() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::getHyperlink (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setHyperlink(Hyperlink)`
    pub fn set_hyperlink(_hyperlink: ()) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setHyperlink (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.removeHyperlink()`
    pub fn remove_hyperlink() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::removeHyperlink (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getCellComment()`
    pub fn get_cell_comment() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::getCellComment (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellComment(Comment)`
    pub fn set_cell_comment(_comment: ()) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellComment (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.removeCellComment()`
    pub fn remove_cell_comment() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::removeCellComment (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.getCellStyle()`
    pub fn get_cell_style() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::getCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.setCellStyle(CellStyle)`
    pub fn set_cell_style(_style: ()) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::setCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `NullCell.asCell()`
    pub fn as_cell() -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCell::asCell (waiting for easyexcel-rs)",
        ))
    }
}