//! 对齐: `cn.hutool.core.io.IORuntimeException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/IORuntimeException.java
//!
//! IO运行时异常，常用于对IOException的包装
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// IO运行时异常，常用于对IOException的包装
#[derive(Debug, thiserror::Error)]
pub enum IORuntimeException {
    /// Placeholder for the Java runtime exception.
    #[error("{0}")]
    Message(String),
}
