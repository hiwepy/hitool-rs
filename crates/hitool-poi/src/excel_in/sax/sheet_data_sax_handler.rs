//! Sheet data SAX handler aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.SheetDataSaxHandler`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/SheetDataSaxHandler.java

use crate::{PoiError, Result};

/// Sheet data SAX handler.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.SheetDataSaxHandler`
#[derive(Debug, Clone, Default)]
pub struct SheetDataSaxHandler;
impl SheetDataSaxHandler {
    /// 对齐 Java: `SheetDataSaxHandler()`
    pub fn new() -> Self {
        Self
    }
    /// 对齐 Java: `SheetDataSaxHandler.startElement(...)`
    pub fn start_element(&mut self, _uri: &str, _local_name: &str, _q_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetDataSaxHandler::startElement (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetDataSaxHandler.endElement(...)`
    pub fn end_element(&mut self, _uri: &str, _local_name: &str, _q_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetDataSaxHandler::endElement (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetDataSaxHandler.characters(...)`
    pub fn characters(&mut self, _chars: &[u8]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetDataSaxHandler::characters (waiting for easyexcel-rs)",
        ))
    }
}