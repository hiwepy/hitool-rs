//! Cell setter interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.CellSetter`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/CellSetter.java
//!
//! 抽象基类 `CellSetter` 封装"将值写入单元格"的策略,具体实现由
//! `cn.hutool.poi.excel.cell.setters.*` 子类提供。

/// Cell setter dispatch trait.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.CellSetter`
pub trait CellSetter {
    /// 对齐 Java: `CellSetter.setValue(Cell cell, Object value)`
    fn set_value(&mut self, value: &str);
}