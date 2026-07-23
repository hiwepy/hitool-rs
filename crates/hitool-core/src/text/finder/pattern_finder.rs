//! 对齐: `cn.hutool.core.text.finder.PatternFinder`

use crate::{CoreError, Result};
use regex::Regex;

/// 对齐 Java: `PatternFinder#`
#[derive(Debug, Clone)]
pub struct PatternFinder {
    re: Regex,
    text: String,
}

impl PatternFinder {
    /// 对齐 Java: `PatternFinder(Pattern pattern)`
    pub fn new(pattern: &str) -> Result<Self> {
        let re = Regex::new(pattern).map_err(|e| CoreError::Codec(format!("regex: {e}")))?;
        Ok(Self {
            re,
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
        match self.re.find_at(&self.text, from) {
            Some(m) => Ok(m.start() as i32),
            None => Ok(-1),
        }
    }

    /// 对齐 Java: `end`
    pub fn end_of(&self, start: i32) -> i32 {
        let start = start.max(0) as usize;
        match self.re.find_at(&self.text, start) {
            Some(m) if m.start() == start => m.end() as i32,
            _ => -1,
        }
    }
}
