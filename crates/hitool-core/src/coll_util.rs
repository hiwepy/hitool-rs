//! General collection operations aligned with Hutool's `CollUtil` capability model.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    sync::{
        Arc,
        mpsc::{Receiver, RecvError, SendError, SyncSender, sync_channel},
    },
};

use indexmap::{IndexMap, IndexSet};
use parking_lot::Mutex;

use crate::{CoreError, IterUtil, ListUtil, Result};

/// Concrete collection kinds replacing Java's reflective `Class<?>` factory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKind {
    /// Contiguous growable list.
    List,
    /// Double-ended linked-style list.
    Deque,
    /// Unordered unique collection.
    Set,
    /// Insertion-ordered unique collection.
    OrderedSet,
    /// Key-ordered unique collection.
    SortedSet,
}

/// A statically typed result for [`CollUtil::create`].
#[derive(Debug, Clone)]
pub enum CreatedCollection<T> {
    /// [`Vec`] collection.
    List(Vec<T>),
    /// [`VecDeque`] collection.
    Deque(VecDeque<T>),
    /// [`HashSet`] collection.
    Set(HashSet<T>),
    /// [`IndexSet`] collection.
    OrderedSet(IndexSet<T>),
    /// [`BTreeSet`] collection.
    SortedSet(BTreeSet<T>),
}

/// A bounded multi-producer queue with blocking send and receive operations.
#[derive(Debug)]
pub struct BlockingQueue<T> {
    sender: SyncSender<T>,
    receiver: Mutex<Receiver<T>>,
}

impl<T> BlockingQueue<T> {
    /// Sends a value, waiting while the queue is full.
    pub fn send(&self, value: T) -> std::result::Result<(), SendError<T>> {
        self.sender.send(value)
    }

    /// Receives a value, waiting while the queue is empty.
    pub fn recv(&self) -> std::result::Result<T, RecvError> {
        self.receiver.lock().recv()
    }
}

/// Hutool-aligned collection operations with Rust-native static typing.
#[derive(Debug, Clone, Copy, Default)]
pub struct CollUtil;

impl CollUtil {
    /// Returns the supplied slice or an empty slice when it is absent.
    #[must_use]
    pub fn empty_if_none<T>(values: Option<&[T]>) -> &[T] {
        values.unwrap_or_default()
    }

    /// Multiset union: keeps the maximum occurrence count from either input.
    #[must_use]
    pub fn union<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        left.iter()
            .chain(right)
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .max(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Multiset union across any number of collections.
    #[must_use]
    pub fn union_many<T>(collections: &[&[T]]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        collections
            .iter()
            .fold(Vec::new(), |union, values| Self::union(&union, values))
    }

    /// Insertion-ordered distinct union across collections.
    #[must_use]
    pub fn union_distinct<T>(collections: &[&[T]]) -> IndexSet<T>
    where
        T: Clone + Eq + Hash,
    {
        collections
            .iter()
            .flat_map(|values| values.iter().cloned())
            .collect()
    }

    /// Concatenates every collection without deduplication.
    #[must_use]
    pub fn union_all<T: Clone>(collections: &[&[T]]) -> Vec<T> {
        collections
            .iter()
            .flat_map(|values| values.iter().cloned())
            .collect()
    }

    /// Multiset intersection: keeps the minimum occurrence count.
    #[must_use]
    pub fn intersection<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        right
            .iter()
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .min(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Multiset intersection across any number of collections.
    #[must_use]
    pub fn intersection_many<T>(collections: &[&[T]]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let Some((first, rest)) = collections.split_first() else {
            return Vec::new();
        };
        rest.iter().fold(first.to_vec(), |values, next| {
            Self::intersection(&values, next)
        })
    }

    /// Insertion-ordered distinct intersection across collections.
    #[must_use]
    pub fn intersection_distinct<T>(collections: &[&[T]]) -> IndexSet<T>
    where
        T: Clone + Eq + Hash,
    {
        let Some((first, rest)) = collections.split_first() else {
            return IndexSet::new();
        };
        first
            .iter()
            .filter(|value| rest.iter().all(|values| values.contains(value)))
            .cloned()
            .collect()
    }

    /// Symmetric multiset difference: keeps the absolute count difference.
    #[must_use]
    pub fn disjunction<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        left.iter()
            .chain(right)
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .abs_diff(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Removes every left value that occurs at least once in `right`.
    #[must_use]
    pub fn subtract<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let removed: HashSet<_> = right.iter().collect();
        left.iter()
            .filter(|value| !removed.contains(value))
            .cloned()
            .collect()
    }

    /// Returns whether a value is present.
    #[must_use]
    pub fn contains<T: PartialEq>(values: &[T], value: &T) -> bool {
        values.contains(value)
    }

    /// Returns whether any value matches a predicate.
    #[must_use]
    pub fn contains_by<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        values.iter().any(matcher)
    }

    /// Returns whether the two slices share at least one value.
    #[must_use]
    pub fn contains_any<T: PartialEq>(left: &[T], right: &[T]) -> bool {
        left.iter().any(|value| right.contains(value))
    }

    /// Returns whether every distinct right value appears in the left slice.
    #[must_use]
    pub fn contains_all<T: PartialEq>(left: &[T], right: &[T]) -> bool {
        right.iter().all(|value| left.contains(value))
    }

    /// Counts occurrences of values.
    #[must_use]
    pub fn count_map<T>(values: impl IntoIterator<Item = T>) -> HashMap<T, usize>
    where
        T: Eq + Hash,
    {
        IterUtil::count_map(values)
    }

    /// Joins displayed values.
    #[must_use]
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String {
        IterUtil::join(values, delimiter)
    }

    /// Joins projected values.
    #[must_use]
    pub fn join_by<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        display: impl FnMut(T) -> String,
    ) -> String {
        IterUtil::join_by(values, delimiter, display)
    }

