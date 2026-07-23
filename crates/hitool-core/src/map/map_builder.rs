//! 对齐: `cn.hutool.core.map.MapBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapBuilder.java

use std::collections::HashMap;
use std::hash::Hash;

use crate::map_util::MapUtil;

/// 对齐 Java 类: `cn.hutool.core.map.MapBuilder`
#[derive(Debug, Clone)]
pub struct MapBuilder<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MapBuilder<K, V> {
    /// 对齐 Java: `MapBuilder.create()`
    pub fn create() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// 对齐 Java: `MapBuilder.create(boolean isLinked)` —— linked 路径仍用 HashMap（有序见 IndexMap 变体）。
    pub fn create_linked(_is_linked: bool) -> Self {
        Self::create()
    }

    /// 对齐 Java: `MapBuilder.create(Map)`
    pub fn create_from(map: HashMap<K, V>) -> Self {
        Self { map }
    }

    /// 对齐 Java: `new MapBuilder(Map)`
    pub fn new(map: HashMap<K, V>) -> Self {
        Self::create_from(map)
    }

    /// 对齐 Java: `MapBuilder.put(K, V)`
    pub fn put(mut self, k: K, v: V) -> Self {
        self.map.insert(k, v);
        self
    }

    /// 对齐 Java: `MapBuilder.put(boolean, K, V)`
    pub fn put_if(mut self, condition: bool, k: K, v: V) -> Self {
        if condition {
            self.map.insert(k, v);
        }
        self
    }

    /// 对齐 Java: `MapBuilder.put(boolean, K, Supplier)`
    pub fn put_if_with<F>(mut self, condition: bool, k: K, supplier: F) -> Self
    where
        F: FnOnce() -> V,
    {
        if condition {
            self.map.insert(k, supplier());
        }
        self
    }

    /// 对齐 Java: `MapBuilder.putAll(Map)`
    pub fn put_all(mut self, other: HashMap<K, V>) -> Self {
        self.map.extend(other);
        self
    }

    /// 对齐 Java: `MapBuilder.clear()`
    pub fn clear(mut self) -> Self {
        self.map.clear();
        self
    }

    /// 对齐 Java: `MapBuilder.map()`
    pub fn map(self) -> HashMap<K, V> {
        self.map
    }

    /// 对齐 Java: `MapBuilder.build()`
    pub fn build(self) -> HashMap<K, V> {
        self.map()
    }

    /// 对齐 Java: `MapBuilder.join(separator, keyValueSeparator)`
    pub fn join(&self, separator: &str, key_value_separator: &str) -> String
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        MapUtil::join(&self.map, separator, key_value_separator)
    }

    /// 对齐 Java: `MapBuilder.joinIgnoreNull`
    pub fn join_ignore_null(&self, separator: &str, key_value_separator: &str) -> String
    where
        K: std::fmt::Display,
        V: std::fmt::Display + Clone,
    {
        // 无 Option 包装时等价于 join
        MapUtil::join(&self.map, separator, key_value_separator)
    }

    /// 对齐 Java: `MapBuilder.join(..., boolean isIgnoreNull)`
    pub fn join_full(
        &self,
        separator: &str,
        key_value_separator: &str,
        is_ignore_null: bool,
    ) -> String
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        MapUtil::join_full(
            &self.map,
            separator,
            key_value_separator,
            is_ignore_null,
            &[],
        )
    }
}

impl<K: Eq + Hash, V> Default for MapBuilder<K, V> {
    fn default() -> Self {
        Self::create()
    }
}
