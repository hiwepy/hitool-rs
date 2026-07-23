//! 对齐: `cn.hutool.core.collection.CollUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/collection/CollUtil.java
//!
//! Hutool 包路径对齐 facade。可映射方法委托到 crate 根上的 idiomatic
//! [`crate::CollUtil`]；反射 / `Class` / 拼音等无法无损映射的保留
//! [`CoreError::PendingEngine`]。
//!
//! 签名统一返回 [`Result`]，与其它 Hutool 命名桩（如 `CharSequenceUtil`）一致。

#![allow(dead_code, unused_variables, clippy::too_many_arguments)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    sync::Arc,
};

use indexmap::IndexSet;

use crate::{
    BlockingQueue, CollUtil as Idiomatic, CollectionKind, CoreError, CreatedCollection, Result,
};

/// 对齐 Java 类: `cn.hutool.core.collection.CollUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct CollUtil;

impl CollUtil {
    // ── 空值 / 判空 ──

    /// 对齐 Java: `CollUtil::emptyIfNull#List/Set (Collection)`
    pub fn empty_if_null<T>(values: Option<&[T]>) -> Result<&[T]> {
        Ok(Idiomatic::empty_if_none(values))
    }

    /// 对齐 Java: `CollUtil::isEmpty#boolean (Collection)`
    pub fn is_empty<T>(values: Option<&[T]>) -> Result<bool> {
        Ok(Idiomatic::is_empty(values))
    }

    /// 对齐 Java: `CollUtil::isNotEmpty#boolean (Collection)`
    pub fn is_not_empty<T>(values: Option<&[T]>) -> Result<bool> {
        Ok(Idiomatic::is_not_empty(values))
    }

    /// 对齐 Java: `CollUtil::defaultIfEmpty#Collection (Collection, Collection)`
    pub fn default_if_empty<T>(
        values: Vec<T>,
        default: impl FnOnce() -> Vec<T>,
    ) -> Result<Vec<T>> {
        Ok(Idiomatic::default_if_empty(values, default))
    }

    // ── 集合运算 ──

