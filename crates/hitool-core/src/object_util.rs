//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;

/// 对齐 Java: `cn.hutool.core.util.ObjectUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectUtil;

impl ObjectUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `ObjectUtil.isNull(Object)`
    pub fn is_null<T>(value: Option<&T>) -> bool {
        value.is_none()
    }

    /// 对齐 Java: `ObjectUtil.isNotNull(Object)`
    pub fn is_not_null<T>(value: Option<&T>) -> bool {
        value.is_some()
    }

    // ── 默认值 ──

    /// 对齐 Java: `ObjectUtil.defaultIfNull(Object, Object)`
    pub fn default_if_null<T: Clone>(value: Option<&T>, default: &T) -> T {
        value.cloned().unwrap_or_else(|| default.clone())
    }

    // ── 相等判断 ──

    /// 对齐 Java: `ObjectUtil.equal(Object, Object)`
    pub fn equal<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.notEqual(Object, Object)`
    pub fn not_equal<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        !Self::equal(a, b)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `ObjectUtil.compare(Object, Object)`
    pub fn compare<T: Ord>(a: Option<&T>, b: Option<&T>) -> i32 {
        match (a, b) {
            (Some(a), Some(b)) => a.cmp(b) as i32,
            (None, Some(_)) => -1,
            (Some(_), None) => 1,
            (None, None) => 0,
        }
    }

    // ── 类型判断 ──

    /// 对齐 Java: `ObjectUtil.isBasicType(Object)`
    pub fn is_basic_type(value: &dyn Any) -> bool {
        value.is::<i32>()
            || value.is::<i64>()
            || value.is::<f32>()
            || value.is::<f64>()
            || value.is::<bool>()
            || value.is::<char>()
            || value.is::<i8>()
            || value.is::<i16>()
            || value.is::<u8>()
            || value.is::<u16>()
            || value.is::<u32>()
            || value.is::<u64>()
    }

    // ── 克隆操作 ──

    /// 对齐 Java: `ObjectUtil.clone(Object)`
    pub fn clone_if_some<T: Clone>(value: Option<&T>) -> Option<T> {
        value.cloned()
    }

    // ── 序列化辅助 ──

    /// 对齐 Java: `ObjectUtil.toString(Object)`
    pub fn to_string<T: std::fmt::Display>(value: Option<&T>) -> String {
        value.map_or_else(|| "null".to_string(), |v| v.to_string())
    }

    // ── 集合判断 ──

    /// 对齐 Java: `ObjectUtil.isEmpty(Object)`
    pub fn is_empty_str(value: Option<&str>) -> bool {
        value.map_or(true, |s| s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.isNotEmpty(Object)`
    pub fn is_not_empty_str(value: Option<&str>) -> bool {
        !Self::is_empty_str(value)
    }
}
