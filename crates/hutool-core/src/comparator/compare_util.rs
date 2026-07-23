//! 对齐: `cn.hutool.core.comparator.CompareUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/CompareUtil.java

use std::cmp::Ordering;
use std::hash::Hash;

use super::indexed_comparator::IndexedComparator;
use super::pinyin_comparator::PinyinComparator;

/// 对齐 Java 类: `cn.hutool.core.comparator.CompareUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct CompareUtil;

impl CompareUtil {
    /// 对齐 Java: `naturalComparator()`
    #[must_use]
    pub fn natural_comparator<T: Ord>() -> impl Fn(&T, &T) -> Ordering {
        |a, b| a.cmp(b)
    }

    /// 对齐 Java: `compare(T c1, T c2, Comparator)`
    #[must_use]
    pub fn compare_with<T, F>(c1: &T, c2: &T, comparator: F) -> i32
    where
        F: Fn(&T, &T) -> Ordering,
    {
        match comparator(c1, c2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `compare(T c1, T c2)` —— 自然序，非空。
    #[must_use]
    pub fn compare_ord<T: Ord>(c1: &T, c2: &T) -> i32 {
        match c1.cmp(c2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `compare(T c1, T c2, boolean isNullGreater)`（`Comparable` / Option）
    #[must_use]
    pub fn compare<T: Ord>(c1: Option<T>, c2: Option<T>, null_greater: bool) -> i32 {
        match (c1, c2) {
            (Some(a), Some(b)) => match a.cmp(&b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            },
            (None, None) => 0,
            (None, Some(_)) => {
                if null_greater {
                    1
                } else {
                    -1
                }
            }
            (Some(_), None) => {
                if null_greater {
                    -1
                } else {
                    1
                }
            }
        }
    }

    /// 对齐 Java: `comparingPinyin(Function<T, String> keyExtractor)`
    #[must_use]
    pub fn comparing_pinyin<T, F>(key: F) -> impl Fn(&T, &T) -> Ordering
    where
        F: Fn(&T) -> &str,
    {
        Self::comparing_pinyin_reverse(key, false)
    }

    /// 对齐 Java: `comparingPinyin(Function<T, String> keyExtractor, boolean reverse)`
    #[must_use]
    pub fn comparing_pinyin_reverse<T, F>(key: F, reverse: bool) -> impl Fn(&T, &T) -> Ordering
    where
        F: Fn(&T) -> &str,
    {
        let pinyin = PinyinComparator::new();
        move |left, right| {
            let ordering = pinyin.compare(key(left), key(right));
            if reverse {
                ordering.reverse()
            } else {
                ordering
            }
        }
    }

    /// 对齐 Java: `comparingIndexed(Function, U...)`
    #[must_use]
    pub fn comparing_indexed<T, U, F>(
        key: F,
        objs: impl IntoIterator<Item = U>,
    ) -> impl Fn(&T, &T) -> Ordering
    where
        U: Eq + Hash + Clone,
        F: Fn(&T) -> U,
    {
        Self::comparing_indexed_miss(key, false, objs)
    }

    /// 对齐 Java: `comparingIndexed(Function, boolean atEndIfMiss, U...)`
    #[must_use]
    pub fn comparing_indexed_miss<T, U, F>(
        key: F,
        at_end_if_miss: bool,
        objs: impl IntoIterator<Item = U>,
    ) -> impl Fn(&T, &T) -> Ordering
    where
        U: Eq + Hash + Clone,
        F: Fn(&T) -> U,
    {
        let indexed = IndexedComparator::with_miss(at_end_if_miss, objs);
        move |left, right| {
            let c = indexed.compare(&key(left), &key(right));
            match c.cmp(&0) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater,
            }
        }
    }
}