    /// 对齐 Java: `CollUtil::union#Collection (Collection, Collection)`
    pub fn union<T>(left: &[T], right: &[T]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::union(left, right))
    }

    /// 对齐 Java: `CollUtil::union#Collection (Collection, Collection, Collection...)`
    pub fn union_2<T>(collections: &[&[T]]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::union_many(collections))
    }

    /// 对齐 Java: `CollUtil::unionDistinct#Set (Collection...)`
    pub fn union_distinct<T>(collections: &[&[T]]) -> Result<IndexSet<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::union_distinct(collections))
    }

    /// 对齐 Java: `CollUtil::unionAll#List (Collection...)`
    pub fn union_all<T: Clone>(collections: &[&[T]]) -> Result<Vec<T>> {
        Ok(Idiomatic::union_all(collections))
    }

    /// 对齐 Java: `CollUtil::intersection#Collection (Collection, Collection)`
    pub fn intersection<T>(left: &[T], right: &[T]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::intersection(left, right))
    }

    /// 对齐 Java: `CollUtil::intersection#Collection (Collection, Collection, Collection...)`
    pub fn intersection_2<T>(collections: &[&[T]]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::intersection_many(collections))
    }

    /// 对齐 Java: `CollUtil::intersectionDistinct#Set (Collection...)`
    pub fn intersection_distinct<T>(collections: &[&[T]]) -> Result<IndexSet<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::intersection_distinct(collections))
    }

    /// 对齐 Java: `CollUtil::disjunction#Collection (Collection, Collection)`
    pub fn disjunction<T>(left: &[T], right: &[T]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::disjunction(left, right))
    }

    /// 对齐 Java: `CollUtil::subtract#Collection (Collection, Collection)`
    pub fn subtract<T>(left: &[T], right: &[T]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::subtract(left, right))
    }

    /// 对齐 Java: `CollUtil::subtractToList#List (Collection, Collection)`
    pub fn subtract_to_list<T>(left: &[T], right: &[T]) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        Ok(Idiomatic::subtract(left, right))
    }

    /// 对齐 Java: `CollUtil::subtractToList#List (Collection, Collection, boolean)`
    pub fn subtract_to_list_2<T>(left: &[T], right: &[T], _is_linked: bool) -> Result<Vec<T>>
    where
        T: Clone + Eq + Hash,
    {
        // Rust 统一收集为 Vec；isLinked 在 idiomatic 层无独立 Deque 重载。
        Ok(Idiomatic::subtract(left, right))
    }

    // ── 包含判断 ──

    /// 对齐 Java: `CollUtil::contains#boolean (Collection, Object)`
    pub fn contains<T: PartialEq>(values: &[T], value: &T) -> Result<bool> {
        Ok(Idiomatic::contains(values, value))
    }

    /// 对齐 Java: `CollUtil::safeContains#boolean (Collection, Object)`
    pub fn safe_contains<T: PartialEq>(values: &[T], value: &T) -> Result<bool> {
        Ok(Idiomatic::contains(values, value))
    }

    /// 对齐 Java: `CollUtil::contains#boolean (Collection, Predicate)`
    pub fn contains_2<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<bool> {
        Ok(Idiomatic::contains_by(values, matcher))
    }

    /// 对齐 Java: `CollUtil::containsAny#boolean (Collection, Collection)`
    pub fn contains_any<T: PartialEq>(left: &[T], right: &[T]) -> Result<bool> {
        Ok(Idiomatic::contains_any(left, right))
    }

    /// 对齐 Java: `CollUtil::containsAll#boolean (Collection, Collection)`
    pub fn contains_all<T: PartialEq>(left: &[T], right: &[T]) -> Result<bool> {
        Ok(Idiomatic::contains_all(left, right))
    }

    // ── 工厂 ──

    /// 对齐 Java: `CollUtil::newHashSet#HashSet (T...)`
    pub fn new_hash_set<T>(values: impl IntoIterator<Item = T>) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        Ok(Idiomatic::new_hash_set(values))
    }

    /// 对齐 Java: `CollUtil::newLinkedHashSet#LinkedHashSet (T...)`
    pub fn new_linked_hash_set<T>(values: impl IntoIterator<Item = T>) -> Result<IndexSet<T>>
    where
        T: Eq + Hash,
    {
        Ok(Idiomatic::new_linked_hash_set(values))
    }

    /// 对齐 Java: `CollUtil::newArrayList#ArrayList (T...)`
    pub fn new_array_list<T>(values: impl IntoIterator<Item = T>) -> Result<Vec<T>> {
        Ok(Idiomatic::new_array_list(values))
    }

    /// 对齐 Java: `CollUtil::toList#ArrayList (T...)`
    pub fn to_list<T>(values: impl IntoIterator<Item = T>) -> Result<Vec<T>> {
        Ok(Idiomatic::new_array_list(values))
    }

    /// 对齐 Java: `CollUtil::newLinkedList#LinkedList (T...)`
    pub fn new_linked_list<T>(values: impl IntoIterator<Item = T>) -> Result<VecDeque<T>> {
        Ok(Idiomatic::new_linked_list(values))
    }

    /// 对齐 Java: `CollUtil::newCopyOnWriteArrayList#CopyOnWriteArrayList (Collection)`
    pub fn new_copy_on_write_array_list<T>(
        values: impl IntoIterator<Item = T>,
    ) -> Result<Arc<Vec<T>>> {
        Ok(Idiomatic::new_copy_on_write_array_list(values))
    }

    /// 对齐 Java: `CollUtil::newBlockingQueue#BlockingQueue (int, boolean)`
    pub fn new_blocking_queue<T>(capacity: usize, _is_linked: bool) -> Result<BlockingQueue<T>> {
        Idiomatic::new_blocking_queue(capacity)
    }

    /// 对齐 Java: `CollUtil::create#Collection (Class)` — 无反射，改用 [`CollectionKind`]。
    pub fn create<T>(kind: CollectionKind) -> Result<CreatedCollection<T>> {
        Ok(Idiomatic::create(kind))
    }

    /// 对齐 Java: `CollUtil::create#Collection (Class, Class)` — 元素类型由泛型承担。
    pub fn create_2<T>(_kind_name: &str) -> Result<()> {
        Err(CoreError::PendingEngine("CollUtil::create_2"))
    }

    // ── 去重 / 过滤 / 变换 ──

    /// 对齐 Java: `CollUtil::distinct#ArrayList (Collection)`
    pub fn distinct<T>(values: impl IntoIterator<Item = T>) -> Result<Vec<T>>
    where
        T: Eq + Hash,
    {
        Ok(Idiomatic::distinct(values))
    }

    /// 对齐 Java: `CollUtil::distinct#List (Collection, Function, boolean)`
    pub fn distinct_2<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
        override_existing: bool,
    ) -> Result<Vec<T>>
    where
        K: Eq + Hash,
    {
        Ok(Idiomatic::distinct_by(values, key_of, override_existing))
    }

    /// 对齐 Java: `CollUtil::filterNew#Collection (Collection, Filter)`
    pub fn filter_new<T>(
        values: impl IntoIterator<Item = T>,
        predicate: impl FnMut(&T) -> bool,
    ) -> Result<Vec<T>> {
        Ok(Idiomatic::filter_new(values, predicate))
    }

    /// 对齐 Java: `CollUtil::filter#Collection (Collection, Filter)`
    pub fn filter<T>(values: &mut Vec<T>, predicate: impl FnMut(&T) -> bool) -> Result<()> {
        Idiomatic::filter(values, predicate);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::edit#Collection (Collection, Editor)`
    pub fn edit<T, U>(
        values: impl IntoIterator<Item = T>,
        editor: impl FnMut(T) -> Option<U>,
    ) -> Result<Vec<U>> {
        Ok(Idiomatic::edit(values, editor))
    }

    /// 对齐 Java: `CollUtil::map#List (Iterable, Function, boolean)`
    pub fn map<T, R>(
        values: impl IntoIterator<Item = T>,
        mut mapper: impl FnMut(T) -> Option<R>,
        ignore_null: bool,
    ) -> Result<Vec<R>> {
        let _ = ignore_null;
        Ok(values.into_iter().filter_map(|value| mapper(value)).collect())
    }

    /// 对齐 Java: `CollUtil::removeAny#Collection (Collection, E...)`
    pub fn remove_any<T>(values: &mut Vec<T>, removed: &[T]) -> Result<()>
    where
        T: Eq + Hash,
    {
        Idiomatic::remove_any(values, removed);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::removeNull#Collection (Collection)`
    pub fn remove_null<T>(values: &mut Vec<Option<T>>) -> Result<()> {
        Idiomatic::remove_none(values);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::removeEmpty#Collection (Collection)`
    pub fn remove_empty(values: &mut Vec<String>) -> Result<()> {
        Idiomatic::remove_empty(values);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::removeBlank#Collection (Collection)`
    pub fn remove_blank(values: &mut Vec<String>) -> Result<()> {
        Idiomatic::remove_blank(values);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::removeWithAddIf#List (Collection, Predicate)`
    pub fn remove_with_add_if<T>(
        values: &mut Vec<T>,
        predicate: impl FnMut(&T) -> bool,
    ) -> Result<Vec<T>> {
        Ok(Idiomatic::remove_with_add_if(values, predicate))
    }

    // ── 切片 / 分页 / 排序 ──

    /// 对齐 Java: `CollUtil::sub#List (List, int, int, int)`
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>> {
        Idiomatic::sub(values, start, end, step)
    }

    /// 对齐 Java: `CollUtil::split#List (Collection, int)`
    pub fn split<T: Clone>(values: &[T], size: usize) -> Result<Vec<Vec<T>>> {
        Idiomatic::split(values, size)
    }

    /// 对齐 Java: `CollUtil::splitList#List (List, int)`
    pub fn split_list<T: Clone>(values: &[T], size: usize) -> Result<Vec<Vec<T>>> {
        Idiomatic::split(values, size)
    }

    /// 对齐 Java: `CollUtil::page#List (int, int, List)`
    pub fn page<T: Clone>(values: &[T], page_no: usize, page_size: usize) -> Result<Vec<T>> {
        Idiomatic::page(values, page_no, page_size)
    }

    /// 对齐 Java: `CollUtil::sort#List (List, Comparator)`
    pub fn sort<T: Clone>(
        values: &[T],
        compare: impl FnMut(&T, &T) -> Ordering,
    ) -> Result<Vec<T>> {
        Ok(Idiomatic::sort(values, compare))
    }

    /// 对齐 Java: `CollUtil::sort` 原地变体（Rust 惯用）
    pub fn sort_in_place<T>(
        values: &mut [T],
        compare: impl FnMut(&T, &T) -> Ordering,
    ) -> Result<()> {
        Idiomatic::sort_in_place(values, compare);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::reverse#List (List)`
    pub fn reverse<T>(values: &mut [T]) -> Result<()> {
        Idiomatic::reverse(values);
        Ok(())
    }

    // ── 聚合 / 查询 ──

    /// 对齐 Java: `CollUtil::countMap#Map (Iterable)`
    pub fn count_map<T>(values: impl IntoIterator<Item = T>) -> Result<HashMap<T, usize>>
    where
        T: Eq + Hash,
    {
        Ok(Idiomatic::count_map(values))
    }

    /// 对齐 Java: `CollUtil::join#String (Iterable, CharSequence)`
    pub fn join<T: Display>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
    ) -> Result<String> {
        Ok(Idiomatic::join(values, delimiter))
    }

    /// 对齐 Java: `CollUtil::join#String (Iterable, CharSequence, Function)`
    pub fn join_2<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        mapper: impl FnMut(T) -> String,
    ) -> Result<String> {
        Ok(Idiomatic::join_by(values, delimiter, mapper))
    }

    /// 对齐 Java: `CollUtil::anyMatch#boolean (Collection, Predicate)`
    pub fn any_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<bool> {
        Ok(Idiomatic::any_match(values, matcher))
    }

    /// 对齐 Java: `CollUtil::allMatch#boolean (Collection, Predicate)`
    pub fn all_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<bool> {
        Ok(Idiomatic::all_match(values, matcher))
    }

    /// 对齐 Java: `CollUtil::findOne#T (Iterable, Filter)`
    pub fn find_one<T>(
        values: impl IntoIterator<Item = T>,
        matcher: impl FnMut(&T) -> bool,
    ) -> Result<Option<T>> {
        Ok(Idiomatic::find_one(values, matcher))
    }

    /// 对齐 Java: `CollUtil::indexOf#int (Collection, Matcher)`
    pub fn index_of<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<Option<usize>> {
        Ok(Idiomatic::index_of(values, matcher))
    }

    /// 对齐 Java: `CollUtil::lastIndexOf#int (Collection, Matcher)`
    pub fn last_index_of<T>(
        values: &[T],
        matcher: impl FnMut(&T) -> bool,
    ) -> Result<Option<usize>> {
        Ok(Idiomatic::last_index_of(values, matcher))
    }

    /// 对齐 Java: `CollUtil::indexOfAll#int[] (Collection, Matcher)`
    pub fn index_of_all<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Result<Vec<usize>> {
        Ok(Idiomatic::index_of_all(values, matcher))
    }

    /// 对齐 Java: `CollUtil::count#int (Iterable, Matcher)`
    pub fn count<T>(
        values: impl IntoIterator<Item = T>,
        matcher: impl FnMut(&T) -> bool,
    ) -> Result<usize> {
        Ok(Idiomatic::count(values, matcher))
    }

    /// 对齐 Java: `CollUtil::get#T (Collection, int)`
    pub fn get<T>(values: &[T], index: isize) -> Result<Option<&T>> {
        Ok(Idiomatic::get(values, index))
    }

    /// 对齐 Java: 首元素
    pub fn get_first<T>(values: &[T]) -> Result<Option<&T>> {
        Ok(Idiomatic::first(values))
    }

    /// 对齐 Java: 末元素
    pub fn get_last<T>(values: &[T]) -> Result<Option<&T>> {
        Ok(Idiomatic::last(values))
    }

    /// 对齐 Java: `CollUtil::popPart#List (Deque, int)`
    pub fn pop_part<T>(values: &mut VecDeque<T>, part_size: usize) -> Result<Vec<T>> {
        Ok(Idiomatic::pop_part(values, part_size))
    }

    /// 对齐 Java: `CollUtil::zip#Map (Iterable, Iterable)`
    pub fn zip<K, V>(
        keys: impl IntoIterator<Item = K>,
        values: impl IntoIterator<Item = V>,
    ) -> Result<HashMap<K, V>>
    where
        K: Eq + Hash,
    {
        Ok(Idiomatic::zip(keys, values))
    }

    /// 对齐 Java: `CollUtil::addIfAbsent#boolean (Collection, T)`
    pub fn add_if_absent<T: PartialEq>(values: &mut Vec<T>, value: Option<T>) -> Result<bool> {
        Ok(Idiomatic::add_if_absent(values, value))
    }

    /// 对齐 Java: `CollUtil::addAll#boolean (Collection, Iterable)`
    pub fn add_all<T>(values: &mut Vec<T>, extra: impl IntoIterator<Item = T>) -> Result<()> {
        Idiomatic::add_all(values, extra);
        Ok(())
    }

    /// 对齐 Java: `CollUtil::toIdentityMap#Map (Iterable, Function)`
    pub fn to_identity_map<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
    ) -> Result<HashMap<K, T>>
    where
        K: Eq + Hash,
    {
        Ok(Idiomatic::to_identity_map(values, key_of))
    }

    /// 对齐 Java: `CollUtil::toMap#Map (Iterable, Function, Function)`
    pub fn to_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
        value_of: impl FnMut(T) -> V,
    ) -> Result<HashMap<K, V>>
    where
        K: Eq + Hash,
    {
        Ok(Idiomatic::to_map(values, key_of, value_of))
    }

    /// 对齐 Java: `CollUtil::group` 简化为索引分组
    pub fn group<T>(
        values: impl IntoIterator<Item = T>,
        hasher: impl FnMut(&T) -> usize,
    ) -> Result<Vec<Vec<T>>> {
        Ok(Idiomatic::group(values, hasher))
    }

    /// 对齐 Java: `CollUtil::set#HashSet (boolean, T...)`
    pub fn set_sorted<T>(
        is_sorted: bool,
        values: impl IntoIterator<Item = T>,
    ) -> Result<CreatedCollection<T>>
    where
        T: Eq + Hash + Ord,
    {
        let kind = if is_sorted {
            CollectionKind::SortedSet
        } else {
            CollectionKind::Set
        };
        let mut created = Idiomatic::create(kind);
        match &mut created {
            CreatedCollection::Set(set) => set.extend(values),
            CreatedCollection::SortedSet(set) => set.extend(values),
            _ => {}
        }
        Ok(created)
    }

    // ── 仍 Pending（反射 / 拼音 / Enumeration 等）──

    /// 对齐 Java: `CollUtil::getFieldValues` — 依赖反射字段名，保留 Pending。
    pub fn get_field_values(_collection: &[()], _field_name: &str) -> Result<Vec<()>> {
        Err(CoreError::PendingEngine("CollUtil::get_field_values"))
    }

    /// 对齐 Java: `CollUtil::findOneByField` — 依赖反射字段名，保留 Pending。
    pub fn find_one_by_field(
        _collection: &[()],
        _field_name: &str,
        _field_value: &(),
    ) -> Result<()> {
        Err(CoreError::PendingEngine("CollUtil::find_one_by_field"))
    }

    /// 对齐 Java: `CollUtil::sortByPinyin` — 无内置拼音引擎，保留 Pending。
    pub fn sort_by_pinyin(_list: &mut [String]) -> Result<()> {
        Err(CoreError::PendingEngine("CollUtil::sort_by_pinyin"))
    }

    /// 对齐 Java: `CollUtil::asEnumeration` — Java Enumeration 无直接对应，保留 Pending。
    pub fn as_enumeration(_iter: &[()]) -> Result<()> {
        Err(CoreError::PendingEngine("CollUtil::as_enumeration"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delegates_is_empty_and_new_array_list() {
        assert!(CollUtil::is_empty(None::<&[i32]>).unwrap());
        assert_eq!(CollUtil::new_array_list([1, 2, 3]).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn delegates_union_and_distinct() {
        let a = [1, 2];
        let b = [2, 3];
        assert_eq!(CollUtil::union(&a, &b).unwrap(), vec![1, 2, 3]);
        assert_eq!(CollUtil::distinct([1, 2, 2, 3]).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn pending_reflection_stays_pending() {
        assert!(matches!(
            CollUtil::get_field_values(&[], "x"),
            Err(CoreError::PendingEngine(_))
        ));
    }
}
