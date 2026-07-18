//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。

use std::collections::HashMap;

/// 对齐 Java: `cn.hutool.core.map.MapUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct MapUtil;

impl MapUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `MapUtil.isEmpty(Map)`
    pub fn is_empty<K, V>(map: &HashMap<K, V>) -> bool {
        map.is_empty()
    }

    /// 对齐 Java: `MapUtil.isNotEmpty(Map)`
    pub fn is_not_empty<K, V>(map: &HashMap<K, V>) -> bool {
        !map.is_empty()
    }

    // ── 创建操作 ──

    /// 对齐 Java: `MapUtil.of(Object[]...)`
    pub fn of<K, V>(pairs: &[(K, V)]) -> HashMap<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone,
    {
        pairs
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// 对齐 Java: `MapUtil.newHashMap()`
    pub fn new_hash_map<K, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    // ── 获取操作 ──

    /// 对齐 Java: `MapUtil.getStr(Map, Object)`
    pub fn get_str<'a, K: Eq + std::hash::Hash>(map: &'a HashMap<K, String>, key: &K) -> Option<&'a str> {
        map.get(key).map(|s| s.as_str())
    }

    /// 对齐 Java: `MapUtil.getInt(Map, Object)`
    pub fn get_int<K: Eq + std::hash::Hash>(map: &HashMap<K, i64>, key: &K) -> Option<i64> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getBool(Map, Object)`
    pub fn get_bool<K: Eq + std::hash::Hash>(map: &HashMap<K, bool>, key: &K) -> Option<bool> {
        map.get(key).copied()
    }

    // ── 设置操作 ──

    /// 对齐 Java: `MapUtil.putAll(Map, Map)`
    pub fn put_all<K, V>(target: &mut HashMap<K, V>, source: HashMap<K, V>)
    where
        K: Eq + std::hash::Hash,
    {
        target.extend(source);
    }

    // ── 转换操作 ──

    /// 对齐 Java: `MapUtil.join(Map, CharSequence, CharSequence)`
    pub fn join<K: std::fmt::Display, V: std::fmt::Display>(
        map: &HashMap<K, V>,
        entry_delimiter: &str,
        kv_delimiter: &str,
    ) -> String {
        map.iter()
            .map(|(k, v)| format!("{}{}{}", k, kv_delimiter, v))
            .collect::<Vec<_>>()
            .join(entry_delimiter)
    }

    // ── 过滤操作 ──

    /// 对齐 Java: `MapUtil.filter(Map, Predicate)`
    pub fn filter<K, V, F>(map: &HashMap<K, V>, predicate: F) -> HashMap<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone,
        F: Fn(&K, &V) -> bool,
    {
        map.iter()
            .filter(|(k, v)| predicate(k, v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    // ── 合并操作 ──

    /// 对齐 Java: `MapUtil.merge(Map, Map)`
    pub fn merge<K, V>(mut left: HashMap<K, V>, right: HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        left.extend(right);
        left
    }

    // ── 键值操作 ──

    /// 对齐 Java: `MapUtil.keys(Map)`
    pub fn keys<K: Clone, V>(map: &HashMap<K, V>) -> Vec<K> {
        map.keys().cloned().collect()
    }

    /// 对齐 Java: `MapUtil.values(Map)`
    pub fn values<K, V: Clone>(map: &HashMap<K, V>) -> Vec<V> {
        map.values().cloned().collect()
    }

    // ── 反转操作 ──

    /// 对齐 Java: `MapUtil.inverse(Map)`
    pub fn inverse<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Eq + std::hash::Hash + Clone,
    {
        map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
    }
}
