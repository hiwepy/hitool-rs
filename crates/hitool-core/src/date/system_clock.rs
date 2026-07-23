//! 对齐: `cn.hutool.core.date.SystemClock`

use crate::Result;

/// 对齐 Java: `cn.hutool.core.date.SystemClock`
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl SystemClock {
    /// 当前毫秒。
    pub fn now() -> i64 {
        chrono::Utc::now().timestamp_millis()
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
