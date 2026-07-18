//! 对齐: `cn.hutool.core.io.watch.WatchKind`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/watch/WatchKind.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.io.watch.WatchKind`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchKind {
    /// 对齐 Java 枚举常量: `OVERFLOW`
    OVERFLOW,
    /// 对齐 Java 枚举常量: `MODIFY`
    MODIFY,
    /// 对齐 Java 枚举常量: `CREATE`
    CREATE,
    /// 对齐 Java 枚举常量: `DELETE`
    DELETE,
}
