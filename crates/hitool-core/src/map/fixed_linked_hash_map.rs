//! 对齐: `cn.hutool.core.map.FixedLinkedHashMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/FixedLinkedHashMap.java

use indexmap::IndexMap;
use std::hash::Hash;

/// 对齐 Java 类: `cn.hutool.core.map.FixedLinkedHashMap`
///
/// 固定容量的 LRU 风格有序 Map：超出容量时移除最久未访问条目。
#[derive(Debug, Clone)]
pub struct FixedLinkedHashMap<K, V> {
    inner: IndexMap<K, V>,
    capacity: usize,
    remove_listener: Option<fn(&K, &V)>,
}

impl<K: Eq + Hash + Clone, V: Clone> FixedLinkedHashMap<K, V> {
    /// 对齐 Java: `FixedLinkedHashMap(int capacity)`
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(capacity.saturating_add(1)),
            capacity,
            remove_listener: None,
        }
    }

    /// 对齐 Java: `getCapacity`
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 对齐 Java: `setCapacity`
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
        self.evict();
    }

    /// 对齐 Java: `setRemoveListener`
    pub fn set_remove_listener(&mut self, listener: fn(&K, &V)) {
        self.remove_listener = Some(listener);
    }

    /// 对齐 Java: `put` —— access-order：命中后移到末尾。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let old = self.inner.insert(key, value);
        self.evict();
        old
    }

    /// 对齐 Java: `get` —— 访问后移到末尾（LRU）。
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let idx = self.inner.get_index_of(key)?;
        // move to end
        if let Some((k, v)) = self.inner.shift_remove_index(idx) {
            self.inner.insert(k, v);
        }
        self.inner.get(key)
    }

    /// 不调整顺序的只读获取。
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    /// 移除。
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.shift_remove(key)
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn evict(&mut self) {
        while self.inner.len() > self.capacity {
            if let Some((k, v)) = self.inner.shift_remove_index(0) {
                if let Some(listener) = self.remove_listener {
                    listener(&k, &v);
                }
            } else {
                break;
            }
        }
    }
}
