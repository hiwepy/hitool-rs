//! Excel reader facade aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelReader`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelReader.java
//!
//! `ExcelReader` 包装 Apache POI 的工作簿,提供 `read()` / `readAll()` /
//! `readAsText()` 等读取方法。所有方法按 Java 签名声明空对齐桩。

use crate::{PoiError, Result};

/// Row representation shared by `read` and `readColumn` paths.
///
/// 对齐 Java: `ExcelReader.read()` 返回的 `List<List<Object>>`
#[derive(Debug, Clone, Default)]
pub struct RowList {
    pub cells: Vec<String>,
}

/// Cell editor dispatch trait shared with `cn.hutool.poi.excel.cell.CellEditor`.
pub trait CellEditorDispatch {
    /// 对齐 Java: `CellEditor.edit(Cell cell, Object newValue)`
    fn edit(&mut self, new_value: &str) -> String;
}

/// Cell handler dispatch trait shared with `cn.hutool.poi.excel.cell.CellHandler`.
pub trait CellHandlerDispatch {
    /// 对齐 Java: `CellHandler.handle(Cell cell, Object value)`
    fn handle(&mut self, column_index: i32, row_index: i64, value: &str);
}

/// Sheet reader dispatch trait shared with `cn.hutool.poi.excel.reader.SheetReader`.
pub trait SheetReaderDispatch {
    /// 对齐 Java: `SheetReader.read(Sheet sheet, int rowIndex, int columnIndex, Cell cell)`
    fn read(&mut self, row_index: i32, column_index: i32, value: &str);
}

/// Extractor dispatch shared with Apache POI `ExcelExtractor`.
pub trait ExcelExtractorDispatch {
    /// 对齐 Java: `ExcelExtractor.getText()` / `getExtractedText()`
    fn text(&mut self) -> Result<String>;
}

/// Excel reader facade.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelReader`
#[derive(Debug, Clone, Default)]
pub struct ExcelReader {
    _private: (),
}

impl ExcelReader {
    /// 对齐 Java: `new ExcelReader(String, int)`
    pub fn new_path_index(_path: &str, _sheet_index: i32) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new ExcelReader(String, String)`
    pub fn new_path_sheet(_path: &str, _sheet_name: &str) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new ExcelReader(InputStream, int)`
    pub fn new_stream_index(_bytes: &[u8], _sheet_index: i32) -> Self {
        Self { _private: () }
    }
    /// 对齐 Java: `new ExcelReader(InputStream, String)`
    pub fn new_stream_sheet(_bytes: &[u8], _sheet_name: &str) -> Self {
        Self { _private: () }
    }

    /// 对齐 Java: `ExcelReader.isIgnoreEmptyRow()`
    pub fn is_ignore_empty_row(&self) -> bool {
        false
    }

    /// 对齐 Java: `ExcelReader.setIgnoreEmptyRow(boolean)`
    pub fn set_ignore_empty_row(mut self, _flag: bool) -> Self {
        self
    }

    /// 对齐 Java: `ExcelReader.setCellEditor(CellEditor)`
    pub fn set_cell_editor(mut self, _editor: Box<dyn CellEditorDispatch>) -> Self {
        self
    }

    /// 对齐 Java: `ExcelReader.read()`
    pub fn read(&self) -> Result<Vec<RowList>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int startRowIndex)`
    pub fn read_from(&self, _start_row_index: i32) -> Result<Vec<RowList>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int)`
    pub fn read_range(&self, _start_row: i32, _end_row: i32) -> Result<Vec<RowList>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int, boolean)`
    pub fn read_range_with_alias(
        &self,
        _start_row: i32,
        _end_row: i32,
        _alias_first_line: bool,
    ) -> Result<Vec<RowList>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readColumn(int, int)`
    pub fn read_column(&self, _column: i32, _start_row: i32) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readColumn (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readColumn(int, int, int)`
    pub fn read_column_range(
        &self,
        _column: i32,
        _start_row: i32,
        _end_row: i32,
    ) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readColumn (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(CellHandler)`
    pub fn read_with_handler(&self, _handler: &mut dyn CellHandlerDispatch) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read(CellHandler) (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int, CellHandler)`
    pub fn read_range_with_handler(
        &self,
        _start_row: i32,
        _end_row: i32,
        _handler: &mut dyn CellHandlerDispatch,
    ) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readAll()` → `List<Map<String, Object>>`
    pub fn read_all(&self) -> Result<Vec<std::collections::BTreeMap<String, String>>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readAll (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int, int)` (header row)
    pub fn read_with_header(
        &self,
        _header_row: i32,
        _start_row: i32,
        _end_row: i32,
    ) -> Result<Vec<std::collections::BTreeMap<String, String>>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readAll(Class<T>)` → Bean 列表
    pub fn read_all_as<T>(&self) -> Result<Vec<T>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readAll(Class) (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int, Class<T>)`
    pub fn read_as<T>(
        &self,
        _header_row: i32,
        _start_row: i32,
    ) -> Result<Vec<T>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read(Class) (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(int, int, int, Class<T>)`
    pub fn read_as_range<T>(
        &self,
        _header_row: i32,
        _start_row: i32,
        _end_row: i32,
    ) -> Result<Vec<T>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.read(SheetReader<T>)`
    pub fn read_with_sheet_reader<T>(
        &self,
        _reader: &mut dyn SheetReaderDispatch,
    ) -> Result<Vec<T>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::read(SheetReader) (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readAsText(boolean)`
    pub fn read_as_text(&self, _with_sheet_name: bool) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readAsText (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.getExtractor()`
    pub fn get_extractor(&self) -> Result<Box<dyn ExcelExtractorDispatch>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::getExtractor (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readRow(int)`
    pub fn read_row(&self, _row_index: i32) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readRow (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelReader.readCellValue(int, int)`
    pub fn read_cell_value(&self, _x: i32, _y: i32) -> Result<String> {
        Err(PoiError::PendingEngine(
            "ExcelReader::readCellValue (waiting for easyexcel-rs)",
        ))
    }
}