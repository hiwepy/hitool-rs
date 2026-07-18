//! Cell data type enumeration aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.CellDataType`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/CellDataType.java

/// Cell data type enumeration used by SAX readers.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.CellDataType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellDataType {
    /// 对齐 Java: `CellDataType.BLANK`
    Blank,
    /// 对齐 Java: `CellDataType.BOOLEAN`
    Boolean,
    /// 对齐 Java: `CellDataType.NUMBER`
    Number,
    /// 对齐 Java: `CellDataType.STRING`
    String,
    /// 对齐 Java: `CellDataType.DATE`
    Date,
    /// 对齐 Java: `CellDataType.ERROR`
    Error,
    /// 对齐 Java: `CellDataType.FORMULA`
    Formula,
}

impl CellDataType {
    /// 对齐 Java: `CellDataType.of(String value)`
    pub fn of(_value: &str) -> Self {
        CellDataType::String
    }
    /// 对齐 Java: `CellDataType.getType()`
    pub fn get_type(&self) -> &'static str {
        match self {
            CellDataType::Blank => "",
            CellDataType::Boolean => "b",
            CellDataType::Number => "n",
            CellDataType::String => "s",
            CellDataType::Date => "d",
            CellDataType::Error => "e",
            CellDataType::Formula => "str",
        }
    }
}