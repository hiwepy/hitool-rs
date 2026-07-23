//! 对齐: `cn.hutool.core.comparator.ReverseComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/ReverseComparator.java

use std::cmp::Ordering;
use std::marker::PhantomData;

/// 对齐 Java 类: `cn.hutool.core.comparator.ReverseComparator`
pub struct ReverseComparator<E, F>
where
    F: Fn(&E, &E) -> Ordering,
{
    inner: F,
    _marker: PhantomData<E>,
}

impl<E, F> ReverseComparator<E, F>
where
    F: Fn(&E, &E) -> Ordering,
{
    /// 对齐 Java: `ReverseComparator(Comparator)`
    #[must_use]
    pub fn new(comparator: F) -> Self {
        Self {
            inner: comparator,
            _marker: PhantomData,
        }
    }

    /// 对齐 Java: `compare(E o1, E o2)`
    #[must_use]
    pub fn compare(&self, o1: &E, o2: &E) -> i32 {
        match (self.inner)(o1, o2).reverse() {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}
