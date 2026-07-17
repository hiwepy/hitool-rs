//! Cell handler interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.CellHandler`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/CellHandler.java
//!
//! `CellHandler` 在 `ExcelReader.read(CellHandler)` 路径中逐单元格回调。

/// Cell handler dispatch trait.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.CellHandler`
pub trait CellHandler {
    /// 对齐 Java: `CellHandler.handle(Cell cell, Object value)`
    fn handle(&mut self, column_index: i32, row_index: i64, value: &str);
}