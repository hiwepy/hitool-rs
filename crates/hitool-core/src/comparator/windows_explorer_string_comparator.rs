//! 对齐: `cn.hutool.core.comparator.WindowsExplorerStringComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/WindowsExplorerStringComparator.java

use regex::Regex;
use std::sync::OnceLock;

/// Windows 资源管理器风格字符串比较器。
#[derive(Debug, Clone, Default)]
pub struct WindowsExplorerStringComparator;

impl WindowsExplorerStringComparator {
    /// 对齐 Java: `INSTANCE`
    pub const INSTANCE: WindowsExplorerStringComparator = WindowsExplorerStringComparator;

    /// 对齐 Java: `compare(CharSequence, CharSequence)`
    pub fn compare(&self, str1: &str, str2: &str) -> i32 {
        let i1 = split_string_preserve_delimiter(str1);
        let i2 = split_string_preserve_delimiter(str2);
        let mut a = i1.into_iter();
        let mut b = i2.into_iter();
        loop {
            match (a.next(), b.next()) {
                (None, None) => return 0,
                (None, Some(_)) => return -1,
                (Some(_), None) => return 1,
                (Some(data1), Some(data2)) => {
                    let result = match (data1.parse::<i64>(), data2.parse::<i64>()) {
                        (Ok(n1), Ok(n2)) => {
                            let mut r = n1.cmp(&n2) as i32;
                            if r == 0 {
                                r = -((data1.len() as i32).cmp(&(data2.len() as i32)) as i32);
                            }
                            r
                        }
                        _ => cmp_ignore_case(&data1, &data2),
                    };
                    if result != 0 {
                        return result;
                    }
                }
            }
        }
    }
}

fn split_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\d+|\.|\s").unwrap())
}

fn split_string_preserve_delimiter(s: &str) -> Vec<String> {
    let re = split_pattern();
    let mut list = Vec::new();
    let mut pos = 0usize;
    for m in re.find_iter(s) {
        list.push(s[pos..m.start()].to_string());
        list.push(m.as_str().to_string());
        pos = m.end();
    }
    list.push(s[pos..].to_string());
    list
}

fn cmp_ignore_case(a: &str, b: &str) -> i32 {
    match a.to_lowercase().cmp(&b.to_lowercase()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
