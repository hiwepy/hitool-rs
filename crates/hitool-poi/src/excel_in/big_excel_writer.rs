//! Big Excel writer facade aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.BigExcelWriter`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/BigExcelWriter.java
//!
//! `BigExcelWriter` 继承自 `ExcelWriter`,基于 `SXSSFWorkbook` 流式写入
//! 适合海量行场景。本文件以对齐桩方式声明,实现等待 easyexcel-rs。

use crate::{PoiError, Result};

/// Big Excel writer facade.
///
/// 对齐 Java: `cn.hutool.poi.excel.BigExcelWriter extends ExcelWriter`
#[derive(Debug, Clone, Default)]
pub struct BigExcelWriter {
    _private: (),
}

impl BigExcelWriter {
    /// 对齐 Java: `new BigExcelWriter()`
    pub fn new() -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new BigExcelWriter(int rowAccessWindowSize)`
    pub fn with_window(_row_access_window_size: i32) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new BigExcelWriter(String)`
    pub fn new_path(_path: &str) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new BigExcelWriter(String, String)`
    pub fn new_path_sheet(_path: &str, _sheet: &str) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `BigExcelWriter.flush()`
    pub fn flush(&self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "BigExcelWriter::flush (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `BigExcelWriter.close()`
    pub fn close(self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "BigExcelWriter::close (waiting for easyexcel-rs)",
        ))
    }
}