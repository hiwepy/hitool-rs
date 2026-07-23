//! 对齐: `cn.hutool.core.lang.ref.ReferenceType`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ref/ReferenceType.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.lang.ref.ReferenceType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    /// 对齐 Java 枚举常量: `STRONG`
    STRONG,
    /// 对齐 Java 枚举常量: `SOFT`
    SOFT,
    /// 对齐 Java 枚举常量: `WEAK`
    WEAK,
}
