//! Excel sub-facade aligned with Hutool's `cn.hutool.poi.excel` package.
//!
//! 对齐: `cn.hutool.poi.excel.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/
//!
//! 模块重导出 Hutool 同名 Java 类的对齐桩:
//! - [`excel_util`]       → `cn.hutool.poi.excel.ExcelUtil`
//! - [`excel_reader`]     → `cn.hutool.poi.excel.ExcelReader`
//! - [`excel_writer`]     → `cn.hutool.poi.excel.ExcelWriter`
//! - [`big_excel_writer`] → `cn.hutool.poi.excel.BigExcelWriter`
//! - [`excel_base`]       → `cn.hutool.poi.excel.ExcelBase`
//! - [`excel_file_util`]  → `cn.hutool.poi.excel.ExcelFileUtil`
//! - [`excel_extractor_util`] → `cn.hutool.poi.excel.ExcelExtractorUtil`
//! - [`excel_pic_util`]   → `cn.hutool.poi.excel.ExcelPicUtil`
//! - [`excel_date_util`]  → `cn.hutool.poi.excel.ExcelDateUtil`
//! - [`row_util`]         → `cn.hutool.poi.excel.RowUtil`
//! - [`workbook_util`]    → `cn.hutool.poi.excel.WorkbookUtil`
//! - [`style_set`]        → `cn.hutool.poi.excel.StyleSet`
//! - [`cell`]             → `cn.hutool.poi.excel.cell.*`
//! - [`sax`]              → `cn.hutool.poi.excel.sax.*`
//! - [`reader`]           → `cn.hutool.poi.excel.reader.*`
//! - [`style`]            → `cn.hutool.poi.excel.style.*`

pub mod big_excel_writer;
pub mod cell;
pub mod excel_base;
pub mod excel_date_util;
pub mod excel_extractor_util;
pub mod excel_file_util;
pub mod excel_pic_util;
pub mod excel_reader;
pub mod excel_util;
pub mod excel_writer;
pub mod reader;
pub mod row_util;
pub mod sax;
pub mod style;
pub mod style_set;
pub mod workbook_util;