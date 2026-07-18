//! Cell alignment enum aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.style.Align`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/style/Align.java
//!
//! Hutool 的 `Align` 枚举把 POI 的 `HorizontalAlignment` / `VerticalAlignment`
//! 整合为 9 个语义对齐方式(`LEFT`、`CENTER`、`RIGHT`、`TOP_LEFT`...),
//! 便于链式调用 `StyleSet.setAlign`。

/// Combined alignment enumeration.
///
/// 对齐 Java: `cn.hutool.poi.excel.style.Align`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// 对齐 Java: `Align.LEFT`
    Left,
    /// 对齐 Java: `Align.CENTER`
    Center,
    /// 对齐 Java: `Align.RIGHT`
    Right,
    /// 对齐 Java: `Align.TOP_LEFT`
    TopLeft,
    /// 对齐 Java: `Align.TOP_CENTER`
    TopCenter,
    /// 对齐 Java: `Align.TOP_RIGHT`
    TopRight,
    /// 对齐 Java: `Align.BOTTOM_LEFT`
    BottomLeft,
    /// 对齐 Java: `Align.BOTTOM_CENTER`
    BottomCenter,
    /// 对齐 Java: `Align.BOTTOM_RIGHT`
    BottomRight,
}

impl Align {
    /// 对齐 Java: `Align.values()`
    pub fn values() -> &'static [Align] {
        &[
            Align::Left, Align::Center, Align::Right,
            Align::TopLeft, Align::TopCenter, Align::TopRight,
            Align::BottomLeft, Align::BottomCenter, Align::BottomRight,
        ]
    }
}