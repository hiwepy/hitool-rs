//! 对齐: `cn.hutool.core.map.AbsEntry`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/AbsEntry.java

/// 对齐 Java 类: `cn.hutool.core.map.AbsEntry`
///
/// 可变键值对条目。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbsEntry<K, V> {
    /// 键
    pub key: K,
    /// 值
    pub value: V,
}

impl<K, V> AbsEntry<K, V> {
    /// 创建条目。
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    /// 对齐 Java: `getKey`
    pub fn get_key(&self) -> &K {
        &self.key
    }

    /// 对齐 Java: `getValue`
    pub fn get_value(&self) -> &V {
        &self.value
    }

    /// 对齐 Java: `setValue`
    pub fn set_value(&mut self, value: V) -> V {
        std::mem::replace(&mut self.value, value)
    }
}

impl<K: std::fmt::Display, V: std::fmt::Display> std::fmt::Display for AbsEntry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
