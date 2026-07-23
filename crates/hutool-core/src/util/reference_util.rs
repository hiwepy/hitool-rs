//! 对齐: `cn.hutool.core.util.ReferenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReferenceUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hutool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ReferenceUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceUtil;

impl ReferenceUtil {
    /// 对齐 Java: `cn.hutool.core.util::ReferenceUtil::create#Reference<T> (ReferenceType type, T referent)`
    pub fn create(_type: *const (), referent: T) -> Result<()> {
        Err(CoreError::PendingEngine("create"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ReferenceUtil::create#Reference<T> (ReferenceType type, T referent, ReferenceQueue<T> queue)`
    pub fn create_2(_type: *const (), referent: T, queue: ReferenceQueue) -> Result<()> {
        Err(CoreError::PendingEngine("create"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ReferenceUtil::get#T (final Reference<T> obj)`
    pub fn get(obj: Reference) -> Result<T> {
        Err(CoreError::PendingEngine("get"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ReferenceUtil::get#T (final Ref<T> obj)`
    pub fn get_2(obj: Ref) -> Result<T> {
        Err(CoreError::PendingEngine("get"))
    }
}
