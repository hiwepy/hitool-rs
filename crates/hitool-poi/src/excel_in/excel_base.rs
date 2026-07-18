//! Shared Excel read/write base aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelBase<T>`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelBase.java
//!
//! `ExcelBase` 是 `ExcelReader` / `ExcelWriter` 的泛型父类,集中实现工作簿
//! 工作表、单元格、样式与表头别名等公共行为。本文件按 Java 签名声明对齐桩。

use crate::{PoiError, Result};

/// Placeholder cell reference.
///
/// 对齐 Java: `org.apache.poi.ss.usermodel.Cell`
#[derive(Debug, Clone, Default)]
pub struct CellRef;

/// Placeholder row reference.
///
/// 对齐 Java: `org.apache.poi.ss.usermodel.Row`
#[derive(Debug, Clone, Default)]
pub struct RowRef;

/// Placeholder sheet reference.
///
/// 对齐 Java: `org.apache.poi.ss.usermodel.Sheet`
#[derive(Debug, Clone, Default)]
pub struct SheetRef;

/// Placeholder workbook reference (also defined in `excel_writer.rs`; here
/// kept independent to avoid cross-module coupling for the stub).
#[derive(Debug, Clone, Default)]
pub struct WbRef;

/// Placeholder cell style reference.
#[derive(Debug, Clone, Default)]
pub struct CssRef;

/// Placeholder hyperlink reference.
#[derive(Debug, Clone, Default)]
pub struct HlRef;

/// Placeholder hyperlink type enum.
///
/// 对齐 Java: `org.apache.poi.common.usermodel.HyperlinkType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HyperlinkKind {
    Url,
    Email,
    File,
    Document,
}

/// Excel base facade.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelBase<T>`
#[derive(Debug, Clone, Default)]
pub struct ExcelBase {
    _private: (),
}

impl ExcelBase {
    /// 对齐 Java: `ExcelBase(Sheet sheet)`
    pub fn new(_sheet: SheetRef) -> Self {
        Self { _private: () }
    }

    /// 对齐 Java: `ExcelBase.getWorkbook()`
    pub fn get_workbook(&self) -> Option<WbRef> {
        None
    }
    /// 对齐 Java: `ExcelBase.getSheetCount()`
    pub fn get_sheet_count(&self) -> i32 {
        0
    }
    /// 对齐 Java: `ExcelBase.getSheets()`
    pub fn get_sheets(&self) -> Vec<SheetRef> {
        Vec::new()
    }
    /// 对齐 Java: `ExcelBase.getSheetNames()`
    pub fn get_sheet_names(&self) -> Vec<String> {
        Vec::new()
    }
    /// 对齐 Java: `ExcelBase.getSheet()`
    pub fn get_sheet(&self) -> Option<SheetRef> {
        None
    }

    /// 对齐 Java: `ExcelBase.renameSheet(String)`
    pub fn rename_sheet(self, _new_name: &str) -> Self {
        self
    }

    /// 对齐 Java: `ExcelBase.setSheet(String)`
    pub fn set_sheet_name(self, _sheet: &str) -> Self {
        self
    }
    /// 对齐 Java: `ExcelBase.setSheet(int)`
    pub fn set_sheet_index(self, _index: i32) -> Self {
        self
    }
    /// 对齐 Java: `ExcelBase.setSheet(Sheet)`
    pub fn set_sheet_ref(self, _sheet: SheetRef) -> Self {
        self
    }

