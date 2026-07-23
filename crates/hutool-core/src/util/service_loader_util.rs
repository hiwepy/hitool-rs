//! 对齐: `cn.hutool.core.util.ServiceLoaderUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ServiceLoaderUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hutool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ServiceLoaderUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ServiceLoaderUtil;

impl ServiceLoaderUtil {
    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::loadFirstAvailable#T (Class<T> clazz)`
    pub fn loadFirstAvailable(clazz: Class) -> Result<T> {
        Err(CoreError::PendingEngine("loadFirstAvailable"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::loadFirst#T (Class<T> clazz)`
    pub fn loadFirst(clazz: Class) -> Result<T> {
        Err(CoreError::PendingEngine("loadFirst"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::load#ServiceLoader<T> (Class<T> clazz)`
    pub fn load(clazz: Class) -> Result<()> {
        Err(CoreError::PendingEngine("load"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::load#ServiceLoader<T> (Class<T> clazz, ClassLoader loader)`
    pub fn load_2(clazz: Class, _loader: *const ()) -> Result<()> {
        Err(CoreError::PendingEngine("load"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::loadList#List<T> (Class<T> clazz)`
    pub fn loadList(clazz: Class) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("loadList"))
    }

    /// 对齐 Java: `cn.hutool.core.util::ServiceLoaderUtil::loadList#List<T> (Class<T> clazz, ClassLoader loader)`
    pub fn loadList_2(clazz: Class, _loader: *const ()) -> Result<Vec<T>> {
        Err(CoreError::PendingEngine("loadList"))
    }
}
