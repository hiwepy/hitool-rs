//! 对齐: `cn.hutool.core.text.csv.CsvParser`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvParser.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvParser#`
#[derive(Debug, Clone, Copy)]
pub struct CsvParser;

impl CsvParser {
    /// 对齐 Java: `CsvParser::parseLine` 类方法(静态入口)
    pub fn parse_line(_line: &str) -> Result<()> {
        Err(CoreError::PendingEngine("CsvParser::parse_line"))
    }
}