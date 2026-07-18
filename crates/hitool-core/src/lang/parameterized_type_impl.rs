//! 对齐: `cn.hutool.core.lang.ParameterizedTypeImpl`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ParameterizedTypeImpl.java
//!
//! Hutool 的 `ParameterizedTypeImpl` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.ParameterizedTypeImpl`
#[derive(Debug, Clone, Default)]
pub struct ParameterizedTypeImpl;

impl ParameterizedTypeImpl {
    /// 对齐 Java: `ParameterizedTypeImpl.getActualTypeArguments()`
    #[allow(clippy::too_many_arguments)]
    pub fn getActualTypeArguments() -> Result<Type[]> {
        Err(CoreError::PendingEngine("ParameterizedTypeImpl::getActualTypeArguments (waiting for full impl)"))
    }
    /// 对齐 Java: `ParameterizedTypeImpl.getOwnerType()`
    #[allow(clippy::too_many_arguments)]
    pub fn getOwnerType() -> Result<Type> {
        Err(CoreError::PendingEngine("ParameterizedTypeImpl::getOwnerType (waiting for full impl)"))
    }
    /// 对齐 Java: `ParameterizedTypeImpl.getRawType()`
    #[allow(clippy::too_many_arguments)]
    pub fn getRawType() -> Result<Type> {
        Err(CoreError::PendingEngine("ParameterizedTypeImpl::getRawType (waiting for full impl)"))
    }
}
