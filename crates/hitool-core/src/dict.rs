//! 对齐: `cn.hutool.core.lang.Dict`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Dict.java
//!
//! Rust 版本提供字典操作的 idiomatic 实现。

use std::collections::HashMap;

/// 对齐 Java: `cn.hutool.core.lang.Dict`
pub type Dict = HashMap<String, serde_json::Value>;

/// 对齐 Java: `cn.hutool.core.lang.Dict` 工具方法
#[derive(Debug, Clone, Copy, Default)]
pub struct DictUtil;

impl DictUtil {
    /// 对齐 Java: `Dict.create()`
    pub fn create() -> Dict {
        HashMap::new()
    }

    /// 对齐 Java: `Dict.of(Object[]...)`
    pub fn of(pairs: &[(&str, serde_json::Value)]) -> Dict {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    /// 对齐 Java: `Dict.getStr(String)`
    pub fn get_str(dict: &Dict, key: &str) -> Option<String> {
        dict.get(key).and_then(|v| {
            if let Some(s) = v.as_str() {
                return Some(s.to_string());
            }
            Some(v.to_string())
        })
    }

    /// 对齐 Java: `Dict.getInt(String)`
    pub fn get_int(dict: &Dict, key: &str) -> Option<i64> {
        dict.get(key).and_then(|v| v.as_i64())
    }

    /// 对齐 Java: `Dict.getFloat(String)`
    pub fn get_float(dict: &Dict, key: &str) -> Option<f64> {
        dict.get(key).and_then(|v| v.as_f64())
    }

    /// 对齐 Java: `Dict.getBool(String)`
    pub fn get_bool(dict: &Dict, key: &str) -> Option<bool> {
        dict.get(key).and_then(|v| v.as_bool())
    }

    /// 对齐 Java: `Dict.set(String, Object)`
    pub fn set(dict: &mut Dict, key: &str, value: serde_json::Value) {
        dict.insert(key.to_string(), value);
    }

    /// 对齐 Java: `Dict.containsKey(String)`
    pub fn contains_key(dict: &Dict, key: &str) -> bool {
        dict.contains_key(key)
    }

    /// 对齐 Java: `Dict.remove(String)`
    pub fn remove(dict: &mut Dict, key: &str) -> Option<serde_json::Value> {
        dict.remove(key)
    }
}
