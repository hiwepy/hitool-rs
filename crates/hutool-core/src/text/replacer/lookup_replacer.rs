//! 对齐: `cn.hutool.core.text.replacer.LookupReplacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/replacer/LookupReplacer.java
//!
//! 按最长优先关键字表做查找替换。

use std::collections::{HashMap, HashSet};

use crate::Result;

/// 对齐 Java: `LookupReplacer#`
#[derive(Debug, Clone)]
pub struct LookupReplacer {
    lookup: HashMap<String, String>,
    prefix_set: HashSet<char>,
    min_length: usize,
    max_length: usize,
}

impl LookupReplacer {
    /// 对齐 Java: `LookupReplacer(String[]... lookup)`
    ///
    /// `pairs` 为 `(key, value)` 列表。
    pub fn new(pairs: &[(&str, &str)]) -> Self {
        let mut lookup = HashMap::new();
        let mut prefix_set = HashSet::new();
        let mut min_length = usize::MAX;
        let mut max_length = 0usize;
        for &(key, value) in pairs {
            if key.is_empty() {
                continue;
            }
            lookup.insert(key.to_string(), value.to_string());
            if let Some(c) = key.chars().next() {
                prefix_set.insert(c);
            }
            let key_size = key.chars().count();
            max_length = max_length.max(key_size);
            min_length = min_length.min(key_size);
        }
        if min_length == usize::MAX {
            min_length = 0;
        }
        Self {
            lookup,
            prefix_set,
            min_length,
            max_length,
        }
    }

    /// 对齐 Java: 扁平 `String[]` 构造（偶数为 key、奇数为 value）。
    pub fn from_flat(lookup: &[&str]) -> Self {
        let mut pairs = Vec::new();
        let mut i = 0;
        while i + 1 < lookup.len() {
            pairs.push((lookup[i], lookup[i + 1]));
            i += 2;
        }
        Self::new(&pairs)
    }

    /// 对齐 Java: `StrReplacer.replace(CharSequence)` — 全串替换。
    pub fn replace(&self, text: &str) -> Result<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut pos = 0usize;
        while pos < chars.len() {
            if let Some(consumed) = self.replace_at(&chars, pos, &mut out) {
                pos += consumed;
            } else {
                out.push(chars[pos]);
                pos += 1;
            }
        }
        Ok(out)
    }

    /// 对齐 Java: `replace(CharSequence, int, StrBuilder)` — 从 `pos` 尝试一次匹配。
    fn replace_at(&self, chars: &[char], pos: usize, out: &mut String) -> Option<usize> {
        let c = chars[pos];
        if !self.prefix_set.contains(&c) || self.max_length == 0 {
            return None;
        }
        let remain = chars.len() - pos;
        let mut max = self.max_length.min(remain);
        while max >= self.min_length && max > 0 {
            let sub: String = chars[pos..pos + max].iter().collect();
            if let Some(result) = self.lookup.get(&sub) {
                out.push_str(result);
                return Some(max);
            }
            max -= 1;
        }
        None
    }
}
