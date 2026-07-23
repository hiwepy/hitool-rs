//! 对齐: `cn.hutool.core.map.FuncMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/FuncMap.java

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java 类: `cn.hutool.core.map.FuncMap`
///
/// 空值时通过工厂函数生成默认值。
#[derive(Debug, Clone)]
pub struct FuncMap<K, V, F>
where
    F: Fn(&K) -> V,
{
    inner: HashMap<K, V>,
    default_func: F,
}

impl<K: Eq + Hash + Clone, V: Clone, F> FuncMap<K, V, F>
where
    F: Fn(&K) -> V,
{
    /// 对齐 Java: `FuncMap(Map, Function)`
    pub fn new(inner: HashMap<K, V>, default_func: F) -> Self {
        Self {
            inner,
            default_func,
        }
    }

    /// 对齐 Java: `get` —— 缺失时用工厂生成并缓存。
    pub fn get(&mut self, key: &K) -> &V {
        if !self.inner.contains_key(key) {
            let v = (self.default_func)(key);
            self.inner.insert(key.clone(), v);
        }
        self.inner.get(key).expect("inserted")
    }

    /// 放入。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
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
