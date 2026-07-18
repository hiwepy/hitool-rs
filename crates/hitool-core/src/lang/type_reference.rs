//! 对齐: `cn.hutool.core.lang.TypeReference`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/TypeReference.java
//!
//! Hutool 的 `TypeReference` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.TypeReference`
#[derive(Debug, Clone, Default)]
pub struct TypeReference;

impl TypeReference {
    /// 对齐 Java: `TypeReference.getType()`
    #[allow(clippy::too_many_arguments)]
    pub fn getType() -> Result<Type> {
        Err(CoreError::PendingEngine("TypeReference::getType (waiting for full impl)"))
    }
}
