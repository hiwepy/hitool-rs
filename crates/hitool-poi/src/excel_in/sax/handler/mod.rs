//! SAX row handler sub-facade aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.handler.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/handler/

pub mod abstract_row_handler;
pub mod bean_row_handler;
pub mod map_row_handler;
pub mod row_handler;

/// SAX row handler dispatch trait shared by all handler types.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.handler.RowHandler`
pub trait RowHandlerDispatch {
    /// 对齐 Java: `RowHandler.handle(int sheetIndex, long rowIndex, List<Object> rowList)`
    fn handle(&mut self, sheet_index: i32, row_index: i64, row: &[String]);
}