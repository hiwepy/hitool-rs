//! 对齐: `cn.hutool.core.map.TransMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/TransMap.java

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java 类: `cn.hutool.core.map.TransMap`
///
/// 对键与值分别应用变换后再存入。
#[derive(Debug, Clone)]
pub struct TransMap<K, V, FK, FV>
where
    FK: Fn(K) -> K,
    FV: Fn(V) -> V,
{
    inner: HashMap<K, V>,
    key_trans: FK,
    value_trans: FV,
}

impl<K: Eq + Hash, V, FK, FV> TransMap<K, V, FK, FV>
where
    FK: Fn(K) -> K,
    FV: Fn(V) -> V,
{
    /// 构造变换 map。
    pub fn new(inner: HashMap<K, V>, key_trans: FK, value_trans: FV) -> Self {
        Self {
            inner,
            key_trans,
            value_trans,
        }
    }

    /// 对齐 Java: `put` —— 先变换再写入。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let k = (self.key_trans)(key);
        let v = (self.value_trans)(value);
        self.inner.insert(k, v)
    }

    /// 取值。
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Clone,
    {
        // 查询侧也尝试变换键（对齐 TransMap customKey）
        let mapped = (self.key_trans)(key.clone());
        self.inner.get(&mapped)
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 底层 map。
    pub fn raw(&self) -> &HashMap<K, V> {
        &self.inner
    }
}
