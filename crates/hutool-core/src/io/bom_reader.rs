//! 对齐: `cn.hutool.core.io.BomReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/BomReader.java
//!
//! 读取带BOM头的流内容的Reader，如果非bom的流或无法识别的编码，则默认UTF-8<br>
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 读取带BOM头的流内容的Reader，如果非bom的流或无法识别的编码，则默认UTF-8<br>
#[derive(Debug, Clone, Copy, Default)]
pub struct BomReader;

impl BomReader {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine("BomReader::pending_io_alignment"))
    }
}
