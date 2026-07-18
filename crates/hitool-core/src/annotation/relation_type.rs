//! 对齐: `cn.hutool.core.annotation.RelationType`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/RelationType.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.annotation.RelationType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    /// 对齐 Java 枚举常量: `MIRROR_FOR`
    MIRROR_FOR,
    /// 对齐 Java 枚举常量: `ALIAS_FOR`
    ALIAS_FOR,
    /// 对齐 Java 枚举常量: `FORCE_ALIAS_FOR`
    FORCE_ALIAS_FOR,
}
