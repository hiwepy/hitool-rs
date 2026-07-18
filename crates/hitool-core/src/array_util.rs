//! 对齐: `cn.hutool.core.util.ArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ArrayUtil.java
//!
//! Rust 版本提供数组操作的 idiomatic 实现。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ArrayUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayUtil;

impl ArrayUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `ArrayUtil.isEmpty(T[])`
    pub fn is_empty<T>(array: &[T]) -> bool {
        array.is_empty()
    }

    /// 对齐 Java: `ArrayUtil.isNotEmpty(T[])`
    pub fn is_not_empty<T>(array: &[T]) -> bool {
        !array.is_empty()
    }

    // ── 默认值 ──

    /// 对齐 Java: `ArrayUtil.defaultIfEmpty(T[], T[])`
    pub fn default_if_empty<T: Clone>(array: &[T], default: &[T]) -> Vec<T> {
        if array.is_empty() {
            default.to_vec()
        } else {
            array.to_vec()
        }
    }

    // ── 查找操作 ──

    /// 对齐 Java: `ArrayUtil.contains(T[], T)`
    pub fn contains<T: PartialEq>(array: &[T], value: &T) -> bool {
        array.contains(value)
    }

    /// 对齐 Java: `ArrayUtil.indexOf(T[], T)`
    pub fn index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().position(|x| x == value)
    }

    /// 对齐 Java: `ArrayUtil.lastIndexOf(T[], T)`
    pub fn last_index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().rposition(|x| x == value)
    }

    // ── 转换操作 ──

    /// 对齐 Java: `ArrayUtil.toArray(T...)`
    pub fn to_vec<T>(values: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        values.to_vec()
    }

    /// 对齐 Java: `ArrayUtil.newArray(Class, int)`
    pub fn new_array<T: Clone + Default>(size: usize) -> Vec<T> {
        vec![T::default(); size]
    }

    // ── 操作 ──

    /// 对齐 Java: `ArrayUtil.append(T[], T...)`
    pub fn append<T: Clone>(array: &[T], elements: &[T]) -> Vec<T> {
        let mut result = array.to_vec();
        result.extend_from_slice(elements);
        result
    }

    /// 对齐 Java: `ArrayUtil.insert(T[], int, T...)`
    pub fn insert<T: Clone>(array: &[T], index: usize, elements: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(array.len() + elements.len());
        result.extend_from_slice(&array[..index]);
        result.extend_from_slice(elements);
        result.extend_from_slice(&array[index..]);
        result
    }

    /// 对齐 Java: `ArrayUtil.remove(T[], int)`
    pub fn remove<T: Clone>(array: &[T], index: usize) -> Vec<T> {
        let mut result = array.to_vec();
        if index < result.len() {
            result.remove(index);
        }
        result
    }

    /// 对齐 Java: `ArrayUtil.removeEle(T[], T)`
    pub fn remove_element<T: Clone + PartialEq>(array: &[T], element: &T) -> Vec<T> {
        array.iter().filter(|x| *x != element).cloned().collect()
    }

    // ── 排序操作 ──

    /// 对齐 Java: `ArrayUtil.reverse(T[])`
    pub fn reverse<T>(array: &mut [T]) {
        array.reverse();
    }

    /// 对齐 Java: `ArrayUtil.sort(T[])`
    pub fn sort<T: Ord>(array: &mut [T]) {
        array.sort();
    }

    /// 对齐 Java: `ArrayUtil.sort(T[], Comparator)`
    pub fn sort_by<T>(array: &mut [T], compare: impl FnMut(&T, &T) -> std::cmp::Ordering) {
        array.sort_by(compare);
    }

    // ── 合并操作 ──

    /// 对齐 Java: `ArrayUtil.addAll(T[]...)`
    pub fn add_all<T: Clone>(arrays: &[&[T]]) -> Vec<T> {
        let total_len: usize = arrays.iter().map(|a| a.len()).sum();
        let mut result = Vec::with_capacity(total_len);
        for array in arrays {
            result.extend_from_slice(array);
        }
        result
    }

    // ── 截取操作 ──

    /// 对齐 Java: `ArrayUtil.sub(T[], int, int)`
    pub fn sub<T: Clone>(array: &[T], start: usize, end: usize) -> Vec<T> {
        let start = start.min(array.len());
        let end = end.min(array.len());
        if start >= end {
            return Vec::new();
        }
        array[start..end].to_vec()
    }

    /// 对齐 Java: `ArrayUtil.split(T[], int)`
    pub fn split<T: Clone>(array: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 || array.is_empty() {
            return vec![array.to_vec()];
        }
        array.chunks(size).map(|chunk| chunk.to_vec()).collect()
    }

    // ── 填充操作 ──

    /// 对齐 Java: `ArrayUtil.fill(T[], T)`
    pub fn fill<T: Clone>(array: &mut [T], value: &T) {
        array.fill_with(|| value.clone());
    }

    /// 对齐 Java: `ArrayUtil.fillRange(T[], int, int, T)`
    pub fn fill_range<T: Clone>(array: &mut [T], from: usize, to: usize, value: &T) {
        let to = to.min(array.len());
        for item in array.iter_mut().take(to).skip(from) {
            *item = value.clone();
        }
    }

    // ── 转换操作 ──

    /// 对齐 Java: `ArrayUtil.toString(T[])`
    pub fn to_string<T: std::fmt::Display>(array: &[T]) -> String {
        let parts: Vec<String> = array.iter().map(|x| x.to_string()).collect();
        format!("[{}]", parts.join(", "))
    }

    /// 对齐 Java: `ArrayUtil.join(CharSequence, T[])`
    pub fn join<T: std::fmt::Display>(array: &[T], delimiter: &str) -> String {
        array
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(delimiter)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `ArrayUtil.equals(T[], T[])`
    pub fn equals<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a == b
    }

    /// 对齐 Java: `ArrayUtil.isEquals(T[], T[])`
    pub fn is_equals<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a == b
    }
}
