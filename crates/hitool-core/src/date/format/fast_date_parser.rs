//! 对齐: `cn.hutool.core.date.format.FastDateParser.java.FastDateParser`
//! 来源: hutool-core/src/main/java/cn/hutool/core/date/format.FastDateParser.java

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.format.FastDateParser` 的类占位。
///
/// 所有方法均返回 [`CoreError::PendingEngine`],等待对应引擎完成后实现;
/// 单元测试在 `crates/hitool-core/src/date/format/fast_date_parser.rs::sentinel` 中断言当前占位行为。
#[derive(Debug, Clone, Copy, Default)]
pub struct FastDateParser;

impl FastDateParser {
    /// 对齐 Java: `sentinel` — 占位入口,用于在 parity 中证明该类已对齐签名。
    pub fn sentinel() -> Result<()> {
        Err(CoreError::PendingEngine("FastDateParser::sentinel"))
    }
}
