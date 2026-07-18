//! 对齐: `cn.hutool.core.io.FastByteArrayOutputStream`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FastByteArrayOutputStream.java
//!
//! 基于快速缓冲FastByteBuffer的OutputStream，随着数据的增长自动扩充缓冲区
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 基于快速缓冲FastByteBuffer的OutputStream，随着数据的增长自动扩充缓冲区
#[derive(Debug, Clone, Copy, Default)]
pub struct FastByteArrayOutputStream;

impl FastByteArrayOutputStream {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine("FastByteArrayOutputStream::pending_io_alignment"))
    }
}
