//! Excel text extractor aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelExtractorUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelExtractorUtil.java
//!
//! 提供 `readAsText(File)` / `readAsText(InputStream)` 等高层文本抽取便捷方法,
//! 内部封装 Apache POI 的 `ExcelExtractor`。

use crate::{PoiError, Result};

/// Excel text extractor utility.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelExtractorUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelExtractorUtil;

impl ExcelExtractorUtil {
    /// 对齐 Java: `ExcelExtractorUtil.readAsText(File)`
    pub fn read_as_text_path(_path: &str) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelExtractorUtil::readAsText (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelExtractorUtil.readAsText(InputStream)`
    pub fn read_as_text_stream(_bytes: &[u8]) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelExtractorUtil::readAsText (waiting for easyexcel-rs)",
        ))
    }
}