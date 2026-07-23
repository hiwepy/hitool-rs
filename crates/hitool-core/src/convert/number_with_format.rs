//! 对齐: `cn.hutool.core.convert.NumberWithFormat`

#![allow(dead_code)]

/// 对齐 Java 类: `cn.hutool.core.convert.NumberWithFormat`
#[derive(Debug, Clone)]
pub struct NumberWithFormat {
    value: i64,
    format: Option<String>,
}

impl Default for NumberWithFormat {
    fn default() -> Self {
        Self {
            value: 0,
            format: None,
        }
    }
}

impl NumberWithFormat {
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    /// 对齐 Java 构造: `new NumberWithFormat(number, format)`
    pub fn new(value: i64, format: Option<String>) -> Self {
        Self { value, format }
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }
}