    /// Joins values with a per-element prefix and suffix.
    #[must_use]
    pub fn join_wrapped<T: Display>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
    ) -> String {
        IterUtil::join_wrapped(values, delimiter, prefix, suffix)
    }

    /// Removes up to `part_size` values from the front of a deque.
    #[must_use]
    pub fn pop_part<T>(values: &mut VecDeque<T>, part_size: usize) -> Vec<T> {
        (0..part_size.min(values.len()))
            .filter_map(|_| values.pop_front())
            .collect()
    }

    /// Hutool-compatible `anyMatch`: empty input returns `false`.
    #[must_use]
    pub fn any_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        values.iter().any(matcher)
    }

    /// Hutool-compatible `allMatch`: empty input returns `false`.
    #[must_use]
    pub fn all_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        !values.is_empty() && values.iter().all(matcher)
    }

    /// Collects values into a hash set.
    #[must_use]
    pub fn new_hash_set<T>(values: impl IntoIterator<Item = T>) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        values.into_iter().collect()
    }

    /// Collects values into an insertion-ordered set.
    #[must_use]
    pub fn new_linked_hash_set<T>(values: impl IntoIterator<Item = T>) -> IndexSet<T>
    where
        T: Eq + Hash,
    {
        values.into_iter().collect()
    }

    /// Collects values into a vector.
    #[must_use]
    pub fn new_array_list<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Collects values into a double-ended queue.
    #[must_use]
    pub fn new_linked_list<T>(values: impl IntoIterator<Item = T>) -> VecDeque<T> {
        values.into_iter().collect()
    }

    /// Creates an atomically shared copy-on-write list.
    #[must_use]
    pub fn new_copy_on_write_array_list<T>(values: impl IntoIterator<Item = T>) -> Arc<Vec<T>> {
        Arc::new(values.into_iter().collect())
    }

    /// Obtains the mutable copy-on-write view, cloning only when the list is shared.
    pub fn copy_on_write_mut<T: Clone>(values: &mut Arc<Vec<T>>) -> &mut Vec<T> {
        Arc::make_mut(values)
    }

    /// Creates a bounded blocking queue.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when capacity is zero.
    pub fn new_blocking_queue<T>(capacity: usize) -> Result<BlockingQueue<T>> {
        if capacity == 0 {
            return Err(CoreError::InvalidArgument {
                name: "capacity",
                reason: "must be greater than zero",
            });
        }
        let (sender, receiver) = sync_channel(capacity);
        Ok(BlockingQueue {
            sender,
            receiver: Mutex::new(receiver),
        })
    }

    /// Creates an empty concrete collection without runtime reflection.
    #[must_use]
    pub fn create<T>(kind: CollectionKind) -> CreatedCollection<T> {
        match kind {
            CollectionKind::List => CreatedCollection::List(Vec::new()),
            CollectionKind::Deque => CreatedCollection::Deque(VecDeque::new()),
            CollectionKind::Set => CreatedCollection::Set(HashSet::new()),
            CollectionKind::OrderedSet => CreatedCollection::OrderedSet(IndexSet::new()),
            CollectionKind::SortedSet => CreatedCollection::SortedSet(BTreeSet::new()),
        }
    }

    /// Stable equality-based deduplication.
    #[must_use]
    pub fn distinct<T>(values: impl IntoIterator<Item = T>) -> Vec<T>
    where
        T: Eq + Hash,
    {
        values
            .into_iter()
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect()
    }

    /// Stable deduplication by a derived key, optionally replacing earlier values.
    #[must_use]
    pub fn distinct_by<T, K>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        replace: bool,
    ) -> Vec<T>
    where
        K: Eq + Hash,
    {
        let mut unique = IndexMap::new();
        for value in values {
            let key = key_of(&value);
            if replace || !unique.contains_key(&key) {
                unique.insert(key, value);
            }
        }
        unique.into_values().collect()
    }

    /// Returns an owned Hutool-style signed sub-list.
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>> {
        ListUtil::sub(values, start, end, step)
    }

    /// Splits values into owned fixed-size segments.
    pub fn split<T: Clone>(values: &[T], size: usize) -> Result<Vec<Vec<T>>> {
        if size == 0 {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "must be greater than zero",
            });
        }
        Ok(values.chunks(size).map(<[T]>::to_vec).collect())
    }

    /// Edits values and drops rejected results.
    #[must_use]
    pub fn edit<T, U>(
        values: impl IntoIterator<Item = T>,
        editor: impl FnMut(T) -> Option<U>,
    ) -> Vec<U> {
        IterUtil::edit(values, editor)
    }

    /// Filters values into a new vector.
    #[must_use]
    pub fn filter_new<T>(
        values: impl IntoIterator<Item = T>,
        predicate: impl FnMut(&T) -> bool,
    ) -> Vec<T> {
        IterUtil::filter_to_list(values, predicate)
    }

    /// Retains accepted values in place.
    pub fn filter<T>(values: &mut Vec<T>, predicate: impl FnMut(&T) -> bool) {
        values.retain(predicate);
    }

    /// Removes all values included in `removed`.
    pub fn remove_any<T>(values: &mut Vec<T>, removed: &[T])
    where
        T: Eq + Hash,
    {
        let removed: HashSet<_> = removed.iter().collect();
        values.retain(|value| !removed.contains(value));
    }

    /// Removes absent optional values.
    pub fn remove_none<T>(values: &mut Vec<Option<T>>) {
        values.retain(Option::is_some);
    }

    /// Removes empty strings.
    pub fn remove_empty(values: &mut Vec<String>) {
        values.retain(|value| !value.is_empty());
    }

    /// Removes blank strings.
    pub fn remove_blank(values: &mut Vec<String>) {
        values.retain(|value| !value.trim().is_empty());
    }

    /// Moves matching elements out of a vector and returns them.
    #[must_use]
    pub fn remove_with_add_if<T>(
        values: &mut Vec<T>,
        mut predicate: impl FnMut(&T) -> bool,
    ) -> Vec<T> {
        let mut retained = Vec::with_capacity(values.len());
        let mut removed = Vec::new();
        for value in values.drain(..) {
            if predicate(&value) {
                removed.push(value);
            } else {
                retained.push(value);
            }
        }
        *values = retained;
        removed
    }
}

