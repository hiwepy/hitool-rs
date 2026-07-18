//! SAX element name enumeration aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.ElementName`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/ElementName.java

/// SAX element name enumeration.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.ElementName`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementName {
    /// 对齐 Java: `ElementName.sheetData`
    SheetData,
    /// 对齐 Java: `ElementName.row`
    Row,
    /// 对齐 Java: `ElementName.c`
    C,
    /// 对齐 Java: `ElementName.v`
    V,
    /// 对齐 Java: `ElementName.f`
    F,
    /// 对齐 Java: `ElementName.is`
    Is,
    /// 对齐 Java: `ElementName.t`
    T,
    /// 对齐 Java: `ElementName.si`
    Si,
}

impl ElementName {
    /// 对齐 Java: `ElementName.values()`
    pub fn values() -> &'static [ElementName] {
        &[
            ElementName::SheetData, ElementName::Row, ElementName::C,
            ElementName::V, ElementName::F, ElementName::Is,
            ElementName::T, ElementName::Si,
        ]
    }
}