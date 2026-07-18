//! 对齐: `cn.hutool.core.date.LocalDateTimeUtil.java.LocalDateTimeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/date/LocalDateTimeUtil.java

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.localdatetimeutil.LocalDateTimeUtil` 的类占位。
///
/// 所有方法均返回 [`CoreError::PendingEngine`],等待对应引擎完成后实现;
/// 单元测试在 `crates/hitool-core/src/date/local_date_time_util.rs::sentinel` 中断言当前占位行为。
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalDateTimeUtil;

impl LocalDateTimeUtil {
    /// 对齐 Java: `sentinel` — 占位入口,用于在 parity 中证明该类已对齐签名。
    pub fn sentinel() -> Result<()> {
        Err(CoreError::PendingEngine("LocalDateTimeUtil::sentinel"))
    }
}
