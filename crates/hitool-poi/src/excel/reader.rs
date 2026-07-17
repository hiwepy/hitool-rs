//! Sheet-reader strategies aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.reader.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/reader/
//!
//! `cn.hutool.poi.excel.reader` 子包提供 6 个 `SheetReader` 实现,按行
//! 读取策略不同(Bean、Map、Column、List 等)。

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.reader.SheetReader<T>`
/// Java interface → Rust trait dispatch.
pub trait SheetReaderDispatch {
    /// 对齐 Java: `SheetReader.read(Sheet sheet, int rowIndex, int columnIndex, Cell cell)`
    fn read(&mut self, row_index: i32, column_index: i32, value: &str);
}

/// 对齐 Java: `cn.hutool.poi.excel.reader.AbstractSheetReader<T>`
#[derive(Debug, Clone, Default)]
pub struct AbstractSheetReader;
impl AbstractSheetReader {
    /// 对齐 Java: `AbstractSheetReader.read(Sheet sheet, int startRowIndex)`
    pub fn read(&self, _start_row_index: i32) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "AbstractSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `AbstractSheetReader.read(Sheet sheet, int startRowIndex, int endRowIndex)`
    pub fn read_range(&self, _start_row_index: i32, _end_row_index: i32) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "AbstractSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `AbstractSheetReader.read(Sheet sheet, int, int, boolean)`
    pub fn read_range_with_alias(
        &self,
        _start_row_index: i32,
        _end_row_index: i32,
        _alias_first_line: bool,
    ) -> Result<Vec<String>> {
        Err(PoiError::PendingEngine(
            "AbstractSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.reader.BeanSheetReader<T>`
#[derive(Debug, Clone, Default)]
pub struct BeanSheetReader;
impl BeanSheetReader {
    /// 对齐 Java: `new BeanSheetReader(Class<T> beanType)`
    pub fn new<T>(_bean_type: std::marker::PhantomData<T>) -> Self {
        Self
    }
    /// 对齐 Java: `BeanSheetReader.read(Sheet sheet, int, int, Cell cell)`
    pub fn read<T>(&mut self, _row_index: i32, _column_index: i32, _value: &str) -> Result<T> {
        Err(PoiError::PendingEngine(
            "BeanSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.reader.MapSheetReader`
#[derive(Debug, Clone, Default)]
pub struct MapSheetReader;
impl MapSheetReader {
    /// 对齐 Java: `MapSheetReader.read(Sheet sheet, int rowIndex, int columnIndex, Cell cell)`
    pub fn read(
        &mut self,
        _row_index: i32,
        _column_index: i32,
        _value: &str,
    ) -> Result<std::collections::BTreeMap<String, String>> {
        Err(PoiError::PendingEngine(
            "MapSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.reader.ListSheetReader`
#[derive(Debug, Clone, Default)]
pub struct ListSheetReader;
impl ListSheetReader {
    /// 对齐 Java: `ListSheetReader.read(Sheet sheet, int rowIndex, int columnIndex, Cell cell)`
    pub fn read(&mut self, _row_index: i32, _column_index: i32, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ListSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.reader.ColumnSheetReader<T>`
#[derive(Debug, Clone, Default)]
pub struct ColumnSheetReader;
impl ColumnSheetReader {
    /// 对齐 Java: `new ColumnSheetReader(Class<T> beanType)`
    pub fn new<T>(_bean_type: std::marker::PhantomData<T>) -> Self {
        Self
    }
    /// 对齐 Java: `ColumnSheetReader.read(Sheet sheet, int rowIndex, int columnIndex, Cell cell)`
    pub fn read<T>(&mut self, _row_index: i32, _column_index: i32, _value: &str) -> Result<T> {
        Err(PoiError::PendingEngine(
            "ColumnSheetReader::read (waiting for easyexcel-rs)",
        ))
    }
}