impl CollUtil {
    /// Maps values while preserving or dropping explicit absence.
    #[must_use]
    pub fn map_optional<T, U>(
        values: impl IntoIterator<Item = Option<T>>,
        ignore_none: bool,
        mut mapper: impl FnMut(Option<T>) -> Option<U>,
    ) -> Vec<Option<U>> {
        values
            .into_iter()
            .filter_map(|value| {
                if ignore_none && value.is_none() {
                    None
                } else {
                    let mapped_value = mapper(value);
                    (!ignore_none || mapped_value.is_some()).then_some(mapped_value)
                }
            })
            .collect()
    }

    /// Extracts explicit fields using a compile-time accessor instead of reflection.
    #[must_use]
    pub fn field_values<T, V>(
        values: impl IntoIterator<Item = T>,
        accessor: impl FnMut(T) -> V,
    ) -> Vec<V> {
        values.into_iter().map(accessor).collect()
    }

    /// Indexes objects by an explicit field accessor; later keys replace earlier ones.
    #[must_use]
    pub fn field_value_map<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, T>
    where
        K: Eq + Hash,
    {
        IterUtil::field_value_map(values, key_of)
    }

    /// Maps one explicit field to another without runtime reflection.
    #[must_use]
    pub fn field_value_as_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(&T) -> V,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        values
            .into_iter()
            .map(|value| (key_of(&value), value_of(&value)))
            .collect()
    }

    /// Finds the first matching owned value.
    pub fn find_one<T>(
        values: impl IntoIterator<Item = T>,
        mut matcher: impl FnMut(&T) -> bool,
    ) -> Option<T> {
        values.into_iter().find(&mut matcher)
    }

    /// Finds the first value whose projected field equals `expected`.
    pub fn find_one_by<T, V: PartialEq>(
        values: impl IntoIterator<Item = T>,
        mut field_of: impl FnMut(&T) -> V,
        expected: &V,
    ) -> Option<T> {
        values
            .into_iter()
            .find(|value| field_of(value) == *expected)
    }

    /// Counts matching values.
    #[must_use]
    pub fn count<T>(values: impl IntoIterator<Item = T>, matcher: impl FnMut(&T) -> bool) -> usize {
        values.into_iter().filter(matcher).count()
    }

    /// Returns the first matching index.
    #[must_use]
    pub fn index_of<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Option<usize> {
        values.iter().position(matcher)
    }

    /// Returns the last matching index.
    #[must_use]
    pub fn last_index_of<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Option<usize> {
        values.iter().rposition(matcher)
    }

    /// Returns every matching index.
    #[must_use]
    pub fn index_of_all<T>(values: &[T], mut matcher: impl FnMut(&T) -> bool) -> Vec<usize> {
        values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| matcher(value).then_some(index))
            .collect()
    }

    /// Returns whether an optional slice is absent or empty.
    #[must_use]
    pub fn is_empty<T>(values: Option<&[T]>) -> bool {
        values.is_none_or(<[T]>::is_empty)
    }

    /// Returns whether an optional slice is present and non-empty.
    #[must_use]
    pub fn is_not_empty<T>(values: Option<&[T]>) -> bool {
        !Self::is_empty(values)
    }

    /// Returns `values` unless empty, otherwise evaluates `default`.
    #[must_use]
    pub fn default_if_empty<T>(values: Vec<T>, default: impl FnOnce() -> Vec<T>) -> Vec<T> {
        if values.is_empty() { default() } else { values }
    }

    /// Returns whether any optional element is absent.
    #[must_use]
    pub fn has_none<T>(values: impl IntoIterator<Item = Option<T>>) -> bool {
        values.into_iter().any(|value| value.is_none())
    }

    /// Zips keys and values to the shorter length; later duplicate keys replace earlier ones.
    #[must_use]
    pub fn zip<K, V>(
        keys: impl IntoIterator<Item = K>,
        values: impl IntoIterator<Item = V>,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        keys.into_iter().zip(values).collect()
    }

    /// Splits delimited key/value strings and zips them.
    #[must_use]
    pub fn zip_strings(keys: &str, values: &str, delimiter: &str) -> HashMap<String, String> {
        Self::zip(
            keys.split(delimiter).map(str::to_owned),
            values.split(delimiter).map(str::to_owned),
        )
    }

    /// Collects entries to a map.
    #[must_use]
    pub fn entries_to_map<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        entries.into_iter().collect()
    }

    /// Creates a sorted set.
    #[must_use]
    pub fn to_tree_set<T: Ord>(values: impl IntoIterator<Item = T>) -> BTreeSet<T> {
        values.into_iter().collect()
    }

    /// Converts an iterable to an owned vector; iterator/enumeration adapters are native iterators.
    #[must_use]
    pub fn to_collection<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Converts row maps to a column map of lists.
    #[must_use]
    pub fn to_list_map<K, V>(rows: impl IntoIterator<Item = HashMap<K, V>>) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut columns = HashMap::<K, Vec<V>>::new();
        for row in rows {
            for (key, value) in row {
                columns.entry(key).or_default().push(value);
            }
        }
        columns
    }

    /// Converts a column map of lists to row maps, omitting exhausted columns.
    #[must_use]
    pub fn to_map_list<K, V>(columns: HashMap<K, Vec<V>>) -> Vec<HashMap<K, V>>
    where
        K: Clone + Eq + Hash,
    {
        let row_count = columns.values().map(Vec::len).max().unwrap_or_default();
        let mut rows: Vec<_> = std::iter::repeat_with(HashMap::new)
            .take(row_count)
            .collect();
        for (key, values) in columns {
            for (index, value) in values.into_iter().enumerate() {
                rows[index].insert(key.clone(), value);
            }
        }
        rows
    }

    /// Maps objects to derived keys and preserves each object as the value.
    #[must_use]
    pub fn to_identity_map<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, T>
    where
        K: Eq + Hash,
    {
        Self::field_value_map(values, key_of)
    }

    /// Maps objects to derived key/value pairs.
    #[must_use]
    pub fn to_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(T) -> V,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        values
            .into_iter()
            .map(|value| (key_of(&value), value_of(value)))
            .collect()
    }

    /// Adds a non-absent value only when it is not already present.
    pub fn add_if_absent<T: PartialEq>(values: &mut Vec<T>, value: Option<T>) -> bool {
        let Some(value) = value else { return false };
        if values.contains(&value) {
            false
        } else {
            values.push(value);
            true
        }
    }

    /// Extends a collection from any iterator.
    pub fn add_all<T>(target: &mut Vec<T>, values: impl IntoIterator<Item = T>) {
        target.extend(values);
    }

    /// Parses Hutool's comma-list string form and appends converted values.
    pub fn add_all_from_str<T, E>(
        target: &mut Vec<T>,
        values: &str,
        mut convert: impl FnMut(&str) -> std::result::Result<T, E>,
    ) -> std::result::Result<(), E> {
        let values = values
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
            .unwrap_or(values);
        target.extend(
            values
                .split(',')
                .map(str::trim)
                .map(&mut convert)
                .collect::<std::result::Result<Vec<_>, _>>()?,
        );
        Ok(())
    }

    /// Appends only values not already present in the target.
    pub fn add_all_if_not_contains<T: Clone + PartialEq>(target: &mut Vec<T>, values: &[T]) {
        for value in values {
            if !target.contains(value) {
                target.push(value.clone());
            }
        }
    }

    /// Gets a value using a signed index; negative positions count from the end.
    #[must_use]
    pub fn get<T>(values: &[T], index: isize) -> Option<&T> {
        let index = if index < 0 {
            values.len().checked_add_signed(index)?
        } else {
            index.unsigned_abs()
        };
        values.get(index)
    }

    /// Gets several signed indices, returning `None` for out-of-range positions.
    #[must_use]
    pub fn get_any<'a, T>(values: &'a [T], indexes: &[isize]) -> Vec<Option<&'a T>> {
        indexes
            .iter()
            .map(|index| Self::get(values, *index))
            .collect()
    }

    /// Gets the first value.
    #[must_use]
    pub fn first<T>(values: &[T]) -> Option<&T> {
        values.first()
    }

    /// Gets the last value.
    #[must_use]
    pub fn last<T>(values: &[T]) -> Option<&T> {
        values.last()
    }

    /// Returns the static element type for non-empty slices.
    #[must_use]
    pub fn element_type<T>(values: &[T]) -> Option<&'static str> {
        IterUtil::element_type(values)
    }

    /// Looks up keys in order, retaining missing entries explicitly as `None`.
    #[must_use]
    pub fn values_of_keys<'a, K, V>(
        map: &'a HashMap<K, V>,
        keys: impl IntoIterator<Item = &'a K>,
    ) -> Vec<Option<&'a V>>
    where
        K: Eq + Hash + 'a,
    {
        keys.into_iter().map(|key| map.get(key)).collect()
    }
}

