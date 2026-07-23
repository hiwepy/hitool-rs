//! 对齐: `cn.hutool.core.text.csv.CsvTokener`

/// 对齐 Java: `CsvTokener#`
#[derive(Debug)]
pub struct CsvTokener {
    chars: Vec<char>,
    pos: isize,
}

impl CsvTokener {
    /// 从字符串构造
    pub fn from_str(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            pos: -1,
        }
    }

    /// 对齐 Java: `next`
    pub fn next(&mut self) -> i32 {
        self.pos += 1;
        if self.pos < 0 || self.pos as usize >= self.chars.len() {
            -1
        } else {
            self.chars[self.pos as usize] as i32
        }
    }

    /// 对齐 Java: `back`
    pub fn back(&mut self) {
        self.pos -= 1;
    }
}
