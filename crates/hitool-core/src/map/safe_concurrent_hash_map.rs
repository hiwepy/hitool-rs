//! 对齐: `cn.hutool.core.map.SafeConcurrentHashMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/SafeConcurrentHashMap.java

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// 对齐 Java 类: `cn.hutool.core.map.SafeConcurrentHashMap`
///
/// 基于 `Arc<Mutex<HashMap>>` 的线程安全 map（避免 Java null 键值问题）。
#[derive(Debug, Clone, Default)]
pub struct SafeConcurrentHashMap<K, V> {
    inner: Arc<Mutex<HashMap<K, V>>>,
}

impl<K: Eq + Hash, V> SafeConcurrentHashMap<K, V> {
    /// 无参构造。
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 指定容量。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::with_capacity(capacity))),
        }
    }

    /// 放入。
    pub fn put(&self, key: K, value: V) -> Option<V> {
        self.inner.lock().expect("lock").insert(key, value)
    }

    /// 取值（克隆）。
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.inner.lock().expect("lock").get(key).cloned()
    }

    /// 移除。
    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.lock().expect("lock").remove(key)
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.lock().expect("lock").len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.lock().expect("lock").is_empty()
    }

    /// 清空。
    pub fn clear(&self) {
        self.inner.lock().expect("lock").clear();
    }

    /// `computeIfAbsent` 语义。
    pub fn compute_if_absent<F>(&self, key: K, mapping: F) -> V
    where
        V: Clone,
        F: FnOnce(&K) -> V,
    {
        let mut guard = self.inner.lock().expect("lock");
        if let Some(v) = guard.get(&key) {
            return v.clone();
        }
        let v = mapping(&key);
        guard.insert(key, v.clone());
        v
    }
}