impl CollUtil {
    /// Sorts all supplied collections and returns one zero-based page.
    pub fn sort_page_all<T: Clone>(
        page_no: usize,
        page_size: usize,
        collections: &[&[T]],
        compare: impl FnMut(&T, &T) -> Ordering,
    ) -> Result<Vec<T>> {
        let mut values = Self::union_all(collections);
        values.sort_by(compare);
        Ok(ListUtil::page(&values, page_no, page_size)?.to_vec())
    }

    /// Returns one zero-based owned page.
    pub fn page<T: Clone>(values: &[T], page_no: usize, page_size: usize) -> Result<Vec<T>> {
        Ok(ListUtil::page(values, page_no, page_size)?.to_vec())
    }

    /// Returns a sorted clone without modifying the input.
    #[must_use]
    pub fn sort<T: Clone>(values: &[T], mut compare: impl FnMut(&T, &T) -> Ordering) -> Vec<T> {
        let mut sorted = values.to_vec();
        sorted.sort_by(&mut compare);
        sorted
    }

    /// Sorts a list in place.
    pub fn sort_in_place<T>(values: &mut [T], compare: impl FnMut(&T, &T) -> Ordering) {
        values.sort_by(compare);
    }

    /// Sorts by an explicit property accessor instead of Java reflection.
    #[must_use]
    pub fn sort_by_property<T: Clone, K: Ord>(
        values: &[T],
        mut property: impl FnMut(&T) -> K,
    ) -> Vec<T> {
        Self::sort(values, |left, right| property(left).cmp(&property(right)))
    }

    /// Sorts strings by a caller-selected collation key (for example a pinyin crate).
    #[must_use]
    pub fn sort_strings_by_key<K: Ord>(
        values: &[String],
        mut collation_key: impl FnMut(&str) -> K,
    ) -> Vec<String> {
        Self::sort(values, |left, right| {
            collation_key(left).cmp(&collation_key(right))
        })
    }

