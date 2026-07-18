//! 对齐: `cn.hutool.core.text.NamingCase`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/NamingCase.java
//!
//! 命名规则封装:驼峰、下划线、短横连接等命名风格互转。

use crate::{CoreError, Result};

/// 对齐 Java: `NamingCase#`
#[derive(Debug, Clone, Copy, Default)]
pub struct NamingCase;

impl NamingCase {
    /// 对齐 Java: `NamingCase::toUnderlineCase#String (CharSequence str)`
    pub fn to_underline_case(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_underline_case"))
    }

    /// 对齐 Java: `NamingCase::toKebabCase#String (CharSequence str)`
    pub fn to_kebab_case(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_kebab_case"))
    }

    /// 对齐 Java: `NamingCase::toSymbolCase#String (CharSequence str, char symbol)`
    pub fn to_symbol_case(_str: &str, _symbol: char) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_symbol_case"))
    }

    /// 对齐 Java: `NamingCase::toPascalCase#String (CharSequence name)`
    pub fn to_pascal_case(_name: &str) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_pascal_case"))
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name)`
    pub fn to_camel_case(_name: &str) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_camel_case"))
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name, char symbol)`
    pub fn to_camel_case_symbol(_name: &str, _symbol: char) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_camel_case_symbol"))
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name, char symbol, boolean otherCharToLower)`
    pub fn to_camel_case_full(
        _name: &str,
        _symbol: char,
        _other_lower: bool,
    ) -> Result<String> {
        Err(CoreError::PendingEngine("NamingCase::to_camel_case_full"))
    }
}