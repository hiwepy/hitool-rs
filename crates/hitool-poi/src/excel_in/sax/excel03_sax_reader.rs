//! Excel 97-2003 SAX reader aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.Excel03SaxReader`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/Excel03SaxReader.java
//!
//! `Excel03SaxReader` 通过 POI 的 `HSSFListener` 模型处理 .xls。

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.sax.Excel03SaxReader`
#[derive(Debug, Clone, Default)]
pub struct Excel03SaxReader;
impl Excel03SaxReader {
    /// 对齐 Java: `new Excel03SaxReader(RowHandler rowHandler)`
    pub fn new(_handler: Box<dyn super::handler::RowHandlerDispatch>) -> Self {
        Self
    }
    /// 对齐 Java: `Excel03SaxReader.read(String path, int sheetIndex)`
    pub fn read_path_index(_path: &str, _sheet_index: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "Excel03SaxReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `Excel03SaxReader.read(String path, String sheetName)`
    pub fn read_path_sheet(_path: &str, _sheet_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "Excel03SaxReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `Excel03SaxReader.read(InputStream inputStream, int sheetIndex)`
    pub fn read_stream_index(_bytes: &[u8], _sheet_index: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "Excel03SaxReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `Excel03SaxReader.read(InputStream inputStream, String sheetName)`
    pub fn read_stream_sheet(_bytes: &[u8], _sheet_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "Excel03SaxReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `Excel03SaxReader.process()`
    pub fn process(&mut self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "Excel03SaxReader::process (waiting for easyexcel-rs)",
        ))
    }
}