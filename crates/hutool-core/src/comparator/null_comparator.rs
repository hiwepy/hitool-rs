//! 对齐: `cn.hutool.core.comparator.NullComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/NullComparator.java

use std::cmp::Ordering;
use std::marker::PhantomData;

/// 对齐 Java 类: `cn.hutool.core.comparator.NullComparator`
pub struct NullComparator<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    null_greater: bool,
    inner: Option<F>,
    _marker: PhantomData<T>,
}

impl<T, F> NullComparator<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    /// 对齐 Java: `NullComparator(boolean nullGreater, Comparator)`
    #[must_use]
    pub fn new(null_greater: bool, comparator: F) -> Self {
        Self {
            null_greater,
            inner: Some(comparator),
            _marker: PhantomData,
        }
    }

    /// 对齐 Java: `compare(T a, T b)` —— `Option` 语义。
    #[must_use]
    pub fn compare_option(&self, a: Option<&T>, b: Option<&T>) -> i32 {
        match (a, b) {
            (None, None) => 0,
            (None, Some(_)) => {
                if self.null_greater {
                    1
                } else {
                    -1
                }
            }
            (Some(_), None) => {
                if self.null_greater {
                    -1
                } else {
                    1
                }
            }
            (Some(x), Some(y)) => match &self.inner {
                Some(cmp) => match cmp(x, y) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                },
                None => 0,
            },
        }
    }

    /// 对齐 Java: `compare(T a, T b)` —— 非空引用。
    #[must_use]
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        match &self.inner {
            Some(cmp) => match cmp(a, b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            },
            None => 0,
        }
    }

    /// 对齐 Java: `thenComparing(Comparator)`
    #[must_use]
    pub fn then_comparing<G>(self, other: G) -> impl Fn(&T, &T) -> Ordering
    where
        G: Fn(&T, &T) -> Ordering + 'static,
        F: 'static,
        T: 'static,
    {
        let first = self.inner;
        move |a, b| {
            let primary = match &first {
                Some(cmp) => cmp(a, b),
                None => Ordering::Equal,
            };
            if primary != Ordering::Equal {
                return primary;
            }
            other(a, b)
        }
    }
}
