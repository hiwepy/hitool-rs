//! Excel file utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelFileUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelFileUtil.java
//!
//! 主要提供文件复制、临时目录创建与 OOXML 容器探测。

use crate::{PoiError, Result};

/// Excel file utility facade.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelFileUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelFileUtil;

impl ExcelFileUtil {
    /// 对齐 Java: `ExcelFileUtil.isXlsx(File)`
    pub fn is_xlsx(_path: &str) -> Result<bool> {
        Err(PoiError::PendingEngine(
            "ExcelFileUtil::isXlsx (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelFileUtil.isXls(File)`
    pub fn is_xls(_path: &str) -> Result<bool> {
        Err(PoiError::PendingEngine(
            "ExcelFileUtil::isXls (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelFileUtil.createTempFile(String prefix)`
    pub fn create_temp_file(_prefix: &str) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelFileUtil::createTempFile (waiting for easyexcel-rs)",
        ))
    }
}