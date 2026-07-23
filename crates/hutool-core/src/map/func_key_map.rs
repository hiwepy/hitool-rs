//! 对齐: `cn.hutool.core.map.FuncKeyMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/FuncKeyMap.java

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java 类: `cn.hutool.core.map.FuncKeyMap`
///
/// 写入前对键应用变换函数。
#[derive(Debug, Clone)]
pub struct FuncKeyMap<K, V, F>
where
    F: Fn(&K) -> K,
{
    inner: HashMap<K, V>,
    key_func: F,
}

impl<K: Eq + Hash + Clone, V, F> FuncKeyMap<K, V, F>
where
    F: Fn(&K) -> K,
{
    /// 对齐 Java: `FuncKeyMap(Map, Function)`
    pub fn new(inner: HashMap<K, V>, key_func: F) -> Self {
        Self { inner, key_func }
    }

    /// 放入（键经变换）。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let mapped = (self.key_func)(&key);
        self.inner.insert(mapped, value)
    }

    /// 取值。
    pub fn get(&self, key: &K) -> Option<&V> {
        let mapped = (self.key_func)(key);
        self.inner.get(&mapped)
    }

    /// 底层 map。
    pub fn raw(&self) -> &HashMap<K, V> {
        &self.inner
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
