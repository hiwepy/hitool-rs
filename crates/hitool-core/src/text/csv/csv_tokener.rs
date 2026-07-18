//! 对齐: `cn.hutool.core.text.csv.CsvTokener`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvTokener.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvTokener#`
#[derive(Debug, Clone)]
pub struct CsvTokener;

impl CsvTokener {
    /// 对齐 Java: `CsvTokener::nextToken#String ()`
    pub fn next_token(&mut self) -> Result<String> {
        Err(CoreError::PendingEngine("CsvTokener::next_token"))
    }

    /// 对齐 Java: `CsvTokener::close#void ()`
    pub fn close(&mut self) -> Result<()> {
        Err(CoreError::PendingEngine("CsvTokener::close"))
    }
}