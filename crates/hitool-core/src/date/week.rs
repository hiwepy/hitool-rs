//! 对齐: `cn.hutool.core.date.Week.java.Week`
//! 来源: hutool-core/src/main/java/cn/hutool/core/date/Week.java

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.week.Week` 的枚举占位。
///
/// 所有方法均返回 [`CoreError::PendingEngine`],等待对应引擎完成后实现;
/// 单元测试在 `crates/hitool-core/src/date/week.rs::sentinel` 中断言当前占位行为。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Week {}

impl Week {
    /// 对齐 Java: `sentinel` — 占位入口,用于在 parity 中证明该枚举已对齐签名。
    pub fn sentinel() -> Result<()> {
        Err(CoreError::PendingEngine("Week::sentinel"))
    }
}
