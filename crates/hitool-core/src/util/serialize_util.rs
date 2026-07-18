//! 对齐: `cn.hutool.core.util.SerializeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/SerializeUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hitool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.SerializeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct SerializeUtil;

impl SerializeUtil {
    /// 对齐 Java: `cn.hutool.core.util::SerializeUtil::clone#T (T obj)`
    pub fn clone(obj: T) -> Result<T> {
        Err(CoreError::PendingEngine("clone"))
    }

    /// 对齐 Java: `cn.hutool.core.util::SerializeUtil::serialize#byte[] (T obj)`
    pub fn serialize(obj: T) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("serialize"))
    }

    /// 对齐 Java: `cn.hutool.core.util::SerializeUtil::deserialize#T (byte[] bytes, Class<?>... acceptClasses)`
    pub fn deserialize(bytes: Vec<i8>, acceptClasses: &[Class]) -> Result<T> {
        Err(CoreError::PendingEngine("deserialize"))
    }
}
