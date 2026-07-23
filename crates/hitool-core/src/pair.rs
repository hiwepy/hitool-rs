//! 对齐: `cn.hutool.core.lang.Pair`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Pair.java

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

/// 对齐 Java: `cn.hutool.core.lang.Pair<K, V>`
#[derive(Clone, Debug, Default)]
pub struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> Pair<K, V> {
    /// 对齐 Java: `Pair.of(K, V)`
    pub fn of(key: K, value: V) -> Self {
        Self { key, value }
    }

    /// 对齐 Java: `Pair.getKey()`
    pub fn get_key(&self) -> &K {
        &self.key
    }

    /// 对齐 Java: `Pair.getValue()`
    pub fn get_value(&self) -> &V {
        &self.value
    }

    /// 拆成元组。
    pub fn into_tuple(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for Pair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl<K: Eq, V: Eq> Eq for Pair<K, V> {}

impl<K: Hash, V: Hash> Hash for Pair<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.value.hash(state);
    }
}

impl<K: Display, V: Display> Display for Pair<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pair [key={}, value={}]", self.key, self.value)
    }
}
