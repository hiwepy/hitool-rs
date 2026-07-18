//! Bean row handler aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.handler.BeanRowHandler<T>`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/handler/BeanRowHandler.java

use crate::{PoiError, Result};

/// Bean row handler.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.handler.BeanRowHandler`
#[derive(Debug, Clone, Default)]
pub struct BeanRowHandler;
impl BeanRowHandler {
    /// 对齐 Java: `new BeanRowHandler(Class<T> beanType, boolean isWriteKeyAsHead)`
    pub fn new<T>(_bean_type: std::marker::PhantomData<T>, _is_write_key_as_head: bool) -> Self {
        Self
    }
    /// 对齐 Java: `BeanRowHandler.handle(int sheetIndex, long rowIndex, List<Object> rowList)`
    pub fn handle<T>(&mut self, _sheet_index: i32, _row_index: i64, _row: &[String]) -> Result<T> {
        Err(PoiError::PendingEngine(
            "BeanRowHandler::handle (waiting for easyexcel-rs)",
        ))
    }
}