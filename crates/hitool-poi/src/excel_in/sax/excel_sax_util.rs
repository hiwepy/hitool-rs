//! SAX reader convenience utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.ExcelSaxUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/ExcelSaxUtil.java
//!
//! `ExcelSaxUtil` 是 SAX 路径的静态工厂。

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.sax.ExcelSaxUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelSaxUtil;

impl ExcelSaxUtil {
    /// 对齐 Java: `ExcelSaxUtil.readBySax(String path, int rid, RowHandler rowHandler)`
    pub fn read_by_sax_path_int(
        _path: &str,
        _rid: i32,
        _handler: Box<dyn super::handler::RowHandlerDispatch>,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.readBySax(String path, String idOrRidOrSheetName, RowHandler rowHandler)`
    pub fn read_by_sax_path_str(
        _path: &str,
        _id_or_rid_or_sheet: &str,
        _handler: Box<dyn super::handler::RowHandlerDispatch>,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.readBySax(InputStream in, int rid, RowHandler rowHandler)`
    pub fn read_by_sax_stream_int(
        _bytes: &[u8],
        _rid: i32,
        _handler: Box<dyn super::handler::RowHandlerDispatch>,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.readBySax(InputStream in, String idOrRidOrSheetName, RowHandler rowHandler)`
    pub fn read_by_sax_stream_str(
        _bytes: &[u8],
        _id_or_rid_or_sheet: &str,
        _handler: Box<dyn super::handler::RowHandlerDispatch>,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.getReader(String path, int rid)`
    pub fn get_reader_path_int(_path: &str, _rid: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.getReader(String path, String idOrRidOrSheetName)`
    pub fn get_reader_path_str(_path: &str, _id_or_rid_or_sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.getReader(InputStream inputStream, int rid)`
    pub fn get_reader_stream_int(_bytes: &[u8], _rid: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.getReader(InputStream inputStream, String idOrRidOrSheetName)`
    pub fn get_reader_stream_str(_bytes: &[u8], _id_or_rid_or_sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxUtil.createSaxReader(String path, boolean isXlsx, RowHandler rowHandler)`
    pub fn create_sax_reader(
        _path: &str,
        _is_xlsx: bool,
        _handler: Box<dyn super::handler::RowHandlerDispatch>,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxUtil::createSaxReader (waiting for easyexcel-rs)",
        ))
    }
}