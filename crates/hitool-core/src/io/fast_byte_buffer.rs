//! 对齐: `cn.hutool.core.io.FastByteBuffer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FastByteBuffer.java
//!
//! 代码移植自<a href="https://github.com/biezhi/blade">blade</a><br>
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 代码移植自<a href="https://github.com/biezhi/blade">blade</a><br>
#[derive(Debug, Clone, Copy, Default)]
pub struct FastByteBuffer;

impl FastByteBuffer {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine("FastByteBuffer::pending_io_alignment"))
    }
}
