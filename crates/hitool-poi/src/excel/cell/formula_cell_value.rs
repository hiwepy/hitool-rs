//! Formula cell value aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.FormulaCellValue`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/FormulaCellValue.java
//!
//! Hutool 的 `FormulaCellValue` 是 `CellValue` 接口实现,封装 POI 公式求值结果。

use crate::{PoiError, Result};

/// Formula cell value.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.FormulaCellValue`
#[derive(Debug, Clone, Default)]
pub struct FormulaCellValue {
    /// Cached formula evaluation result (string form).
    pub result: String,
}

impl FormulaCellValue {
    /// 对齐 Java: `new FormulaCellValue(String formula)`
    pub fn new_formula(_formula: &str) -> Result<Self> {
        Err(PoiError::PendingEngine(
            "FormulaCellValue::new (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `new FormulaCellValue(String formula, CellValue fallback)`
    pub fn new_with_fallback(_formula: &str, _fallback: ()) -> Result<Self> {
        Err(PoiError::PendingEngine(
            "FormulaCellValue::new (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `FormulaCellValue.getValue()`
    pub fn get_value(&self) -> &str {
        &self.result
    }
    /// 对齐 Java: `FormulaCellValue.setValue(Object)`
    pub fn set_value(&mut self, _value: &str) {}
}