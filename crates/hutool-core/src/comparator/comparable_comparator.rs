//! 对齐: `cn.hutool.core.comparator.ComparableComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/ComparableComparator.java
//!
//! 对实现 `Ord` 的类型做自然序比较。

use std::cmp::Ordering;

/// 对齐 Java 类: `cn.hutool.core.comparator.ComparableComparator`
#[derive(Debug, Clone, Copy, Default)]
pub struct ComparableComparator;

impl ComparableComparator {
    /// 对齐 Java: `ComparableComparator.INSTANCE`
    pub const INSTANCE: ComparableComparator = ComparableComparator;

    /// 对齐 Java: `ComparableComparator()`
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `compare(E obj1, E obj2)`
    #[must_use]
    pub fn compare<E: Ord>(&self, obj1: &E, obj2: &E) -> i32 {
        match obj1.cmp(obj2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

impl PartialEq for ComparableComparator {
    /// 对齐 Java: `equals(Object)` —— 同类即相等。
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for ComparableComparator {}

impl std::hash::Hash for ComparableComparator {
    /// 对齐 Java: `hashCode()`
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        "ComparableComparator".hash(state);
    }
}
