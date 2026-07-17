//! Cell location value aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.cell.CellLocation`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/cell/CellLocation.java
//!
//! `CellLocation` 是 Hutool 对 A1 风格单元格引用的不可变包装,
//! 提供 `x`/`y` 字段、`isAt`/`equals` 等比较方法。

use crate::{PoiError, Result};

/// Cell location A1 reference.
///
/// 对齐 Java: `cn.hutool.poi.excel.cell.CellLocation`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellLocation {
    /// X coordinate (zero-based column).
    pub x: u32,
    /// Y coordinate (zero-based row).
    pub y: u32,
}

impl CellLocation {
    /// 对齐 Java: `new CellLocation(String locationRef)`
    pub fn new(_location_ref: &str) -> Result<Self> {
        Err(PoiError::PendingEngine(
            "CellLocation::new (waiting for easyexcel-rs)",
        ))
    }

    /// 对齐 Java: `new CellLocation(int x, int y)`
    pub fn new_xy(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// 对齐 Java: `CellLocation.getX()`
    pub fn get_x(&self) -> u32 {
        self.x
    }
    /// 对齐 Java: `CellLocation.getY()`
    pub fn get_y(&self) -> u32 {
        self.y
    }
    /// 对齐 Java: `CellLocation.toString()`
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
    /// 对齐 Java: `CellLocation.equals(Object)`
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
    /// 对齐 Java: `CellLocation.isAt(int, int)`
    pub fn is_at(&self, x: u32, y: u32) -> bool {
        self.x == x && self.y == y
    }
}