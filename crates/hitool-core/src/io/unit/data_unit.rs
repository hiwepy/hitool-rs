//! 对齐: `cn.hutool.core.io.unit.DataUnit`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/unit/DataUnit.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.io.unit.DataUnit`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataUnit {
    /// 对齐 Java 枚举常量: `BYTES`
    BYTES,
    /// 对齐 Java 枚举常量: `KILOBYTES`
    KILOBYTES,
    /// 对齐 Java 枚举常量: `MEGABYTES`
    MEGABYTES,
    /// 对齐 Java 枚举常量: `GIGABYTES`
    GIGABYTES,
    /// 对齐 Java 枚举常量: `TERABYTES`
    TERABYTES,
}
