//! Sheet relationship-id reader aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.SheetRidReader`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/SheetRidReader.java
//!
//! `SheetRidReader` 在读取 `xl/_rels/workbook.xml.rels` 时按关系 ID
//! 提取工作表名与目标路径。

use crate::{PoiError, Result};

/// Sheet relationship-id reader.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.SheetRidReader`
#[derive(Debug, Clone, Default)]
pub struct SheetRidReader;
impl SheetRidReader {
    /// 对齐 Java: `new SheetRidReader()`
    pub fn new() -> Self {
        Self
    }
    /// 对齐 Java: `SheetRidReader.parseRid(InputStream inputStream)`
    pub fn parse_rid_stream(_bytes: &[u8]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::parseRid (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.parseRid(File bookFile)`
    pub fn parse_rid_file(_path: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::parseRid (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.startElement(...)`
    pub fn start_element(&mut self, _uri: &str, _local_name: &str, _q_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::startElement (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.endElement(...)`
    pub fn end_element(&mut self, _uri: &str, _local_name: &str, _q_name: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::endElement (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.characters(...)`
    pub fn characters(&mut self, _chars: &[u8]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::characters (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.getRidByName(String sheetName)`
    pub fn get_rid_by_name(&self, _sheet_name: &str) -> Result<String> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::getRidByName (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `SheetRidReader.getNameByRid(String rid)`
    pub fn get_name_by_rid(&self, _rid: &str) -> Result<String> {
        Err(PoiError::PendingEngine(
            "SheetRidReader::getNameByRid (waiting for easyexcel-rs)",
        ))
    }
}