//! 对齐: `cn.hutool.core.lang.ansi.AnsiStyle`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ansi/AnsiStyle.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.lang.ansi.AnsiStyle`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiStyle {
    /// 对齐 Java 枚举常量: `NORMAL`
    NORMAL,
    /// 对齐 Java 枚举常量: `BOLD`
    BOLD,
    /// 对齐 Java 枚举常量: `FAINT`
    FAINT,
    /// 对齐 Java 枚举常量: `ITALIC`
    ITALIC,
    /// 对齐 Java 枚举常量: `UNDERLINE`
    UNDERLINE,
}
