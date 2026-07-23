//! 对齐: `cn.hutool.core.map.CaseInsensitiveMap`

use std::collections::HashMap;

/// 对齐 Java 类: `cn.hutool.core.map.CaseInsensitiveMap`
#[derive(Debug, Clone, Default)]
pub struct CaseInsensitiveMap<V> {
    inner: HashMap<String, V>,
}

impl<V> CaseInsensitiveMap<V> {
    /// 无参构造。
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 指定容量。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    /// 从已有 map 复制并归一化键。
    pub fn from_map(map: HashMap<String, V>) -> Self {
        let mut inner = HashMap::with_capacity(map.len());
        for (k, v) in map {
            inner.insert(k.to_lowercase(), v);
        }
        Self { inner }
    }

    /// 放入。
    pub fn put(&mut self, key: impl AsRef<str>, value: V) -> Option<V> {
        self.inner.insert(key.as_ref().to_lowercase(), value)
    }

    /// 取值。
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V> {
        self.inner.get(&key.as_ref().to_lowercase())
    }

    /// 移除。
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<V> {
        self.inner.remove(&key.as_ref().to_lowercase())
    }

    /// 是否包含键。
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(&key.as_ref().to_lowercase())
    }

    /// 批量放入。
    pub fn put_all(&mut self, map: HashMap<String, V>) {
        for (k, v) in map {
            self.put(k, v);
        }
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
    pub fn raw(&self) -> &HashMap<String, V> {
        &self.inner
    }
}
