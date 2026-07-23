//! 对齐: `cn.hutool.core.map.CaseInsensitiveTreeMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/CaseInsensitiveTreeMap.java

use std::collections::BTreeMap;

/// 对齐 Java 类: `cn.hutool.core.map.CaseInsensitiveTreeMap`
///
/// 忽略大小写的有序 Map（BTreeMap + 小写键）。
#[derive(Debug, Clone, Default)]
pub struct CaseInsensitiveTreeMap<V> {
    inner: BTreeMap<String, V>,
}

impl<V> CaseInsensitiveTreeMap<V> {
    /// 对齐 Java: 默认构造
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
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
        self.inner.remove(&Self::normalize(key))
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

    /// 有序迭代。
    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.inner.iter()
    }
}
