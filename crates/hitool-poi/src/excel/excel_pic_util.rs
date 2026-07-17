//! Excel picture utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelPicUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelPicUtil.java
//!
//! 提供 `getPictures(File)` 等从工作簿抽取图片字节的方法。

use crate::{PoiError, Result};

/// Picture bytes extracted from an Excel workbook.
#[derive(Debug, Clone, Default)]
pub struct ExcelPicture {
    /// Raw picture bytes.
    pub bytes: Vec<u8>,
    /// Suggested extension (e.g. `png`, `jpeg`).
    pub extension: String,
}

/// Excel picture utility.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelPicUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelPicUtil;

impl ExcelPicUtil {
    /// 对齐 Java: `ExcelPicUtil.getPictures(File)`
    pub fn get_pictures(_path: &str) -> Result<Vec<ExcelPicture>> {
        Err(PoiError::PendingEngine(
            "ExcelPicUtil::getPictures (waiting for easyexcel-rs)",
        ))
    }
}