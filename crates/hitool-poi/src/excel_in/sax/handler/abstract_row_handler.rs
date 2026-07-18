//! Abstract row handler aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.handler.AbstractRowHandler<T>`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/handler/AbstractRowHandler.java

use crate::{PoiError, Result};

/// Abstract row handler.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.handler.AbstractRowHandler`
#[derive(Debug, Clone, Default)]
pub struct AbstractRowHandler;
impl AbstractRowHandler {
    /// 对齐 Java: `new AbstractRowHandler()`
    pub fn new() -> Self {
        Self
    }
    /// 对齐 Java: `AbstractRowHandler.handle(int sheetIndex, long rowIndex, List<Object> rowList)`
    pub fn handle(&mut self, _sheet_index: i32, _row_index: i64, _row: &[String]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "AbstractRowHandler::handle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `AbstractRowHandler.convertRow(...)`
    pub fn convert_row(&self, _row: &[String]) -> Result<String> {
        Err(PoiError::PendingEngine(
            "AbstractRowHandler::convertRow (waiting for easyexcel-rs)",
        ))
    }
}