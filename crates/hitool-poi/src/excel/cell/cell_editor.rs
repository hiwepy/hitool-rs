//! Cell editor interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.CellEditor`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/CellEditor.java
//!
//! Hutool 的 `CellEditor` 是在 `CellUtil.setCellValue` 时对值做回调修改的接口。

/// Cell editor dispatch trait.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.CellEditor`
/// Java 形态为 interface;Rust 用 trait 表达相同契约。
pub trait CellEditor {
    /// 对齐 Java: `CellEditor.edit(Cell cell, Object newValue)`
    fn edit(&mut self, new_value: &str) -> String;
}