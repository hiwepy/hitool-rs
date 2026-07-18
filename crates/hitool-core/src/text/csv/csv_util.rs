//! 对齐: `cn.hutool.core.text.csv.CsvUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvUtil.java

use crate::{CoreError, Result};

/// 对齐 Java: `CsvUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct CsvUtil;

impl CsvUtil {
    /// 对齐 Java: `CsvUtil::getReader(CsvReadConfig config)`
    pub fn get_reader(_config: ()) -> Result<()> {
        Err(CoreError::PendingEngine("CsvUtil::get_reader"))
    }

    /// 对齐 Java: `CsvUtil::getReader()`
    pub fn get_reader_default() -> Result<()> {
        Err(CoreError::PendingEngine("CsvUtil::get_reader_default"))
    }
}