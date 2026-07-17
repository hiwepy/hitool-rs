//! OFD (Open Fixed-layout Document) writer placeholder aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.ofd.OfdWriter`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/ofd/OfdWriter.java
//!
//! OFD 是国家版式文档标准。Hutool 通过 `OfdWriter` 提供写入能力,
//! Rust 版本与 `hitool-poi` 的 `pending` 策略一致:仅保留对象/方法/参数对齐,
//! 实现留待后续 `easyofd-rs` 完成。

use crate::{PoiError, Result};

/// OFD writer facade.
///
/// 对齐 Java: `cn.hutool.poi.ofd.OfdWriter`
#[derive(Debug, Clone, Default)]
pub struct OfdWriter;

impl OfdWriter {
    /// Creates a new OFD writer targeting the supplied destination path.
    ///
    /// 对齐 Java: `new OfdWriter(File destFile)`
    pub fn new(_dest_file: impl AsRef<std::path::Path>) -> Self {
        Self
    }

    /// Appends a text run to the current page.
    ///
    /// 对齐 Java: `OfdWriter.addText(Page page, Font font, String text)`
    pub fn add_text(&mut self, _text: impl AsRef<str>) -> Result<()> {
        Err(PoiError::PendingEngine("OfdWriter::add_text (waiting for easyofd-rs)"))
    }

    /// Flushes the in-memory OFD package to its destination.
    ///
    /// 对齐 Java: `OfdWriter.flush()`
    pub fn flush(&mut self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "OfdWriter::flush (waiting for easyofd-rs)",
        ))
    }

    /// Closes the underlying writer, flushing any pending bytes.
    ///
    /// 对齐 Java: `OfdWriter.close()`
    pub fn close(&mut self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "OfdWriter::close (waiting for easyofd-rs)",
        ))
    }
}