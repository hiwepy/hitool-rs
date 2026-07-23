//! 对齐: `cn.hutool.core.map.CamelCaseMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/CamelCaseMap.java

use std::collections::HashMap;

/// 对齐 Java 类: `cn.hutool.core.map.CamelCaseMap`
///
/// 键自动转为驼峰风格（`int_value` ↔ `intValue`）。
#[derive(Debug, Clone, Default)]
pub struct CamelCaseMap<V> {
    inner: HashMap<String, V>,
}

impl<V> CamelCaseMap<V> {
    /// 对齐 Java: 默认构造
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 带容量构造。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    /// 对齐 Java: `CamelCaseMap(Map)`
    pub fn from_map(map: HashMap<String, V>) -> Self {
        let mut out = Self::with_capacity(map.len());
        for (k, v) in map {
            out.put(k, v);
        }
        out
    }

    /// 键转驼峰。
    pub fn to_camel_case(key: impl AsRef<str>) -> String {
        let name = key.as_ref();
        if !name.contains('_') {
            return name.to_string();
        }
        let mut sb = String::with_capacity(name.len());
        let mut upper = false;
        for c in name.chars() {
            if c == '_' {
                upper = true;
            } else if upper {
                for u in c.to_uppercase() {
                    sb.push(u);
                }
                upper = false;
            } else {
                for l in c.to_lowercase() {
                    sb.push(l);
                }
            }
        }
        sb
    }

    /// 放入（键转驼峰）。
    pub fn put(&mut self, key: impl AsRef<str>, value: V) -> Option<V> {
        self.inner.insert(Self::to_camel_case(key), value)
    }

    /// 获取。
    pub fn get(&self, key: impl AsRef<str>) -> Option<&V> {
        self.inner.get(&Self::to_camel_case(key))
    }

    /// 移除。
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<V> {
        self.inner.remove(&Self::to_camel_case(key))
    }

    /// 是否包含键。
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.inner.contains_key(&Self::to_camel_case(key))
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
    pub fn raw(&self) -> &HashMap<String, V> {
        &self.inner
    }
}
