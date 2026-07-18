//! 对齐: `cn.hutool.core.text.csv.CsvWriteConfig`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvWriteConfig.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvWriteConfig#`
#[derive(Debug, Clone)]
pub struct CsvWriteConfig;

impl CsvWriteConfig {
    /// 对齐 Java: `CsvWriteConfig::defaultConfig()`
    pub fn default_config() -> Self {
        Self
    }

    /// 对齐 Java: `CsvWriteConfig::setLineDelimiter#CsvWriteConfig (char[])`
    pub fn set_line_delimiter(&mut self, _delim: &[char]) -> Result<&mut Self> {
        Err(CoreError::PendingEngine(
            "CsvWriteConfig::set_line_delimiter",
        ))
    }

    /// 对齐 Java: `CsvWriteConfig::setAlwaysDelimitText#CsvWriteConfig (boolean)`
    pub fn set_always_delimit_text(&mut self, _always: bool) -> Result<&mut Self> {
        Err(CoreError::PendingEngine(
            "CsvWriteConfig::set_always_delimit_text",
        ))
    }
}