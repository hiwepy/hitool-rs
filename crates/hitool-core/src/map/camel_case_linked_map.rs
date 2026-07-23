//! 对齐: `cn.hutool.core.map.CamelCaseLinkedMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/CamelCaseLinkedMap.java

use indexmap::IndexMap;

use crate::map::camel_case_map::CamelCaseMap;

/// 对齐 Java 类: `cn.hutool.core.map.CamelCaseLinkedMap`
#[derive(Debug, Clone, Default)]
pub struct CamelCaseLinkedMap<V> {
    inner: IndexMap<String, V>,
}

impl<V> CamelCaseLinkedMap<V> {
    /// 默认构造。
    pub fn new() -> Self {
        Self {
            inner: IndexMap::new(),
        }
    }

    /// 放入。
    pub fn put(&mut self, key: impl AsRef<str>, value: V) -> Option<V> {
        self.inner
            .insert(CamelCaseMap::<V>::to_camel_case(key), value)
    }

    /// 获取。
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V> {
        self.inner.get(&CamelCaseMap::<V>::to_camel_case(key))
    }

    /// 移除。
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<V> {
        self.inner
            .shift_remove(&CamelCaseMap::<V>::to_camel_case(key))
    }

    /// 是否包含键。
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner
            .contains_key(&CamelCaseMap::<V>::to_camel_case(key))
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
