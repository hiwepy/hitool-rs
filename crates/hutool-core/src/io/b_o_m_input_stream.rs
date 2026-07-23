//! 对齐: `cn.hutool.core.io.BOMInputStream`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/BOMInputStream.java
//!
//! 读取带BOM头的流内容，{@code getCharset()}方法调用后会得到BOM头的编码，且会去除BOM头<br>
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 读取带BOM头的流内容，{@code getCharset()}方法调用后会得到BOM头的编码，且会去除BOM头<br>
#[derive(Debug, Clone, Copy, Default)]
pub struct BOMInputStream;

impl BOMInputStream {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine("BOMInputStream::pending_io_alignment"))
    }
}
