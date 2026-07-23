//! 对齐: `cn.hutool.core.map.BiMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/BiMap.java

use std::collections::HashMap;
use std::hash::Hash;

use crate::map::map_wrapper::MapWrapper;
use crate::map_util::MapUtil;

/// 对齐 Java 类: `cn.hutool.core.map.BiMap`
///
/// 双向 Map：维护正向与反向索引，重复值以后写入者为准。
#[derive(Debug, Clone, Default)]
pub struct BiMap<K, V> {
    forward: MapWrapper<K, V>,
    inverse: Option<HashMap<V, K>>,
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone> BiMap<K, V> {
    /// 对齐 Java: `BiMap(Map)`
    pub fn new(raw: HashMap<K, V>) -> Self {
        Self {
            forward: MapWrapper::new(raw),
            inverse: None,
        }
    }

    /// 空 BiMap。
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    /// 对齐 Java: `put`
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let old = self.forward.put(key.clone(), value.clone());
        if let Some(ref mut inv) = self.inverse {
            if let Some(ref old_v) = old {
                inv.remove(old_v);
            }
            inv.insert(value, key);
        }
        old
    }

    /// 对齐 Java: `putAll`
    pub fn put_all(&mut self, other: HashMap<K, V>) {
        for (k, v) in other {
            self.put(k, v);
        }
    }

    /// 对齐 Java: `remove`
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let v = self.forward.remove(key);
        if let (Some(inv), Some(val)) = (&mut self.inverse, &v) {
            inv.remove(val);
        }
        v
    }

    /// 对齐 Java: `remove(K, V)`
    pub fn remove_entry(&mut self, key: &K, value: &V) -> bool {
        if self.forward.remove_entry(key, value) {
            if let Some(inv) = &mut self.inverse {
                inv.remove(value);
            }
            true
        } else {
            false
        }
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.forward.clear();
        self.inverse = None;
    }

    /// 对齐 Java: `getInverse`
    pub fn get_inverse(&mut self) -> &HashMap<V, K> {
        if self.inverse.is_none() {
            self.inverse = Some(MapUtil::inverse(self.forward.raw()));
        }
        self.inverse.as_ref().expect("inverse just initialized")
    }

    /// 对齐 Java: `getKey(V)`
    pub fn get_key(&mut self, value: &V) -> Option<&K> {
        self.get_inverse().get(value)
    }

    /// 正向 get。
    pub fn get(&self, key: &K) -> Option<&V> {
        self.forward.get(key)
    }

    /// 对齐 Java: `putIfAbsent`
    pub fn put_if_absent(&mut self, key: K, value: V) -> Option<&V> {
        if self.forward.contains_key(&key) {
            return self.forward.get(&key);
        }
        self.put(key, value);
        None
    }

    /// 对齐 Java: `computeIfAbsent`
    pub fn compute_if_absent<F>(&mut self, key: K, f: F) -> &V
    where
        F: FnOnce(&K) -> V,
    {
        if !self.forward.contains_key(&key) {
            let v = f(&key);
            self.put(key.clone(), v);
        }
        self.reset_inverse_map();
        self.forward.get(&key).expect("just inserted")
    }

    /// 对齐 Java: `computeIfPresent`
    pub fn compute_if_present<F>(&mut self, key: &K, f: F) -> Option<&V>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        let _ = self.forward.compute_if_present(key, f);
        self.reset_inverse_map();
        self.forward.get(key)
    }

    /// 对齐 Java: `compute`
    pub fn compute<F>(&mut self, key: K, f: F) -> Option<&V>
    where
        F: FnOnce(&K, Option<V>) -> Option<V>,
    {
        let _ = self.forward.compute(key.clone(), f);
        self.reset_inverse_map();
        self.forward.get(&key)
    }

    /// 对齐 Java: `merge`
    pub fn merge<F>(&mut self, key: K, value: V, remapping: F) -> &V
    where
        F: FnOnce(V, V) -> V,
    {
        let _ = self.forward.merge(key.clone(), value, remapping);
        self.reset_inverse_map();
        self.forward.get(&key).expect("merged")
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.forward.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }

    /// 内层正向 Map。
    pub fn raw(&self) -> &HashMap<K, V> {
        self.forward.raw()
    }

    fn reset_inverse_map(&mut self) {
        self.inverse = None;
    }
}