    /// Converts a map to a key-ordered map.
    #[must_use]
    pub fn sort_map<K: Ord, V>(values: impl IntoIterator<Item = (K, V)>) -> BTreeMap<K, V> {
        values.into_iter().collect()
    }

    /// Sorts entries with an arbitrary comparator and preserves that order.
    #[must_use]
    pub fn sort_entries<K, V>(
        entries: impl IntoIterator<Item = (K, V)>,
        mut compare: impl FnMut(&(K, V), &(K, V)) -> Ordering,
    ) -> IndexMap<K, V>
    where
        K: Eq + Hash,
    {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_by(&mut compare);
        entries.into_iter().collect()
    }

    /// Sorts entries by their values.
    #[must_use]
    pub fn sort_entries_by_value<K, V: Ord>(
        entries: impl IntoIterator<Item = (K, V)>,
    ) -> Vec<(K, V)> {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_by(|left, right| left.1.cmp(&right.1));
        entries
    }

    /// Visits values with a stable zero-based index.
    pub fn for_each<T>(values: impl IntoIterator<Item = T>, mut consumer: impl FnMut(T, usize)) {
        values
            .into_iter()
            .enumerate()
            .for_each(|(index, value)| consumer(value, index));
    }

    /// Visits map entries with a stable zero-based iteration index.
    pub fn for_each_map<K, V>(
        values: impl IntoIterator<Item = (K, V)>,
        mut consumer: impl FnMut(K, V, usize),
    ) {
        values
            .into_iter()
            .enumerate()
            .for_each(|(index, (key, value))| consumer(key, value, index));
    }

    /// Places values in sparse groups selected by a non-negative index.
    pub fn group<T>(
        values: impl IntoIterator<Item = T>,
        mut group_index: impl FnMut(&T) -> usize,
    ) -> Vec<Vec<T>> {
        let mut groups = Vec::<Vec<T>>::new();
        for value in values {
            let index = group_index(&value);
            if groups.len() <= index {
                groups.resize_with(index + 1, Vec::new);
            }
            groups[index].push(value);
        }
        groups
    }

    /// Groups values by first-seen explicit field values instead of reflection.
    #[must_use]
    pub fn group_by_field<T, K>(
        values: impl IntoIterator<Item = T>,
        mut field_of: impl FnMut(&T) -> K,
    ) -> Vec<Vec<T>>
    where
        K: Eq + Hash,
    {
        let mut indexes = HashMap::<K, usize>::new();
        let mut groups = Vec::<Vec<T>>::new();
        for value in values {
            let next_index = indexes.len();
            let index = *indexes.entry(field_of(&value)).or_insert(next_index);
            if groups.len() == index {
                groups.push(Vec::new());
            }
            groups[index].push(value);
        }
        groups
    }

    /// Reverses values in place.
    pub fn reverse<T>(values: &mut [T]) {
        values.reverse();
    }

    /// Returns a reversed clone.
    #[must_use]
    pub fn reverse_new<T: Clone>(values: &[T]) -> Vec<T> {
        values.iter().rev().cloned().collect()
    }

    /// Replaces an existing index or appends when it is out of range.
    pub fn set_or_append<T>(values: &mut Vec<T>, index: usize, value: T) {
        ListUtil::set_or_append(values, index, value);
    }

    /// Returns the union of keys from a collection of maps.
    #[must_use]
    pub fn key_set<K, V>(maps: &[HashMap<K, V>]) -> HashSet<K>
    where
        K: Clone + Eq + Hash,
    {
        maps.iter().flat_map(|map| map.keys().cloned()).collect()
    }

    /// Returns every map value, preserving map iteration order.
    #[must_use]
    pub fn values<K, V: Clone>(maps: &[HashMap<K, V>]) -> Vec<V> {
        maps.iter().flat_map(|map| map.values().cloned()).collect()
    }

    /// Returns the maximum value, or `None` when empty.
    #[must_use]
    pub fn max<T: Ord>(values: &[T]) -> Option<&T> {
        values.iter().max()
    }

    /// Returns the minimum value, or `None` when empty.
    #[must_use]
    pub fn min<T: Ord>(values: &[T]) -> Option<&T> {
        values.iter().min()
    }

    /// Returns a read-only slice view.
    #[must_use]
    pub const fn unmodifiable<T>(values: &[T]) -> &[T] {
        values
    }

    /// Clears every supplied vector.
    pub fn clear<T>(collections: &mut [&mut Vec<T>]) {
        for values in collections.iter_mut() {
            values.clear();
        }
    }

    /// Prepends clones until a minimum length is reached.
    pub fn pad_left<T: Clone>(values: &mut Vec<T>, minimum_len: usize, padding: T) {
        if values.len() < minimum_len {
            let mut prefix = vec![padding; minimum_len - values.len()];
            prefix.append(values);
            *values = prefix;
        }
    }

    /// Appends clones until a minimum length is reached.
    pub fn pad_right<T: Clone>(values: &mut Vec<T>, minimum_len: usize, padding: T) {
        values.resize(minimum_len.max(values.len()), padding);
    }

    /// Lazily transforms a collection using Rust's native iterator view.
    pub fn trans<I, F, U>(values: I, transform: F) -> std::iter::Map<I::IntoIter, F>
    where
        I: IntoIterator,
        F: FnMut(I::Item) -> U,
    {
        values.into_iter().map(transform)
    }

