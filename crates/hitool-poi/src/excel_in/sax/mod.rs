//! SAX sub-facade aligned with Hutool's `cn.hutool.poi.excel.sax.*` package.
//!
//! 模块重导出 Hutool 同名 Java 类的对齐桩:
//! - [`excel_sax_reader`]      → `cn.hutool.poi.excel.sax.ExcelSaxReader`
//! - [`excel07_sax_reader`]    → `cn.hutool.poi.excel.sax.Excel07SaxReader`
//! - [`excel03_sax_reader`]    → `cn.hutool.poi.excel.sax.Excel03SaxReader`
//! - [`excel_sax_util`]        → `cn.hutool.poi.excel.sax.ExcelSaxUtil`
//! - [`cell_data_type`]        → `cn.hutool.poi.excel.sax.CellDataType`
//! - [`attribute_name`]        → `cn.hutool.poi.excel.sax.AttributeName`
//! - [`element_name`]          → `cn.hutool.poi.excel.sax.ElementName`
//! - [`sheet_data_sax_handler`]→ `cn.hutool.poi.excel.sax.SheetDataSaxHandler`
//! - [`sheet_rid_reader`]      → `cn.hutool.poi.excel.sax.SheetRidReader`
//! - [`handler`]               → `cn.hutool.poi.excel.sax.handler.*`

pub mod attribute_name;
pub mod cell_data_type;
pub mod element_name;
pub mod excel03_sax_reader;
pub mod excel07_sax_reader;
pub mod excel_sax_reader;
pub mod excel_sax_util;
pub mod handler;
pub mod sheet_data_sax_handler;
pub mod sheet_rid_reader;