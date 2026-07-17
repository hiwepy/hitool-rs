//! Excel static facade aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelUtil.java
//!
//! `ExcelUtil` 是 Hutool 对 Apache POI 工作簿/工作表的静态入口,提供
//! `getReader`、`getWriter`、`getBigWriter`、`readBySax` 等工厂方法。
//! 所有方法均在 stub 中以原签名声明,等待 easyexcel-rs 完成。

use crate::{PoiError, Result};

/// Cell location reference.
///
/// 对齐 Java: `cn.hutool.poi.excel.CellLocation`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellLocation {
    /// X coordinate (zero-based column).
    pub x: u32,
    /// Y coordinate (zero-based row).
    pub y: u32,
}

/// Static facade for Excel reader/writer factories.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelUtil;

impl ExcelUtil {
    /// MIME type for legacy `.xls` workbooks.
    pub const XLS_CONTENT_TYPE: &'static str = "application/vnd.ms-excel";
    /// MIME type for modern `.xlsx` workbooks.
    pub const XLSX_CONTENT_TYPE: &'static str =
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

    // --- readBySax family ---

    /// 对齐 Java: `ExcelUtil.readBySax(String, int, RowHandler)`
    pub fn read_by_sax_path_int(
        _path: &str,
        _rid: i32,
        _handler: &mut dyn RowHandlerDispatch,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelUtil.readBySax(String, String, RowHandler)`
    pub fn read_by_sax_path_str(
        _path: &str,
        _id_or_rid: &str,
        _handler: &mut dyn RowHandlerDispatch,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelUtil.readBySax(InputStream, int, RowHandler)`
    pub fn read_by_sax_stream_int(
        _input: &[u8],
        _rid: i32,
        _handler: &mut dyn RowHandlerDispatch,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelUtil.readBySax(InputStream, String, RowHandler)`
    pub fn read_by_sax_stream_str(
        _input: &[u8],
        _id_or_rid_or_sheet: &str,
        _handler: &mut dyn RowHandlerDispatch,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::readBySax (waiting for easyexcel-rs)",
        ))
    }

    // --- getReader family ---

    /// 对齐 Java: `ExcelUtil.getReader(String)`
    pub fn get_reader_path(_path: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getReader(File)`
    pub fn get_reader_file() -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getReader(String, int)`
    pub fn get_reader_path_sheet_index(_path: &str, _index: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getReader(String, String)`
    pub fn get_reader_path_sheet_name(_path: &str, _sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getReader (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getReader(InputStream)`
    pub fn get_reader_stream(_bytes: &[u8]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getReader (waiting for easyexcel-rs)",
        ))
    }

    // --- getWriter family ---

    /// 对齐 Java: `ExcelUtil.getWriter()`
    pub fn get_writer() -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getWriter(boolean isXlsx)`
    pub fn get_writer_xlsx(_is_xlsx: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getWriter(String)`
    pub fn get_writer_path(_path: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getWriterWithSheet(String)`
    pub fn get_writer_with_sheet(_sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getWriterWithSheet (waiting for easyexcel-rs)",
        ))
    }

    // --- BigWriter family ---

    /// 对齐 Java: `ExcelUtil.getBigWriter()`
    pub fn get_big_writer() -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getBigWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getBigWriter(int)`
    pub fn get_big_writer_window(_row_access_window_size: i32) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getBigWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelUtil.getBigWriter(String, String)`
    pub fn get_big_writer_path_sheet(_path: &str, _sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::getBigWriter (waiting for easyexcel-rs)",
        ))
    }

    // --- helpers ---

    /// 对齐 Java: `ExcelUtil.indexToColName(int)`
    pub fn index_to_col_name(index: i32) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelUtil::indexToColName (waiting for easyexcel-rs)",
        ))
        .map(|_: String| {
            let _ = index;
            String::new()
        })
    }

    /// 对齐 Java: `ExcelUtil.colNameToIndex(String)`
    pub fn col_name_to_index(name: &str) -> Result<i32> {
        let _ = name;
        Err(PoiError::PendingEngine(
            "ExcelUtil::colNameToIndex (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelUtil.toLocation(String)`
    pub fn to_location(location_ref: &str) -> Result<CellLocation> {
        let _ = location_ref;
        Err(PoiError::PendingEngine(
            "ExcelUtil::toLocation (waiting for easyexcel-rs)",
        ))
    }
}

/// Row handler dispatch trait shared with `cn.hutool.poi.excel.sax.RowHandler`.
///
/// Hutool 在 SAX 路径中通过 `RowHandler.handle(...)` 逐行回调;Rust 桩以
/// `dyn` trait 表达相同语义,等待 easyexcel-rs 接入。
pub trait RowHandlerDispatch {
    /// 对齐 Java: `RowHandler.handle(int sheetIndex, long rowIndex, RowList)`
    fn handle(&mut self, sheet_index: i32, row_index: i64, row: &[String]);
}