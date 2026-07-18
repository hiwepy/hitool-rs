//! Cell value implementations aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.values.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/values/

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.cell.values.ErrorCellValue`
#[derive(Debug, Clone, Default)]
pub struct ErrorCellValue {
    pub code: i32,
}
impl ErrorCellValue {
    /// 对齐 Java: `new ErrorCellValue(Cell cell)`
    pub fn new(_cell: ()) -> Self {
        Self { code: 0 }
    }
    /// 对齐 Java: `ErrorCellValue.getValue()`
    pub fn get_value(&self) -> i32 {
        self.code
    }
    /// 对齐 Java: `ErrorCellValue.getErrorString()`
    pub fn get_error_string(&self) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ErrorCellValue::getErrorString (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.values.NumericCellValue`
#[derive(Debug, Clone, Default)]
pub struct NumericCellValue {
    pub value: f64,
}
impl NumericCellValue {
    /// 对齐 Java: `new NumericCellValue(Cell cell)`
    pub fn new(_cell: ()) -> Self {
        Self { value: 0.0 }
    }
    /// 对齐 Java: `NumericCellValue.getValue()`
    pub fn get_value(&self) -> f64 {
        self.value
    }
    /// 对齐 Java: `NumericCellValue.setValue(Number)`
    pub fn set_value(&mut self, _value: f64) {}
    /// 对齐 Java: `NumericCellValue.getCellType()`
    pub fn get_cell_type(&self) -> i32 {
        0
    }
    /// 对齐 Java: `NumericCellValue.toString()`
    pub fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}