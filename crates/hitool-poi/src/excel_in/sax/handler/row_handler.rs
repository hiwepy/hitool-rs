//! Row handler interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.handler.RowHandler`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/handler/RowHandler.java
//!
//! `RowHandler` 是 SAX 路径逐行回调的根接口,Rust 形态通过 trait dispatch 表达。

use crate::{PoiError, Result};

/// Row handler interface.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.handler.RowHandler`
#[derive(Debug, Clone, Copy, Default)]
pub struct RowHandler;
impl RowHandler {
    /// 对齐 Java: `RowHandler.handle(int sheetIndex, long rowIndex, List<Object> rowList)`
    pub fn handle(&mut self, _sheet_index: i32, _row_index: i64, _row: &[String]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RowHandler::handle (waiting for easyexcel-rs)",
        ))
    }
}