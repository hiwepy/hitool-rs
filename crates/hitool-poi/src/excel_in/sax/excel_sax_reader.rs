//! SAX reader base interface aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.ExcelSaxReader`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/ExcelSaxReader.java
//!
//! `ExcelSaxReader` 是所有 SAX 路径读取器的抽象父类。

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.sax.ExcelSaxReader`
/// Java 形态为抽象类,Rust 通过 trait 表达相同契约。
pub trait ExcelSaxReader {
    /// 对齐 Java: `ExcelSaxReader.read(String path, String idOrRidOrSheetName, RowHandler rowHandler)`
    fn read_path(&self, _path: &str, _id_or_rid_or_sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelSaxReader.read(InputStream inputStream, String idOrRidOrSheetName, RowHandler rowHandler)`
    fn read_stream(&self, _bytes: &[u8], _id_or_rid_or_sheet: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelSaxReader::read (waiting for easyexcel-rs)",
        ))
    }
}