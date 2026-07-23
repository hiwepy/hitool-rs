//! 对齐: `cn.hutool.core.text.finder.StrFinder`

use crate::{CoreError, Result};

/// 对齐 Java: `StrFinder#`
#[derive(Debug, Clone)]
pub struct StrFinder {
    needle: String,
    case_insensitive: bool,
    text: String,
}

impl StrFinder {
    /// 对齐 Java: `StrFinder(CharSequence str, boolean caseInsensitive)`
    pub fn new(needle: &str, case_insensitive: bool) -> Result<Self> {
        if needle.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "strToFind",
                reason: "empty string not allowed",
            });
        }
        Ok(Self {
            needle: needle.to_string(),
            case_insensitive,
            text: String::new(),
        })
    }

    /// 对齐 Java: `setText`
    pub fn set_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// 对齐 Java: `start`
    pub fn start(&self, from: i32) -> Result<i32> {
        let from = from.max(0) as usize;
        if from > self.text.len() {
            return Ok(-1);
        }
        let hay = if self.case_insensitive {
            self.text.to_ascii_lowercase()
        } else {
            self.text.clone()
        };
        let needle = if self.case_insensitive {
            self.needle.to_ascii_lowercase()
        } else {
            self.needle.clone()
        };
        // Use char indices carefully — for ASCII needles OK
        match hay[from..].find(&needle) {
            Some(rel) => Ok((from + rel) as i32),
            None => Ok(-1),
        }
    }

    /// 对齐 Java: `end`
    pub fn end(&self, start: i32) -> i32 {
        if start < 0 {
            -1
        } else {
            start + self.needle.chars().count() as i32
        }
    }
}
