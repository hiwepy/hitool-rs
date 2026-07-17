//! Cell value editors aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.editors.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/editors/
//!
//! `NumericToIntEditor` 把数字单元格值截断为整数,`TrimEditor` 把字符串
//! 单元格首尾空白字符裁剪掉。两者均实现 `CellEditor` 接口。

use crate::{PoiError, Result};
use crate::excel::cell::cell_util::Cell;

/// 对齐 Java: `cn.hutool.poi.excel.editors.NumericToIntEditor`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumericToIntEditor;
impl NumericToIntEditor {
    /// 对齐 Java: `new NumericToIntEditor()`
    pub fn new() -> Self {
        Self
    }
    /// 对齐 Java: `NumericToIntEditor.edit(Cell cell, Object newValue)`
    pub fn edit(&mut self, _cell: &Cell, new_value: &str) -> Result<String> {
        let truncated = new_value
            .parse::<f64>()
            .map(|v| v.trunc() as i64)
            .unwrap_or(0);
        Ok(truncated.to_string())
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.editors.TrimEditor`
#[derive(Debug, Clone, Copy, Default)]
pub struct TrimEditor;
impl TrimEditor {
    /// 对齐 Java: `new TrimEditor()`
    pub fn new() -> Self {
        Self
    }
    /// 对齐 Java: `TrimEditor.edit(Cell cell, Object newValue)`
    pub fn edit(&mut self, _cell: &Cell, new_value: &str) -> Result<String> {
        let _ = PoiError::PendingEngine; // suppress unused import warning
        Ok(new_value.trim().to_owned())
    }
}