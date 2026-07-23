//! 对齐: `cn.hutool.core.collection.ListUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/collection/ListUtil.java
//!
//! Hutool 包路径对齐 facade。可映射方法委托到 crate 根上的 idiomatic
//! [`crate::ListUtil`]；无法无损映射的保留 [`CoreError::PendingEngine`]。

#![allow(dead_code, unused_variables, clippy::too_many_arguments)]

use std::{cmp::Ordering, collections::VecDeque, sync::Arc};

use crate::{AvgPartition, CoreError, ListUtil as Idiomatic, Partition, Result};

/// 对齐 Java 类: `cn.hutool.core.collection.ListUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ListUtil;

impl ListUtil {
    /// 对齐 Java: `ListUtil::toList#ArrayList (T...)`
    pub fn to_list<T>(values: impl IntoIterator<Item = T>) -> Result<Vec<T>> {
        Ok(Idiomatic::to_list(values))
    }

    /// 对齐 Java: `ListUtil::toLinkedList#LinkedList (T...)`
    pub fn to_linked_list<T>(values: impl IntoIterator<Item = T>) -> Result<VecDeque<T>> {
        Ok(Idiomatic::to_linked_list(values))
    }

    /// 对齐 Java: `ListUtil::of#List (T...)`
    pub fn of<T>(values: impl IntoIterator<Item = T>) -> Result<Vec<T>> {
        Ok(Idiomatic::to_list(values))
    }

    /// 对齐 Java: `ListUtil::toCopyOnWriteArrayList#CopyOnWriteArrayList (Collection)`
    pub fn to_copy_on_write_array_list<T>(
        values: impl IntoIterator<Item = T>,
    ) -> Result<Arc<Vec<T>>> {
        Ok(Arc::new(Idiomatic::to_list(values)))
    }

    /// 对齐 Java: `ListUtil::page#List (int, int, List)`
    pub fn page<T>(values: &[T], page_no: usize, page_size: usize) -> Result<&[T]> {
        Idiomatic::page(values, page_no, page_size)
    }

    /// 对齐 Java: `ListUtil::page#void (List, int, Consumer)`
    pub fn page_2<T>(
        values: &[T],
        page_size: usize,
        consumer: impl FnMut(&[T]),
    ) -> Result<()> {
        Idiomatic::for_each_page(values, page_size, consumer)
    }

    /// 对齐 Java: `ListUtil::sort#List (List, Comparator)`
    pub fn sort<T>(values: &mut [T], compare: impl FnMut(&T, &T) -> Ordering) -> Result<()> {
        Idiomatic::sort_by(values, compare);
        Ok(())
    }

    /// 对齐 Java: `ListUtil::reverse#List (List)`
    pub fn reverse<T>(values: &mut [T]) -> Result<()> {
        Idiomatic::reverse(values);
        Ok(())
    }

    /// 对齐 Java: `ListUtil::reverseNew#List (List)`
    pub fn reverse_new<T: Clone>(values: &[T]) -> Result<Vec<T>> {
        Ok(Idiomatic::reverse_new(values))
    }

    /// 对齐 Java: `ListUtil::setOrAppend#List (List, int, T)`
    pub fn set_or_append<T>(values: &mut Vec<T>, index: usize, value: T) -> Result<()> {
        Idiomatic::set_or_append(values, index, value);
        Ok(())
    }

    /// 对齐 Java: `ListUtil::setOrPadding#List (List, int, T, T)`
    pub fn set_or_padding<T: Clone>(
        values: &mut Vec<T>,
        index: usize,
        value: T,
        padding: T,
    ) -> Result<()> {
        Idiomatic::set_or_padding(values, index, value, padding)
    }

    /// 对齐 Java: `ListUtil::sub#List (List, int, int, int)`
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>> {
        Idiomatic::sub(values, start, end, step)
    }

    /// 对齐 Java: `ListUtil::lastIndexOf#int (List, Matcher)`
    pub fn last_index_of<T>(
        values: &[T],
        matcher: impl FnMut(&T) -> bool,
    ) -> Result<Option<usize>> {
        Ok(Idiomatic::last_index_of(values, matcher))
    }

    /// 对齐 Java: `ListUtil::indexOfAll#int[] (List, Matcher)`
    pub fn index_of_all<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<Vec<usize>> {
        Ok(Idiomatic::index_of_all(values, matcher))
    }

    /// 对齐 Java: `ListUtil::partition#List (List, int)`
    pub fn partition<T>(values: &[T], size: usize) -> Result<Partition<'_, T>> {
        Idiomatic::partition(values, size)
    }

    /// 对齐 Java: `ListUtil::split#List (List, int)`
    pub fn split<T>(values: &[T], size: usize) -> Result<Partition<'_, T>> {
        Idiomatic::partition(values, size)
    }

    /// 对齐 Java: `ListUtil::splitAvg#List (List, int)`
    pub fn split_avg<T>(values: &[T], limit: usize) -> Result<AvgPartition<'_, T>> {
        Idiomatic::split_avg(values, limit)
    }

    /// 对齐 Java: `ListUtil::swapTo`
    pub fn swap_to<T: PartialEq>(
        values: &mut [T],
        element: &T,
        target_index: usize,
    ) -> Result<bool> {
        Idiomatic::swap_to(values, element, target_index)
    }

    /// 对齐 Java: `ListUtil::swapElement`
    pub fn swap_element<T: PartialEq>(
        values: &mut [T],
        element: &T,
        target: &T,
    ) -> Result<bool> {
        Ok(Idiomatic::swap_element(values, element, target))
    }

    /// 对齐 Java: `ListUtil::move`
    pub fn move_element<T: PartialEq>(
        values: &mut Vec<T>,
        element: T,
        new_position: usize,
    ) -> Result<()> {
        Idiomatic::move_element(values, element, new_position)
    }

    /// 对齐 Java: `ListUtil::zip#List (List, List, BiFunction)`
    pub fn zip<A, B, R>(
        left: impl IntoIterator<Item = A>,
        right: impl IntoIterator<Item = B>,
        zipper: impl FnMut(A, B) -> R,
    ) -> Result<Vec<R>> {
        Ok(Idiomatic::zip(left, right, zipper))
    }

    /// 对齐 Java: `ListUtil::empty#List ()`
    pub fn empty<T>() -> Result<Vec<T>> {
        Ok(Vec::new())
    }

    /// 对齐 Java: `ListUtil::unmodifiable` — Rust 无运行时不可变包装，保留 Pending。
    pub fn unmodifiable<T>(_values: &[T]) -> Result<()> {
        Err(CoreError::PendingEngine("ListUtil::unmodifiable"))
    }

    /// 对齐 Java: `ListUtil::sortByProperty` — 依赖反射属性名，保留 Pending。
    pub fn sort_by_property(_list: &mut [()], _property: &str) -> Result<()> {
        Err(CoreError::PendingEngine("ListUtil::sort_by_property"))
    }

    /// 对齐 Java: `ListUtil::sortByPinyin` — 无内置拼音引擎，保留 Pending。
    pub fn sort_by_pinyin(_list: &mut [String]) -> Result<()> {
        Err(CoreError::PendingEngine("ListUtil::sort_by_pinyin"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delegates_to_list_and_reverse() {
        assert_eq!(ListUtil::to_list([1, 2, 3]).unwrap(), vec![1, 2, 3]);
        let mut v = vec![1, 2, 3];
        ListUtil::reverse(&mut v).unwrap();
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn pending_stays_pending() {
        assert!(matches!(
            ListUtil::unmodifiable(&[1]),
            Err(CoreError::PendingEngine(_))
        ));
    }
}
