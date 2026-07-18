//! SAX attribute name enumeration aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.sax.AttributeName`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/AttributeName.java
//!
//! `AttributeName` 枚举 XLSX XML 流中常见的属性名,例如 `r`(单元格引用)、
//! `t`(数据类型)、`s`(样式索引)等。

/// SAX attribute name enumeration.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.AttributeName`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeName {
    /// 对齐 Java: `AttributeName.r`
    R,
    /// 对齐 Java: `AttributeName.t`
    T,
    /// 对齐 Java: `AttributeName.s`
    S,
    /// 对齐 Java: `AttributeName.ref`
    Ref,
    /// 对齐 Java: `AttributeName.si`
    Si,
    /// 对齐 Java: `AttributeName.id`
    Id,
}

impl AttributeName {
    /// 对齐 Java: `AttributeName.values()`
    pub fn values() -> &'static [AttributeName] {
        &[
            AttributeName::R, AttributeName::T, AttributeName::S,
            AttributeName::Ref, AttributeName::Si, AttributeName::Id,
        ]
    }
}