//! 对齐: `cn.hutool.core.io.NioUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/NioUtil.java
//!
//! NIO相关工具封装，主要针对Channel读写、拷贝等封装
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// NIO相关工具封装，主要针对Channel读写、拷贝等封装
#[derive(Debug, Clone, Copy, Default)]
pub struct NioUtil;

impl NioUtil {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine("NioUtil::pending_io_alignment"))
    }
}
