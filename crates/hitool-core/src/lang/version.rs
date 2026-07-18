//! 对齐: `cn.hutool.core.lang.Version`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Version.java
//!
//! Hutool 的 `Version` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.Version`
#[derive(Debug, Clone, Default)]
pub struct Version;

impl Version {
    /// 对齐 Java: `Version.of(final String v)`
    #[allow(clippy::too_many_arguments)]
    pub fn of(&str v) -> Result<Version> {
        Err(CoreError::PendingEngine("Version::of (waiting for full impl)"))
    }
    /// 对齐 Java: `Version.compareTo(final Version that)`
    #[allow(clippy::too_many_arguments)]
    pub fn compareTo(Version that) -> Result<i32> {
        Err(CoreError::PendingEngine("Version::compareTo (waiting for full impl)"))
    }
    /// 对齐 Java: `Version.equals(final Object ob)`
    #[allow(clippy::too_many_arguments)]
    pub fn equals(Object ob) -> Result<bool> {
        Err(CoreError::PendingEngine("Version::equals (waiting for full impl)"))
    }
}