    /// 对齐 Java: `ExcelBase.cloneSheet(int, String, boolean)`
    pub fn clone_sheet(
        self,
        _sheet_index: i32,
        _new_sheet_name: &str,
        _set_as_current: bool,
    ) -> Result<Self> {
        Err(PoiError::PendingEngine(
            "ExcelBase::cloneSheet (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelBase.getCell(String)`
    pub fn get_cell_str(&self, _location_ref: &str) -> Option<CellRef> {
        None
    }
    /// 对齐 Java: `ExcelBase.getCell(int, int)`
    pub fn get_cell(&self, _x: i32, _y: i32) -> Option<CellRef> {
        None
    }
    /// 对齐 Java: `ExcelBase.getOrCreateCell(String)`
    pub fn get_or_create_cell_str(&self, _location_ref: &str) -> Result<CellRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateCell (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getOrCreateCell(int, int)`
    pub fn get_or_create_cell(&self, _x: i32, _y: i32) -> Result<CellRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateCell (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getCell(String, boolean)`
    pub fn get_cell_str_or_create(
        &self,
        _location_ref: &str,
        _create_if_missing: bool,
    ) -> Result<CellRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getCell (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getCell(int, int, boolean)`
    pub fn get_cell_or_create(
        &self,
        _x: i32,
        _y: i32,
        _create_if_missing: bool,
    ) -> Result<CellRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getCell (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getOrCreateRow(int)`
    pub fn get_or_create_row(&self, _y: i32) -> Result<RowRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateRow (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelBase.getOrCreateCellStyle(String)`
    pub fn get_or_create_cell_style_str(&self, _location_ref: &str) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getOrCreateCellStyle(int, int)`
    pub fn get_or_create_cell_style(&self, _x: i32, _y: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createCellStyle(String)`
    pub fn create_cell_style_str(&self, _location_ref: &str) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createCellStyle(int, int)`
    pub fn create_cell_style(&self, _x: i32, _y: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createCellStyle()`
    pub fn create_default_cell_style(&self) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createCellStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getOrCreateRowStyle(int)`
    pub fn get_or_create_row_style(&self, _y: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateRowStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createRowStyle(int)`
    pub fn create_row_style(&self, _y: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createRowStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.getOrCreateColumnStyle(int)`
    pub fn get_or_create_column_style(&self, _x: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::getOrCreateColumnStyle (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createColumnStyle(int)`
    pub fn create_column_style(&self, _x: i32) -> Result<CssRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createColumnStyle (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelBase.createHyperlink(HyperlinkType, String)`
    pub fn create_hyperlink(&self, _kind: HyperlinkKind, _address: &str) -> Result<HlRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createHyperlink (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelBase.createHyperlink(HyperlinkType, String, String)`
    pub fn create_hyperlink_with_label(
        &self,
        _kind: HyperlinkKind,
        _address: &str,
        _label: &str,
    ) -> Result<HlRef> {
        Err(PoiError::PendingEngine(
            "ExcelBase::createHyperlink (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `ExcelBase.getRowCount()`
    pub fn get_row_count(&self) -> i32 {
        0
    }
    /// 对齐 Java: `ExcelBase.getPhysicalRowCount()`
    pub fn get_physical_row_count(&self) -> i32 {
        0
    }
    /// 对齐 Java: `ExcelBase.getColumnCount()`
    pub fn get_column_count(&self) -> i32 {
        0
    }
    /// 对齐 Java: `ExcelBase.getColumnCount(int)`
    pub fn get_column_count_at(&self, _row_index: i32) -> i32 {
        0
    }
    /// 对齐 Java: `ExcelBase.isXlsx()`
    pub fn is_xlsx(&self) -> bool {
        true
    }

    /// 对齐 Java: `ExcelBase.close()`
    pub fn close(self) {}

    /// 对齐 Java: `ExcelBase.getHeaderAlias()`
    pub fn get_header_alias(&self) -> std::collections::BTreeMap<String, String> {
        std::collections::BTreeMap::new()
    }
    /// 对齐 Java: `ExcelBase.setHeaderAlias(Map)`
    pub fn set_header_alias(self, _aliases: &[(&str, &str)]) -> Self {
        self
    }
    /// 对齐 Java: `ExcelBase.addHeaderAlias(String, String)`
    pub fn add_header_alias(self, _name: &str, _alias: &str) -> Self {
        self
    }
    /// 对齐 Java: `ExcelBase.removeHeaderAlias(String)`
    pub fn remove_header_alias(self, _name: &str) -> Self {
        self
    }
    /// 对齐 Java: `ExcelBase.clearHeaderAlias()`
    pub fn clear_header_alias(self) -> Self {
        self
    }
}