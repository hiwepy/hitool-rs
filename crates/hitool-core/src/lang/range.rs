//! 对齐: `cn.hutool.core.lang.Range`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Range.java
//!
//! Hutool 的 `Range` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.Range` (容器类型)
#[derive(Debug, Clone, Default)]
pub struct Range;

impl Range {
    /// 对齐 Java: `Range.disableLock()`
    #[allow(clippy::too_many_arguments)]
    pub fn disableLock() -> Result<Range<T>> {
        Err(CoreError::PendingEngine("Range::disableLock (waiting for full impl)"))
    }
    /// 对齐 Java: `Range.hasNext()`
    #[allow(clippy::too_many_arguments)]
    pub fn hasNext() -> Result<bool> {
        Err(CoreError::PendingEngine("Range::hasNext (waiting for full impl)"))
    }
    /// 对齐 Java: `Range.next()`
    #[allow(clippy::too_many_arguments)]
    pub fn next() -> Result<T> {
        Err(CoreError::PendingEngine("Range::next (waiting for full impl)"))
    }
    /// 对齐 Java: `Range.remove()`
    #[allow(clippy::too_many_arguments)]
    pub fn remove() -> Result<()> {
        Err(CoreError::PendingEngine("Range::remove (waiting for full impl)"))
    }
    /// 对齐 Java: `Range.iterator()`
    #[allow(clippy::too_many_arguments)]
    pub fn iterator() -> Result<Iterator<T>> {
        Err(CoreError::PendingEngine("Range::iterator (waiting for full impl)"))
    }
    /// 对齐 Java: `Range.reset()`
    #[allow(clippy::too_many_arguments)]
    pub fn reset() -> Result<Range<T>> {
        Err(CoreError::PendingEngine("Range::reset (waiting for full impl)"))
    }
}
