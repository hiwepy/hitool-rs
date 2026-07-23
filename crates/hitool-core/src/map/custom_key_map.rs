//! 对齐: `cn.hutool.core.map.CustomKeyMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/CustomKeyMap.java

use std::collections::HashMap;
use std::hash::Hash;

use super::FuncKeyMap;

/// 对齐 Java 类: `cn.hutool.core.map.CustomKeyMap`
///
/// 抽象自定义键变换；Rust 用闭包 `FuncKeyMap` 表达。
pub type CustomKeyMap<K, V, F> = FuncKeyMap<K, V, F>;

/// 便捷构造：自定义键函数的 map。
pub fn custom_key_map<K, V, F>(key_func: F) -> CustomKeyMap<K, V, F>
where
    K: Eq + Hash + Clone,
    F: Fn(&K) -> K,
{
    FuncKeyMap::new(HashMap::new(), key_func)
}
