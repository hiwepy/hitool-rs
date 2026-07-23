//! 对齐: `cn.hutool.core.comparator.PinyinComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/PinyinComparator.java
//!
//! Java 使用 `Collator.getInstance(Locale.CHINESE)`；Rust 侧以 GBK 字节序近似中文拼音排序，
//! 与 Hutool 文档「按 GBK 拼音顺序」及 `CompareUtilTest.comparingPinyin` 用例一致。

use std::cmp::Ordering;

use encoding_rs::GBK;

/// 中文（拼音/GBK）字符串比较器 —— 对齐 Java `PinyinComparator`。
#[derive(Debug, Clone, Copy, Default)]
pub struct PinyinComparator;

impl PinyinComparator {
    /// 对齐 Java: `PinyinComparator()`
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `compare(String o1, String o2)`
    #[must_use]
    pub fn compare(&self, o1: &str, o2: &str) -> Ordering {
        gbk_sort_key(o1).cmp(&gbk_sort_key(o2))
    }
}

/// 将字符串编码为 GBK 排序键（不可编码字符按 UTF-8 回退）。
fn gbk_sort_key(text: &str) -> Vec<u8> {
    let (encoded, _, had_errors) = GBK.encode(text);
    if had_errors {
        text.as_bytes().to_vec()
    } else {
        encoded.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 对齐 Hutool `CompareUtilTest.comparingPinyin` 四城顺序。
    #[test]
    fn city_pinyin_order() {
        let cmp = PinyinComparator::new();
        let mut cities = ["成都", "北京", "上海", "深圳"];
        cities.sort_by(|a, b| cmp.compare(a, b));
        assert_eq!(cities, ["北京", "成都", "上海", "深圳"]);
    }
}
