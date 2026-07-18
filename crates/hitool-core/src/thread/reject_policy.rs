//! 对齐: `cn.hutool.core.thread.RejectPolicy`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/RejectPolicy.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.thread.RejectPolicy`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectPolicy {
    /// 对齐 Java 枚举常量: `ABORT`
    ABORT,
    /// 对齐 Java 枚举常量: `DISCARD`
    DISCARD,
    /// 对齐 Java 枚举常量: `DISCARD_OLDEST`
    DISCARD_OLDEST,
    /// 对齐 Java 枚举常量: `CALLER_RUNS`
    CALLER_RUNS,
    /// 对齐 Java 枚举常量: `BLOCK`
    BLOCK,
}
