//! 对齐: `cn.hutool.core.map.CaseInsensitiveLinkedMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/CaseInsensitiveLinkedMap.java

use indexmap::IndexMap;

/// 对齐 Java 类: `cn.hutool.core.map.CaseInsensitiveLinkedMap`
///
/// 忽略大小写且保持插入顺序（IndexMap）。
#[derive(Debug, Clone, Default)]
pub struct CaseInsensitiveLinkedMap<V> {
    inner: IndexMap<String, V>,
}

impl<V> CaseInsensitiveLinkedMap<V> {
    /// 对齐 Java: 默认构造
    pub fn new() -> Self {
        Self {
            inner: IndexMap::new(),
        }
    }

    /// 带容量构造。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(capacity),
        }
    }

    fn normalize(key: impl AsRef<str>) -> String {
        key.as_ref().to_lowercase()
    }

    /// 放入。
    pub fn put(&mut self, key: impl AsRef<str>, value: V) -> Option<V> {
        self.inner.insert(Self::normalize(key), value)
    }

    /// 获取。
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V> {
        self.inner.get(&Self::normalize(key))
    }

    /// 移除。
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<V> {
        self.inner.shift_remove(&Self::normalize(key))
    }

    /// 是否包含键。
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(&Self::normalize(key))
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 按插入顺序迭代。
    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.inner.iter()
    }
}
