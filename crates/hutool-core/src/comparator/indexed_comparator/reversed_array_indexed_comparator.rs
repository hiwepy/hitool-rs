//! 对齐: `cn.hutool.core.comparator.IndexedComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/IndexedComparator.java

use std::collections::HashMap;
use std::hash::Hash;

use super::array_indexed_comparator::ArrayIndexedComparator;

/// 反序包装。
#[derive(Debug, Clone)]
pub struct ReversedArrayIndexedComparator<T> {
    inner: ArrayIndexedComparator<T>,
}

impl<T: PartialEq> ReversedArrayIndexedComparator<T> {
    /// 反序比较。
    pub fn compare(&self, o1: &T, o2: &T) -> i32 {
        -self.inner.compare(o1, o2)
    }
}
