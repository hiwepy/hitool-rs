//! 对齐: `cn.hutool.core.map.TolerantMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/TolerantMap.java

use std::collections::HashMap;
use std::hash::Hash;

use crate::map::map_wrapper::MapWrapper;

/// 对齐 Java 类: `cn.hutool.core.map.TolerantMap`
///
/// 缺失键时返回默认值。
#[derive(Debug, Clone)]
pub struct TolerantMap<K, V> {
    inner: MapWrapper<K, V>,
    default_value: V,
}

impl<K: Eq + Hash + Clone, V: Clone> TolerantMap<K, V> {
    /// 对齐 Java: `TolerantMap(V)`
    pub fn new(default_value: V) -> Self {
        Self {
            inner: MapWrapper::new(HashMap::new()),
            default_value,
        }
    }

    /// 对齐 Java: `TolerantMap(Map, V)` / `of`
    pub fn of(map: HashMap<K, V>, default_value: V) -> Self {
        Self {
            inner: MapWrapper::new(map),
            default_value,
        }
    }

    /// 对齐 Java: `TolerantMap(int, V)`
    pub fn with_capacity(capacity: usize, default_value: V) -> Self {
        Self {
            inner: MapWrapper::new(HashMap::with_capacity(capacity)),
            default_value,
        }
    }

    /// 对齐 Java: `get` —— 缺失返回默认值的克隆。
    pub fn get(&self, key: &K) -> V {
        self.inner
            .get(key)
            .cloned()
            .unwrap_or_else(|| self.default_value.clone())
    }

    /// 可选引用（不含默认）。
    pub fn get_opt(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    /// 放入。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.inner.put(key, value)
    }

    /// 移除。
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    /// 默认值。
    pub fn default_value(&self) -> &V {
        &self.default_value
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 内层。
    pub fn raw(&self) -> &HashMap<K, V> {
        self.inner.raw()
    }
}