    /// Applies mapped values to elements whose generated key exists in the map.
    pub fn set_value_by_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        map: &HashMap<K, V>,
        mut key_of: impl FnMut(&T) -> K,
        mut apply: impl FnMut(T, &V),
    ) where
        K: Eq + Hash,
    {
        for value in values {
            if let Some(mapped) = map.get(&key_of(&value)) {
                apply(value, mapped);
            }
        }
    }

    /// Counts values in any iterator. Runtime object inspection is unnecessary in Rust.
    #[must_use]
    pub fn size<T>(values: impl IntoIterator<Item = T>) -> usize {
        values.into_iter().count()
    }

    /// Returns whether two iterables have equal values in equal order.
    #[must_use]
    pub fn is_equal_list<T: PartialEq>(
        left: impl IntoIterator<Item = T>,
        right: impl IntoIterator<Item = T>,
    ) -> bool {
        left.into_iter().eq(right)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn even(value: &i32) -> bool {
        value % 2 == 0
    }

    fn identity_optional(value: Option<i32>) -> Option<i32> {
        value
    }

    fn default_values() -> Vec<i32> {
        vec![9]
    }

    fn parse_i32(value: &str) -> std::result::Result<i32, std::num::ParseIntError> {
        value.parse()
    }

    #[test]
    fn coll_util_matches_hutool_collection_algebra_and_construction() {
        assert_eq!(CollUtil::empty_if_none::<i32>(None), []);
        assert_eq!(CollUtil::empty_if_none(Some(&[1, 2])), [1, 2]);

        let left = [1, 1, 2, 3];
        let right = [1, 2, 2, 4];
        assert_eq!(CollUtil::union(&left, &right), [1, 1, 2, 2, 3, 4]);
        assert_eq!(
            CollUtil::union_many(&[&left, &right, &[2, 2, 2]]),
            [1, 1, 2, 2, 2, 3, 4]
        );
        assert_eq!(
            CollUtil::union_distinct(&[&left, &right]),
            IndexSet::from([1, 2, 3, 4])
        );
        assert_eq!(CollUtil::union_all(&[&left, &right]).len(), 8);
        assert_eq!(CollUtil::intersection(&left, &right), [1, 2]);
        assert_eq!(CollUtil::intersection_many(&[&left, &right, &[2, 4]]), [2]);
        assert!(CollUtil::intersection_many::<i32>(&[]).is_empty());
        assert_eq!(
            CollUtil::intersection_distinct(&[&left, &right]),
            IndexSet::from([1, 2])
        );
        assert!(CollUtil::intersection_distinct::<i32>(&[]).is_empty());
        assert_eq!(CollUtil::disjunction(&left, &right), [1, 2, 3, 4]);
        assert_eq!(CollUtil::subtract(&left, &right), [3]);
        assert!(CollUtil::contains(&left, &2));
        assert!(CollUtil::contains_by(&left, even));
        assert!(CollUtil::contains_any(&left, &right));
        assert!(CollUtil::contains_all(&left, &[1, 3]));
        assert_eq!(CollUtil::count_map(left)[&1], 2);
        assert_eq!(CollUtil::join([1, 2], ","), "1,2");
        assert_eq!(
            CollUtil::join_by([1, 2], ":", |value| format!("v{value}")),
            "v1:v2"
        );
        assert_eq!(CollUtil::join_wrapped([1, 2], ",", "[", "]"), "[1],[2]");

        let mut deque = VecDeque::from([1, 2, 3]);
        assert_eq!(CollUtil::pop_part(&mut deque, 2), [1, 2]);
        assert_eq!(CollUtil::pop_part(&mut deque, 9), [3]);
        assert!(CollUtil::any_match(&left, even));
        assert!(!CollUtil::all_match(&[], even));
        assert!(CollUtil::all_match(&[2, 4], even));
        assert_eq!(CollUtil::new_hash_set([1, 1, 2]), HashSet::from([1, 2]));
        assert_eq!(
            CollUtil::new_linked_hash_set([2, 1, 2]),
            IndexSet::from([2, 1])
        );
        assert_eq!(CollUtil::new_array_list(1..=2), [1, 2]);
        assert_eq!(CollUtil::new_linked_list(1..=2), VecDeque::from([1, 2]));
        let mut copy_on_write = CollUtil::new_copy_on_write_array_list([1, 2]);
        let original = Arc::clone(&copy_on_write);
        CollUtil::copy_on_write_mut(&mut copy_on_write).push(3);
        assert_eq!(original.as_slice(), [1, 2]);
        assert_eq!(copy_on_write.as_slice(), [1, 2, 3]);

        assert!(CollUtil::new_blocking_queue::<i32>(0).is_err());
        let queue = CollUtil::new_blocking_queue(1).unwrap();
        queue.send(7).unwrap();
        assert_eq!(queue.recv().unwrap(), 7);

        assert!(matches!(
            CollUtil::create::<i32>(CollectionKind::List),
            CreatedCollection::List(values) if values.is_empty()
        ));
        assert!(matches!(
            CollUtil::create::<i32>(CollectionKind::Deque),
            CreatedCollection::Deque(values) if values.is_empty()
        ));
        assert!(matches!(
            CollUtil::create::<i32>(CollectionKind::Set),
            CreatedCollection::Set(values) if values.is_empty()
        ));
        assert!(matches!(
            CollUtil::create::<i32>(CollectionKind::OrderedSet),
            CreatedCollection::OrderedSet(values) if values.is_empty()
        ));
        assert!(matches!(
            CollUtil::create::<i32>(CollectionKind::SortedSet),
            CreatedCollection::SortedSet(values) if values.is_empty()
        ));

        assert_eq!(CollUtil::distinct([1, 2, 1]), [1, 2]);
        assert_eq!(
            CollUtil::distinct_by([("a", 1), ("a", 2)], |value| value.0, false),
            [("a", 1)]
        );
        assert_eq!(
            CollUtil::distinct_by([("a", 1), ("a", 2)], |value| value.0, true),
            [("a", 2)]
        );
        assert_eq!(CollUtil::sub(&[0, 1, 2, 3], -3, 4, 2).unwrap(), [1, 3]);
        assert_eq!(
            CollUtil::split(&[1, 2, 3], 2).unwrap(),
            [vec![1, 2], vec![3]]
        );
        assert!(CollUtil::split(&[1], 0).is_err());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn coll_util_matches_hutool_edit_search_mapping_and_mutation() {
        assert_eq!(
            CollUtil::edit(0..4, |value| even(&value).then_some(value * 2)),
            [0, 4]
        );
        assert_eq!(CollUtil::filter_new(0..4, even), [0, 2]);
        let mut values = vec![1, 2, 3, 4];
        CollUtil::filter(&mut values, even);
        assert_eq!(values, [2, 4]);
        CollUtil::remove_any(&mut values, &[4]);
        assert_eq!(values, [2]);

        let mut optional = vec![None, Some(1)];
        CollUtil::remove_none(&mut optional);
        assert_eq!(optional, [Some(1)]);
        let mut text = vec![String::new(), " ".into(), "x".into()];
        CollUtil::remove_empty(&mut text);
        assert_eq!(text, [" ", "x"]);
        CollUtil::remove_blank(&mut text);
        assert_eq!(text, ["x"]);
        let mut removable = vec![1, 2, 3, 4];
        assert_eq!(CollUtil::remove_with_add_if(&mut removable, even), [2, 4]);
        assert_eq!(removable, [1, 3]);

        assert_eq!(
            CollUtil::map_optional([None, Some(1)], true, identity_optional),
            [Some(1)]
        );
        assert_eq!(
            CollUtil::map_optional([None, Some(1)], false, identity_optional),
            [None, Some(1)]
        );
        assert_eq!(
            CollUtil::field_values([("a", 1), ("b", 2)], |value| value.1),
            [1, 2]
        );
        assert_eq!(
            CollUtil::field_value_map([("a", 1), ("b", 2)], |value| value.0)["a"],
            ("a", 1)
        );
        assert_eq!(
            CollUtil::field_value_as_map([("a", 1), ("b", 2)], |value| value.0, |value| value.1),
            HashMap::from([("a", 1), ("b", 2)])
        );
        assert_eq!(CollUtil::find_one(0..4, even), Some(0));
        assert_eq!(
            CollUtil::find_one_by([("a", 1), ("b", 2)], |value| value.0, &"b"),
            Some(("b", 2))
        );
        assert_eq!(CollUtil::count(0..5, even), 3);
        assert_eq!(CollUtil::index_of(&[1, 2, 2], even), Some(1));
        assert_eq!(CollUtil::last_index_of(&[1, 2, 2], even), Some(2));
        assert_eq!(CollUtil::index_of_all(&[1, 2, 2], even), [1, 2]);
        assert!(CollUtil::is_empty::<i32>(None));
        assert!(CollUtil::is_empty(Some(&[] as &[i32])));
        assert!(CollUtil::is_not_empty(Some(&[1])));
        assert_eq!(CollUtil::default_if_empty(Vec::new(), default_values), [9]);
        assert_eq!(CollUtil::default_if_empty(vec![1], default_values), [1]);
        assert!(CollUtil::has_none([Some(1), None]));

        assert_eq!(CollUtil::zip(["a", "b"], [1]), HashMap::from([("a", 1)]));
        assert_eq!(
            CollUtil::zip_strings("a,b", "1,2", ","),
            HashMap::from([("a".into(), "1".into()), ("b".into(), "2".into())])
        );
        assert_eq!(
            CollUtil::entries_to_map([("a", 1), ("b", 2)]),
            HashMap::from([("a", 1), ("b", 2)])
        );
        assert_eq!(CollUtil::to_tree_set([2, 1, 2]), BTreeSet::from([1, 2]));
        assert_eq!(CollUtil::to_collection(1..=2), [1, 2]);

        let columns = CollUtil::to_list_map([
            HashMap::from([("a", 1), ("b", 2)]),
            HashMap::from([("a", 3)]),
        ]);
        assert_eq!(columns["a"], [1, 3]);
        let rows = CollUtil::to_map_list(columns);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["a"], 1);
        assert_eq!(rows[1]["a"], 3);
        assert!(CollUtil::to_map_list::<i32, i32>(HashMap::new()).is_empty());
        assert_eq!(
            CollUtil::to_identity_map([("a", 1), ("b", 2)], |value| value.0)["b"],
            ("b", 2)
        );
        assert_eq!(
            CollUtil::to_map([("a", 1), ("b", 2)], |value| value.0, |value| value.1),
            HashMap::from([("a", 1), ("b", 2)])
        );

        let mut added = vec![1];
        assert!(!CollUtil::add_if_absent(&mut added, None));
        assert!(!CollUtil::add_if_absent(&mut added, Some(1)));
        assert!(CollUtil::add_if_absent(&mut added, Some(2)));
        CollUtil::add_all(&mut added, [3, 4]);
        CollUtil::add_all_from_str(&mut added, "[5, 6]", parse_i32).unwrap();
        CollUtil::add_all_from_str(&mut added, "7,8", parse_i32).unwrap();
        assert!(CollUtil::add_all_from_str(&mut added, "bad", parse_i32).is_err());
        CollUtil::add_all_if_not_contains(&mut added, &[8, 9]);
        assert_eq!(added, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(CollUtil::get(&added, 0), Some(&1));
        assert_eq!(CollUtil::get(&added, -1), Some(&9));
        assert_eq!(CollUtil::get(&added, -99), None);
        assert_eq!(CollUtil::get(&added, isize::MAX), None);
        assert_eq!(
            CollUtil::get_any(&added, &[0, -1, 99]),
            [Some(&1), Some(&9), None]
        );
        assert_eq!(CollUtil::first(&added), Some(&1));
        assert_eq!(CollUtil::last(&added), Some(&9));
        assert!(CollUtil::element_type(&added).unwrap().ends_with("i32"));
        assert_eq!(CollUtil::element_type::<i32>(&[]), None);

        let map = HashMap::from([("a", 1)]);
        let keys = ["a", "missing"];
        assert_eq!(CollUtil::values_of_keys(&map, &keys), [Some(&1), None]);
    }

    #[test]
    fn coll_util_matches_hutool_sort_group_views_and_iteration() {
        let values = [3, 1, 2];
        assert_eq!(
            CollUtil::sort_page_all(0, 2, &[&values], Ord::cmp).unwrap(),
            [1, 2]
        );
        assert!(CollUtil::sort_page_all(0, 0, &[&values], Ord::cmp).is_err());
        assert_eq!(CollUtil::page(&values, 1, 2).unwrap(), [2]);
        assert!(CollUtil::page(&values, 0, 0).is_err());
        assert_eq!(CollUtil::sort(&values, Ord::cmp), [1, 2, 3]);
        let mut sorted = values;
        CollUtil::sort_in_place(&mut sorted, Ord::cmp);
        assert_eq!(sorted, [1, 2, 3]);
        assert_eq!(
            CollUtil::sort_by_property(&[("b", 2), ("a", 1)], |value| value.0),
            [("a", 1), ("b", 2)]
        );
        assert_eq!(
            CollUtil::sort_strings_by_key(&["b".into(), "A".into()], str::to_lowercase),
            ["A", "b"]
        );
        assert_eq!(
            CollUtil::sort_map([("b", 2), ("a", 1)])
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            ["a", "b"]
        );
        let sorted_entries =
            CollUtil::sort_entries([("a", 2), ("b", 1)], |left, right| left.1.cmp(&right.1));
        assert_eq!(
            sorted_entries.keys().copied().collect::<Vec<_>>(),
            ["b", "a"]
        );
        assert_eq!(
            CollUtil::sort_entries_by_value([("a", 2), ("b", 1)]),
            [("b", 1), ("a", 2)]
        );

        let indexed = Rc::new(RefCell::new(Vec::new()));
        let indexed_output = Rc::clone(&indexed);
        CollUtil::for_each([10, 20], move |value, index| {
            indexed_output.borrow_mut().push((value, index));
        });
        assert_eq!(*indexed.borrow(), [(10, 0), (20, 1)]);
        let mapped = Rc::new(RefCell::new(Vec::new()));
        let mapped_output = Rc::clone(&mapped);
        CollUtil::for_each_map([("a", 1), ("b", 2)], move |key, value, index| {
            mapped_output.borrow_mut().push((key, value, index));
        });
        assert_eq!(*mapped.borrow(), [("a", 1, 0), ("b", 2, 1)]);

        assert_eq!(
            CollUtil::group(["a", "b", "c"], |value| usize::from(*value == "b")),
            [vec!["a", "c"], vec!["b"]]
        );
        assert_eq!(
            CollUtil::group_by_field([("a", 1), ("b", 1), ("c", 2)], |value| value.1),
            [vec![("a", 1), ("b", 1)], vec![("c", 2)]]
        );
        let mut reversible = vec![1, 2, 3];
        CollUtil::reverse(&mut reversible);
        assert_eq!(reversible, [3, 2, 1]);
        assert_eq!(CollUtil::reverse_new(&reversible), [1, 2, 3]);
        CollUtil::set_or_append(&mut reversible, 1, 9);
        CollUtil::set_or_append(&mut reversible, 99, 4);
        assert_eq!(reversible, [3, 9, 1, 4]);

        let maps = [HashMap::from([("a", 1)]), HashMap::from([("b", 2)])];
        assert_eq!(CollUtil::key_set(&maps), HashSet::from(["a", "b"]));
        let mut map_values = CollUtil::values(&maps);
        map_values.sort_unstable();
        assert_eq!(map_values, [1, 2]);
        assert_eq!(CollUtil::max(&[1, 3, 2]), Some(&3));
        assert_eq!(CollUtil::min(&[1, 3, 2]), Some(&1));
        assert_eq!(CollUtil::max::<i32>(&[]), None);
        assert_eq!(CollUtil::unmodifiable(&values), values);

        let mut one = vec![1];
        let mut two = vec![2];
        CollUtil::clear(&mut [&mut one, &mut two]);
        assert!(one.is_empty() && two.is_empty());
        let mut padded = vec![2];
        CollUtil::pad_left(&mut padded, 3, 1);
        CollUtil::pad_left(&mut padded, 2, 0);
        assert_eq!(padded, [1, 1, 2]);
        CollUtil::pad_right(&mut padded, 5, 3);
        assert_eq!(padded, [1, 1, 2, 3, 3]);
        assert_eq!(
            CollUtil::trans([1, 2], |value| value * 2).collect::<Vec<_>>(),
            [2, 4]
        );

        let applied = Rc::new(RefCell::new(Vec::new()));
        let applied_output = Rc::clone(&applied);
        CollUtil::set_value_by_map(
            ["a", "missing"],
            &HashMap::from([("a", 1)]),
            |value| *value,
            move |value, mapped| applied_output.borrow_mut().push((value, *mapped)),
        );
        assert_eq!(*applied.borrow(), [("a", 1)]);
        assert_eq!(CollUtil::size(0..4), 4);
        assert!(CollUtil::is_equal_list([1, 2], [1, 2]));
        assert!(!CollUtil::is_equal_list([1, 2], [2, 1]));
    }
}
