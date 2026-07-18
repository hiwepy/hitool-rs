//! 对齐: `cn.hutool.core.lang.Pair`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Pair.java
//!
//! Hutool 的 `Pair` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.Pair` (容器类型)
#[derive(Debug, Clone, Default)]
pub struct Pair;

impl Pair {
    /// 对齐 Java: `Pair.of(K key, V value)`
    #[allow(clippy::too_many_arguments)]
    pub fn of(K key, V value) -> Result<Pair<K, V>> {
        Err(CoreError::PendingEngine("Pair::of (waiting for full impl)"))
    }
    /// 对齐 Java: `Pair.getKey()`
    #[allow(clippy::too_many_arguments)]
    pub fn getKey() -> Result<K> {
        Err(CoreError::PendingEngine("Pair::getKey (waiting for full impl)"))
    }
    /// 对齐 Java: `Pair.getValue()`
    #[allow(clippy::too_many_arguments)]
    pub fn getValue() -> Result<V> {
        Err(CoreError::PendingEngine("Pair::getValue (waiting for full impl)"))
    }
    /// 对齐 Java: `Pair.equals(Object o)`
    #[allow(clippy::too_many_arguments)]
    pub fn equals(Object o) -> Result<bool> {
        Err(CoreError::PendingEngine("Pair::equals (waiting for full impl)"))
    }
}
