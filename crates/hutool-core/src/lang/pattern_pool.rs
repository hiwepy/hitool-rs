//! 对齐: `cn.hutool.core.lang.PatternPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/PatternPool.java
//!
//! 编译正则缓存；flags 对齐 Java `Pattern` 位掩码的常用子集（CASE_INSENSITIVE=2）。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use regex::{Regex, RegexBuilder};

use super::regex_pool::RegexPool;

/// 对齐 Java: `Pattern.CASE_INSENSITIVE`
pub const FLAG_CASE_INSENSITIVE: i32 = 2;

/// 对齐 Java: `PatternPool.RegexWithFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexWithFlag {
    regex: String,
    flag: i32,
}

impl RegexWithFlag {
    /// 对齐 Java: `RegexWithFlag(String, int)`
    #[must_use]
    pub fn new(regex: impl Into<String>, flag: i32) -> Self {
        Self {
            regex: regex.into(),
            flag,
        }
    }
}

fn pool() -> &'static Mutex<HashMap<RegexWithFlag, Arc<Regex>>> {
    static POOL: std::sync::OnceLock<Mutex<HashMap<RegexWithFlag, Arc<Regex>>>> =
        std::sync::OnceLock::new();
    POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

fn compile(regex: &str, flags: i32) -> Result<Regex, regex::Error> {
    let mut builder = RegexBuilder::new(regex);
    if flags & FLAG_CASE_INSENSITIVE != 0 {
        builder.case_insensitive(true);
    }
    builder.build()
}

/// 对齐 Java: `cn.hutool.core.lang.PatternPool`
pub struct PatternPool;

impl PatternPool {
    /// 对齐 Java: `PatternPool.GENERAL` 等预编译访问（按需编译入池）。
    pub fn general() -> Arc<Regex> {
        Self::get(RegexPool::GENERAL)
    }

    /// 对齐 Java: `PatternPool.get(String)`
    pub fn get(regex: &str) -> Arc<Regex> {
        Self::get_with_flags(regex, 0)
    }

    /// 对齐 Java: `PatternPool.get(String, int)`
    pub fn get_with_flags(regex: &str, flags: i32) -> Arc<Regex> {
        let key = RegexWithFlag::new(regex, flags);
        let mut g = pool().lock();
        if let Some(existing) = g.get(&key) {
            return Arc::clone(existing);
        }
        let compiled = Arc::new(
            compile(regex, flags).unwrap_or_else(|_| Regex::new(r"(?!)").expect("never")),
        );
        g.insert(key, Arc::clone(&compiled));
        compiled
    }

    /// 对齐 Java: `PatternPool.remove(String, int)`
    pub fn remove(regex: &str, flags: i32) -> Option<Arc<Regex>> {
        pool().lock().remove(&RegexWithFlag::new(regex, flags))
    }

    /// 对齐 Java: `PatternPool.clear()`
    pub fn clear() {
        pool().lock().clear();
    }
}

#[cfg(test)]
mod pattern_pool_idiomatic_parity {
    use super::*;

    #[test]
    fn pattern_pool_get_remove_clear_and_flag_key() {
        PatternPool::clear();
        let a = PatternPool::get(r"^\d+$");
        let b = PatternPool::get(r"^\d+$");
        assert!(Arc::ptr_eq(&a, &b));
        assert!(a.is_match("12"));
        let c = PatternPool::get_with_flags("abc", FLAG_CASE_INSENSITIVE);
        assert!(c.is_match("ABC"));
        assert!(PatternPool::remove(r"^\d+$", 0).is_some());
        PatternPool::clear();
        let k1 = RegexWithFlag::new("x", 0);
        let k2 = RegexWithFlag::new("x", 0);
        assert_eq!(k1, k2);
    }
}
