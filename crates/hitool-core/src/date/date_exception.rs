//! 对齐: `cn.hutool.core.date.DateException`

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.DateException` — 映射为 [`CoreError`]。
#[derive(Debug, Clone, Copy, Default)]
pub struct DateException;

impl DateException {
    /// 构造解析错误。
    pub fn parse(msg: &'static str) -> CoreError {
        CoreError::InvalidArgument {
            name: "date",
            reason: msg,
        }
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
