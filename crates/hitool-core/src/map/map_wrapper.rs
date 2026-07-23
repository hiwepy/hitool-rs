//! 对齐: `cn.hutool.core.map.MapWrapper`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapWrapper.java

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java 类: `cn.hutool.core.map.MapWrapper`
///
/// 对内层 `HashMap` 的薄包装，提供 Hutool 同名委托方法。
#[derive(Debug, Clone, Default)]
pub struct MapWrapper<K, V> {
    raw: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MapWrapper<K, V> {
    /// 对齐 Java: `MapWrapper(Map)`
    pub fn new(raw: HashMap<K, V>) -> Self {
        Self { raw }
    }

    /// 对齐 Java: `MapWrapper(Supplier)` —— 由工厂创建空 Map。
    pub fn from_factory<F>(factory: F) -> Self
    where
        F: FnOnce() -> HashMap<K, V>,
    {
        Self { raw: factory() }
    }

    /// 对齐 Java: `getRaw()`
    pub fn raw(&self) -> &HashMap<K, V> {
        &self.raw
    }

    /// 可变内层引用。
    pub fn raw_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.raw
    }

    /// 取出内层 Map。
    pub fn into_raw(self) -> HashMap<K, V> {
        self.raw
    }

    /// 对齐 Java: `size()`
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// 对齐 Java: `isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// 对齐 Java: `containsKey`
    pub fn contains_key(&self, key: &K) -> bool {
        self.raw.contains_key(key)
    }

    /// 对齐 Java: `containsValue`
    pub fn contains_value(&self, value: &V) -> bool
    where
        V: PartialEq,
    {
        self.raw.values().any(|v| v == value)
    }

    /// 对齐 Java: `get`
    pub fn get(&self, key: &K) -> Option<&V> {
        self.raw.get(key)
    }

    /// 对齐 Java: `put`
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.raw.insert(key, value)
    }

    /// 对齐 Java: `remove`
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.raw.remove(key)
    }

    /// 对齐 Java: `putAll`
    pub fn put_all(&mut self, other: HashMap<K, V>) {
        self.raw.extend(other);
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.raw.clear();
    }

    /// 对齐 Java: `keySet`
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.raw.keys()
    }

    /// 对齐 Java: `values`
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.raw.values()
    }

    /// 对齐 Java: `entrySet` / `iterator`
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.raw.iter()
    }

    /// 对齐 Java: `putIfAbsent`
    pub fn put_if_absent(&mut self, key: K, value: V) -> Option<&V> {
        use std::collections::hash_map::Entry;
        match self.raw.entry(key) {
            Entry::Occupied(e) => Some(e.into_mut()),
            Entry::Vacant(e) => {
                e.insert(value);
                None
            }
        }
    }

    /// 对齐 Java: `getOrDefault`
    pub fn get_or_default<'a>(&'a self, key: &K, default: &'a V) -> &'a V {
        self.raw.get(key).unwrap_or(default)
    }

    /// 对齐 Java: `computeIfAbsent`
    pub fn compute_if_absent<F>(&mut self, key: K, f: F) -> &V
    where
        F: FnOnce(&K) -> V,
    {
        self.raw.entry(key).or_insert_with_key(f)
    }

    /// 对齐 Java: `computeIfPresent`
    pub fn compute_if_present<F>(&mut self, key: &K, f: F) -> Option<&V>
    where
        F: FnOnce(&K, V) -> Option<V>,
        K: Clone,
    {
        if let Some(old) = self.raw.remove(key) {
            if let Some(new_v) = f(key, old) {
                self.raw.insert(key.clone(), new_v);
                return self.raw.get(key);
            }
        }
        None
    }

    /// 对齐 Java: `compute`
    pub fn compute<F>(&mut self, key: K, f: F) -> Option<&V>
    where
        F: FnOnce(&K, Option<V>) -> Option<V>,
        K: Clone,
    {
        let old = self.raw.remove(&key);
        if let Some(new_v) = f(&key, old) {
            self.raw.insert(key.clone(), new_v);
            self.raw.get(&key)
        } else {
            None
        }
    }

    /// 对齐 Java: `merge`
    pub fn merge<F>(&mut self, key: K, value: V, remapping: F) -> &V
    where
        F: FnOnce(V, V) -> V,
    {
        match self.raw.remove(&key) {
            Some(old) => {
                let merged = remapping(old, value);
                self.raw.entry(key).or_insert(merged)
            }
            None => self.raw.entry(key).or_insert(value),
        }
    }

    /// 对齐 Java: `replace(K, V)`
    pub fn replace(&mut self, key: &K, value: V) -> Option<V>
    where
        K: Clone,
    {
        if self.raw.contains_key(key) {
            self.raw.insert(key.clone(), value)
        } else {
            None
        }
    }

    /// 对齐 Java: `replace(K, V, V)`
    pub fn replace_old(&mut self, key: &K, old_value: &V, new_value: V) -> bool
    where
        K: Clone,
        V: PartialEq,
    {
        match self.raw.get(key) {
            Some(v) if v == old_value => {
                self.raw.insert(key.clone(), new_value);
                true
            }
            _ => false,
        }
    }

    /// 对齐 Java: `remove(K, V)`
    pub fn remove_entry(&mut self, key: &K, value: &V) -> bool
    where
        V: PartialEq,
    {
        match self.raw.get(key) {
            Some(v) if v == value => {
                self.raw.remove(key);
                true
            }
            _ => false,
        }
    }

    /// 对齐 Java: `forEach`
    pub fn for_each<F>(&self, mut action: F)
    where
        F: FnMut(&K, &V),
    {
        for (k, v) in &self.raw {
            action(k, v);
        }
    }

    /// 对齐 Java: `replaceAll`
    pub fn replace_all<F>(&mut self, mut function: F)
    where
        F: FnMut(&K, V) -> V,
        K: Clone,
    {
        let keys: Vec<K> = self.raw.keys().cloned().collect();
        for k in keys {
            if let Some(old) = self.raw.remove(&k) {
                let new_v = function(&k, old);
                self.raw.insert(k, new_v);
            }
        }
    }
}
