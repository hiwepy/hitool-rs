//! Cell value interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.CellValue`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/CellValue.java
//!
//! `CellValue` 是 Hutool 用于在公式求值等场景包装单元格内容的接口。

/// Cell value dispatch trait.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.CellValue`
pub trait CellValue {
    /// 对齐 Java: `CellValue.getValue()`
    fn get_value(&self) -> String;
    /// 对齐 Java: `CellValue.setValue(Object)`
    fn set_value(&mut self, value: &str);
    /// 对齐 Java: `CellValue.getCellType()`
    fn get_cell_type(&self) -> i32;
}