//! Cell setter implementations aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.setters.*`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/setters/
//!
//! `cn.hutool.poi.excel.cell.setters` 子包包含 11 个具体 `CellSetter`:
//! - BooleanCellSetter / NumberCellSetter / CharSequenceCellSetter
//! - DateCellSetter / CalendarCellSetter / TemporalAccessorCellSetter
//! - HyperlinkCellSetter / RichTextCellSetter / EscapeStrCellSetter
//! - NullCellSetter / CellSetterFactory

use crate::{PoiError, Result};

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.BooleanCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct BooleanCellSetter;
impl BooleanCellSetter {
    /// 对齐 Java: `new BooleanCellSetter(Cell cell, boolean value)`
    pub fn new(_cell: (), _value: bool) -> Self {
        Self
    }
    /// 对齐 Java: `BooleanCellSetter.setValue(Cell cell, boolean)`
    pub fn set_value(&mut self, _value: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "BooleanCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.NumberCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberCellSetter;
impl NumberCellSetter {
    /// 对齐 Java: `new NumberCellSetter(Cell cell, Number value)`
    pub fn new(_cell: (), _value: f64) -> Self {
        Self
    }
    /// 对齐 Java: `NumberCellSetter.setValue(Cell cell, Number)`
    pub fn set_value(&mut self, _value: f64) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NumberCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.CharSequenceCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct CharSequenceCellSetter;
impl CharSequenceCellSetter {
    /// 对齐 Java: `new CharSequenceCellSetter(Cell cell, CharSequence value)`
    pub fn new(_cell: (), _value: &str) -> Self {
        Self
    }
    /// 对齐 Java: `CharSequenceCellSetter.setValue(Cell cell, CharSequence)`
    pub fn set_value(&mut self, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "CharSequenceCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.DateCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct DateCellSetter;
impl DateCellSetter {
    /// 对齐 Java: `new DateCellSetter(Cell cell, Date value)`
    pub fn new(_cell: (), _value: &str) -> Self {
        Self
    }
    /// 对齐 Java: `DateCellSetter.setValue(Cell cell, Date)`
    pub fn set_value(&mut self, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "DateCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.CalendarCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct CalendarCellSetter;
impl CalendarCellSetter {
    /// 对齐 Java: `new CalendarCellSetter(Cell cell, Calendar value)`
    pub fn new(_cell: (), _value: &str) -> Self {
        Self
    }
    /// 对齐 Java: `CalendarCellSetter.setValue(Cell cell, Calendar)`
    pub fn set_value(&mut self, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "CalendarCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.TemporalAccessorCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalAccessorCellSetter;
impl TemporalAccessorCellSetter {
    /// 对齐 Java: `new TemporalAccessorCellSetter(Cell cell, TemporalAccessor value)`
    pub fn new(_cell: (), _value: &str) -> Self {
        Self
    }
    /// 对齐 Java: `TemporalAccessorCellSetter.setValue(Cell cell, TemporalAccessor)`
    pub fn set_value(&mut self, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "TemporalAccessorCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.HyperlinkCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct HyperlinkCellSetter;
impl HyperlinkCellSetter {
    /// 对齐 Java: `new HyperlinkCellSetter(Cell cell, Hyperlink value)`
    pub fn new(_cell: (), _value: ()) -> Self {
        Self
    }
    /// 对齐 Java: `HyperlinkCellSetter.setValue(Cell cell, Hyperlink)`
    pub fn set_value(&mut self, _value: ()) -> Result<()> {
        Err(PoiError::PendingEngine(
            "HyperlinkCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.RichTextCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct RichTextCellSetter;
impl RichTextCellSetter {
    /// 对齐 Java: `new RichTextCellSetter(Cell cell, RichTextString value)`
    pub fn new(_cell: (), _value: ()) -> Self {
        Self
    }
    /// 对齐 Java: `RichTextCellSetter.setValue(Cell cell, RichTextString)`
    pub fn set_value(&mut self, _value: ()) -> Result<()> {
        Err(PoiError::PendingEngine(
            "RichTextCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.EscapeStrCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct EscapeStrCellSetter;
impl EscapeStrCellSetter {
    /// 对齐 Java: `new EscapeStrCellSetter(Cell cell, String value)`
    pub fn new(_cell: (), _value: &str) -> Self {
        Self
    }
    /// 对齐 Java: `EscapeStrCellSetter.setValue(Cell cell, String)`
    pub fn set_value(&mut self, _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "EscapeStrCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.NullCellSetter`
#[derive(Debug, Clone, Copy, Default)]
pub struct NullCellSetter;
impl NullCellSetter {
    /// 对齐 Java: `new NullCellSetter(Cell cell)`
    pub fn new(_cell: ()) -> Self {
        Self
    }
    /// 对齐 Java: `NullCellSetter.setValue(Cell cell)`
    pub fn set_value(&mut self) -> Result<()> {
        Err(PoiError::PendingEngine(
            "NullCellSetter::setValue (waiting for easyexcel-rs)",
        ))
    }
}

/// 对齐 Java: `cn.hutool.poi.excel.cell.setters.CellSetterFactory`
#[derive(Debug, Clone, Copy, Default)]
pub struct CellSetterFactory;
impl CellSetterFactory {
    /// 对齐 Java: `CellSetterFactory.create(Cell cell, Object value)`
    pub fn create(_cell: (), _value: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "CellSetterFactory::create (waiting for easyexcel-rs)",
        ))
    }
}