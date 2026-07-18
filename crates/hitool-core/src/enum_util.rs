//! 对齐: `cn.hutool.core.util.EnumUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/EnumUtil.java
//!
//! Rust 版本提供枚举操作的 idiomatic 实现。

use std::collections::HashMap;

/// 对齐 Java: `cn.hutool.core.util.EnumUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct EnumUtil;

impl EnumUtil {
    // ── 枚举名称 ──

    /// 对齐 Java: `EnumUtil.getName(Enum)`
    pub fn name_of<E: std::fmt::Debug>(value: &E) -> String {
        format!("{:?}", value)
    }

    // ── 枚举查找 ──

    /// 对齐 Java: `EnumUtil.fromString(Class, String)`
    pub fn from_names<E: std::fmt::Debug + Clone>(
        variants: &[E],
        name: &str,
    ) -> Option<E> {
        variants.iter().find(|v| format!("{:?}", v) == name).cloned()
    }

    // ── 枚举列表 ──

    /// 对齐 Java: `EnumUtil.getNames(Class)`
    pub fn names<E: std::fmt::Debug>(variants: &[E]) -> Vec<String> {
        variants.iter().map(|v| format!("{:?}", v)).collect()
    }

    /// 对齐 Java: `EnumUtil.getValues(Class)`
    pub fn count<E>(variants: &[E]) -> usize {
        variants.len()
    }

    // ── 枚举映射 ──

    /// 创建名称到枚举值的映射
    pub fn name_map<E: std::fmt::Debug + Clone>(variants: &[E]) -> HashMap<String, E> {
        variants
            .iter()
            .map(|v| (format!("{:?}", v), v.clone()))
            .collect()
    }

    // ── 枚举验证 ──

    /// 对齐 Java: `EnumUtil.contains(Class, String)`
    pub fn contains_name<E: std::fmt::Debug>(variants: &[E], name: &str) -> bool {
        variants.iter().any(|v| format!("{:?}", v) == name)
    }
}
