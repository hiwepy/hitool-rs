//! 对齐: `cn.hutool.core.text.StrMatcher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrMatcher.java
//!
//! 字符串模式匹配,使用 `${XXXXX}` 作为变量占位符。

use crate::{CoreError, Result};
use indexmap::IndexMap;

/// 对齐 Java: `StrMatcher#`
#[derive(Debug, Clone)]
pub struct StrMatcher {
    patterns: Vec<String>,
}

impl StrMatcher {
    /// 对齐 Java: `StrMatcher(String pattern)`
    pub fn new(pattern: &str) -> Self {
        Self {
            patterns: parse(pattern),
        }
    }

    /// 对齐 Java: `StrMatcher::match#Map<String,String> (String text)`
    pub fn match_text(&self, text: &str) -> Result<IndexMap<String, String>> {
        let mut result = IndexMap::new();
        let mut from = 0usize;
        let mut key: Option<String> = None;
        for part in &self.patterns {
            if is_wrap(part, "${", "}") {
                if key.is_some() {
                    return Err(CoreError::InvalidArgument {
                        name: "pattern",
                        reason: "Consecutive variables like ${a}${b} are not supported",
                    });
                }
                key = Some(part[2..part.len() - 1].to_string());
            } else {
                let to = text[from..].find(part.as_str()).map(|i| from + i);
                let Some(to) = to else {
                    return Ok(IndexMap::new());
                };
                if let Some(ref k) = key {
                    if to > from {
                        result.insert(k.clone(), text[from..to].to_string());
                    }
                }
                from = to + part.len();
                key = None;
            }
        }
        if let Some(k) = key {
            if from < text.len() {
                result.insert(k, text[from..].to_string());
            }
        }
        Ok(result)
    }
}

impl Default for StrMatcher {
    fn default() -> Self {
        Self::new("")
    }
}

fn is_wrap(s: &str, prefix: &str, suffix: &str) -> bool {
    s.starts_with(prefix) && s.ends_with(suffix) && s.len() >= prefix.len() + suffix.len()
}

fn parse(pattern: &str) -> Vec<String> {
    let mut patterns = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let length = chars.len();
    let mut c = '\0';
    let mut in_var = false;
    let mut part = String::new();
    for i in 0..length {
        let pre = c;
        c = chars[i];
        if in_var {
            part.push(c);
            if c == '}' {
                in_var = false;
                patterns.push(std::mem::take(&mut part));
            }
        } else if c == '{' && pre == '$' {
            in_var = true;
            let pre_text = if part.len() >= 1 {
                part[..part.len() - 1].to_string()
            } else {
                String::new()
            };
            if !pre_text.is_empty() {
                patterns.push(pre_text);
            }
            part.clear();
            part.push(pre);
            part.push(c);
        } else {
            part.push(c);
        }
    }
    if !part.is_empty() {
        patterns.push(part);
    }
    patterns
}
