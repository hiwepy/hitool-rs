//! 对齐: `cn.hutool.core.text.csv.CsvConfig`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvConfig.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvConfig<T extends CsvConfig<T>>#`
#[derive(Debug, Clone)]
pub struct CsvConfig;

impl CsvConfig {
    /// 对齐 Java: `CsvConfig::setFieldSeparator#CsvConfig (char)`
    pub fn set_field_separator(&mut self, _sep: char) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("CsvConfig::set_field_separator"))
    }

    /// 对齐 Java: `CsvConfig::setTextDelimiter#CsvConfig (char)`
    pub fn set_text_delimiter(&mut self, _delim: char) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("CsvConfig::set_text_delimiter"))
    }
}