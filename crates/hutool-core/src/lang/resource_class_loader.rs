//! 对齐: `cn.hutool.core.lang.ResourceClassLoader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ResourceClassLoader.java
//!
//! Hutool 的 `ResourceClassLoader` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hutool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.ResourceClassLoader`
#[derive(Debug, Clone, Default)]
pub struct ResourceClassLoader;

impl ResourceClassLoader {
    /// 对齐 Java: `ResourceClassLoader.addResource(T resource)`
    #[allow(clippy::too_many_arguments)]
    pub fn addResource(T resource) -> Result<ResourceClassLoader<T>> {
        Err(CoreError::PendingEngine("ResourceClassLoader::addResource (waiting for full impl)"))
    }
}
