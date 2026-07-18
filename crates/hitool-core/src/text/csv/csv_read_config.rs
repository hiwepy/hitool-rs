//! 对齐: `cn.hutool.core.text.csv.CsvReadConfig`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvReadConfig.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvReadConfig#`
#[derive(Debug, Clone)]
pub struct CsvReadConfig;

impl CsvReadConfig {
    /// 对齐 Java: `CsvReadConfig::defaultConfig()`
    pub fn default_config() -> Self {
        Self
    }

    /// 对齐 Java: `CsvReadConfig::setContainsHeader#CsvReadConfig (boolean)`
    pub fn set_contains_header(&mut self, _contains: bool) -> Result<&mut Self> {
        Err(CoreError::PendingEngine(
            "CsvReadConfig::set_contains_header",
        ))
    }
}