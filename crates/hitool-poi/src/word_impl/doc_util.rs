//! Document creation helper aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.word.DocUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/word/DocUtil.java
//!
//! `DocUtil` 提供从已有 `.docx` 文件加载 `XWPFDocument` 的便捷方法。

use crate::{PoiError, Result};
use crate::word_impl::word07_writer::XwpfDocument;

/// Document utility facade.
///
/// 对齐 Java: `cn.hutool.poi.word.DocUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct DocUtil;

impl DocUtil {
    /// 对齐 Java: `DocUtil.create(File file)`
    pub fn create(_file: &str) -> Result<XwpfDocument> {
        Err(PoiError::PendingEngine(
            "DocUtil::create (waiting for easydoc-rs)",
        ))
    }
}