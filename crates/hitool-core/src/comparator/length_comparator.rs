//! 对齐: `cn.hutool.core.comparator.LengthComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/LengthComparator.java

use std::cmp::Ordering;

/// 对齐 Java 类: `cn.hutool.core.comparator.LengthComparator`
#[derive(Debug, Clone, Copy, Default)]
pub struct LengthComparator;

impl LengthComparator {
    /// 对齐 Java: `LengthComparator.INSTANCE`
    pub const INSTANCE: LengthComparator = LengthComparator;

    /// 对齐 Java: `compare(CharSequence o1, CharSequence o2)` —— 按字符长度。
    #[must_use]
    pub fn compare(&self, o1: &str, o2: &str) -> i32 {
        match o1.chars().count().cmp(&o2.chars().count()) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}
