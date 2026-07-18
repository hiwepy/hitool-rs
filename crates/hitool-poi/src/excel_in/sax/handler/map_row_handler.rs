//! Map row handler aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.handler.MapRowHandler`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/handler/MapRowHandler.java

use crate::{PoiError, Result};
use std::collections::BTreeMap;

/// Map row handler.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.handler.MapRowHandler`
#[derive(Debug, Clone, Default)]
pub struct MapRowHandler;
impl MapRowHandler {
    /// 对齐 Java: `new MapRowHandler(boolean isWriteKeyAsHead)`
    pub fn new(_is_write_key_as_head: bool) -> Self {
        Self
    }
    /// 对齐 Java: `MapRowHandler.handle(int sheetIndex, long rowIndex, List<Object> rowList)`
    pub fn handle(&mut self, _sheet_index: i32, _row_index: i64, _row: &[String]) -> Result<BTreeMap<String, String>> {
        Err(PoiError::PendingEngine(
            "MapRowHandler::handle (waiting for easyexcel-rs)",
        ))
    }
}