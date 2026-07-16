//! Hutool-aligned collection types with Rust-native ownership and concurrency.

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt,
    hash::Hash,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering as AtomicOrdering},
    },
};

use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::{CoreError, Result};

/// Advances `index` with compare-and-swap and wraps it into `0..modulo`.
///
/// Like Hutool's `RingIndexUtil`, an index initialized to zero returns one on
/// the first call when the modulo is greater than one.
pub fn ring_next_index(modulo: usize, index: &AtomicUsize) -> Result<usize> {
    if modulo == 0 {
        return Err(CoreError::InvalidArgument {
            name: "modulo",
            reason: "must be greater than zero",
        });
    }
    if modulo == 1 {
        return Ok(0);
    }
    let current = index
        .fetch_update(AtomicOrdering::AcqRel, AtomicOrdering::Relaxed, |current| {
            Some(current.wrapping_add(1) % modulo)
        })
        .unwrap_or_default();
    Ok(current.wrapping_add(1) % modulo)
}

/// Advances a large atomic index and wraps it into `0..modulo`.
pub fn ring_next_u64(modulo: u64, index: &AtomicU64) -> Result<u64> {
    if modulo == 0 {
        return Err(CoreError::InvalidArgument {
            name: "modulo",
            reason: "must be greater than zero",
        });
    }
    if modulo == 1 {
        return Ok(0);
    }
    let current = index
        .fetch_update(AtomicOrdering::AcqRel, AtomicOrdering::Relaxed, |current| {
            Some(current.wrapping_add(1) % modulo)
        })
        .unwrap_or_default();
    Ok(current.wrapping_add(1) % modulo)
}

/// Advances an atomic index using a collection or slice length.
pub fn ring_next_for_len<T>(items: &[T], index: &AtomicUsize) -> Result<usize> {
    ring_next_index(items.len(), index)
}

type Comparator<T> = dyn Fn(&T, &T) -> Ordering + Send + Sync;

/// A bounded priority queue that retains the best `capacity` values.
///
/// Values considered smaller by the comparator are retained, matching
/// Hutool's natural-order behavior.
pub struct BoundedPriorityQueue<T> {
    capacity: usize,
    comparator: Arc<Comparator<T>>,
    values: Vec<T>,
}

impl<T> BoundedPriorityQueue<T> {
    /// Creates a queue using a custom ordering function.
    pub fn with_comparator(
        capacity: usize,
        comparator: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static,
    ) -> Result<Self> {
        if capacity == 0 {
            return Err(CoreError::InvalidArgument {
                name: "capacity",
                reason: "must be greater than zero",
            });
        }
        Ok(Self {
            capacity,
            comparator: Arc::new(comparator),
            values: Vec::with_capacity(capacity),
        })
    }

    /// Offers one value. An inferior value is ignored but still reports success.
    pub fn offer(&mut self, value: T) -> bool {
        if self.values.len() < self.capacity {
            self.values.push(value);
            return true;
        }
        let worst = self
            .values
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| (self.comparator)(left, right))
            .map(|(index, _)| index)
            .unwrap_or_default();
        if (self.comparator)(&value, &self.values[worst]) == Ordering::Less {
            self.values[worst] = value;
        }
        true
    }

    /// Offers every value and reports whether the input was non-empty.
    pub fn extend_values(&mut self, values: impl IntoIterator<Item = T>) -> bool {
        let mut changed = false;
        for value in values {
            self.offer(value);
            changed = true;
        }
        changed
    }

    /// Returns the number of retained values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no values are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the currently worst retained value.
    #[must_use]
    pub fn peek_worst(&self) -> Option<&T> {
        self.values
            .iter()
            .max_by(|left, right| (self.comparator)(left, right))
    }

    /// Removes and returns the currently worst retained value.
    pub fn pop_worst(&mut self) -> Option<T> {
        let index = self
            .values
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| (self.comparator)(left, right))
            .map(|(index, _)| index)?;
        Some(self.values.swap_remove(index))
    }

    /// Returns a comparator-sorted snapshot.
    #[must_use]
    pub fn to_sorted_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut values = self.values.clone();
        values.sort_by(|left, right| (self.comparator)(left, right));
        values
    }

    /// Clears all retained values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<T: Ord + 'static> BoundedPriorityQueue<T> {
    /// Creates a natural-order queue.
    pub fn new(capacity: usize) -> Result<Self> {
        Self::with_comparator(capacity, Ord::cmp)
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for BoundedPriorityQueue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedPriorityQueue")
            .field("capacity", &self.capacity)
            .field("values", &self.to_sorted_vec())
            .finish_non_exhaustive()
    }
}

/// A shareable concurrent hash set.
#[derive(Debug, Default)]
pub struct ConcurrentHashSet<T> {
    values: RwLock<HashSet<T>>,
}

impl<T: Eq + Hash> ConcurrentHashSet<T> {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: RwLock::new(HashSet::new()),
        }
    }

    /// Creates an empty set with capacity for at least `capacity` values.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: RwLock::new(HashSet::with_capacity(capacity)),
        }
    }

    /// Inserts a value and reports whether it was newly added.
    pub fn insert(&self, value: T) -> bool {
        self.values.write().insert(value)
    }

    /// Returns whether the set contains `value`.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.values.read().contains(value)
    }

    /// Removes a value and reports whether it existed.
    pub fn remove(&self, value: &T) -> bool {
        self.values.write().remove(value)
    }

    /// Returns the current element count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.read().len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.read().is_empty()
    }

    /// Clears the set.
    pub fn clear(&self) {
        self.values.write().clear();
    }

    /// Returns a point-in-time snapshot of the values.
    #[must_use]
    pub fn snapshot(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.values.read().iter().cloned().collect()
    }
}

impl<T: Eq + Hash> FromIterator<T> for ConcurrentHashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            values: RwLock::new(iter.into_iter().collect()),
        }
    }
}

/// A set whose uniqueness is determined by a caller-provided key function.
pub struct UniqueKeySet<K, V, F> {
    values: IndexMap<K, V>,
    key_of: F,
}

impl<K, V, F> UniqueKeySet<K, V, F>
where
    K: Eq + Hash,
    F: Fn(&V) -> K,
{
    /// Creates an empty set.
    #[must_use]
    pub fn new(key_of: F) -> Self {
        Self {
            values: IndexMap::new(),
            key_of,
        }
    }

    /// Creates an empty set with initial capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize, key_of: F) -> Self {
        Self {
            values: IndexMap::with_capacity(capacity),
            key_of,
        }
    }

    /// Creates a set from values, replacing earlier duplicate keys.
    #[must_use]
    pub fn from_values(key_of: F, values: impl IntoIterator<Item = V>) -> Self {
        let mut set = Self::new(key_of);
        set.extend_replace(values);
        set
    }

    /// Inserts or replaces a value and reports whether its key was new.
    pub fn insert(&mut self, value: V) -> bool {
        let key = (self.key_of)(&value);
        self.values.insert(key, value).is_none()
    }

    /// Inserts a value only when its generated key is absent.
    pub fn insert_if_absent(&mut self, value: V) -> bool {
        let key = (self.key_of)(&value);
        if self.values.contains_key(&key) {
            return false;
        }
        self.values.insert(key, value);
        true
    }

    /// Inserts every absent value and reports whether at least one was added.
    pub fn extend_if_absent(&mut self, values: impl IntoIterator<Item = V>) -> bool {
        let mut changed = false;
        for value in values {
            changed = self.insert_if_absent(value) || changed;
        }
        changed
    }

    /// Inserts all values, replacing values with duplicate generated keys.
    pub fn extend_replace(&mut self, values: impl IntoIterator<Item = V>) {
        for value in values {
            self.insert(value);
        }
    }

    /// Returns whether a value with the same generated key exists.
    #[must_use]
    pub fn contains_value(&self, value: &V) -> bool {
        self.values.contains_key(&(self.key_of)(value))
    }

    /// Removes and returns the value with the same generated key.
    pub fn remove_value(&mut self, value: &V) -> Option<V> {
        self.values.shift_remove(&(self.key_of)(value))
    }

    /// Gets a value by its generated key.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }

    /// Iterates values in first-key insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &V> {
        self.values.values()
    }

    /// Returns the number of unique keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no values are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<K: Clone, V: Clone, F: Clone> Clone for UniqueKeySet<K, V, F> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            key_of: self.key_of.clone(),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug, F> fmt::Debug for UniqueKeySet<K, V, F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UniqueKeySet")
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, atomic::AtomicUsize},
        thread,
    };

    use super::*;

    #[test]
    fn ring_indices_match_hutool_progression_and_validate_bounds() {
        let index = AtomicUsize::new(0);
        assert_eq!(ring_next_index(3, &index).unwrap(), 1);
        assert_eq!(ring_next_index(3, &index).unwrap(), 2);
        assert_eq!(ring_next_index(3, &index).unwrap(), 0);
        assert_eq!(ring_next_for_len(&[1], &index).unwrap(), 0);
        assert!(ring_next_for_len::<u8>(&[], &index).is_err());

        let large = AtomicU64::new(u64::MAX);
        assert_eq!(ring_next_u64(7, &large).unwrap(), 0);
        assert_eq!(ring_next_u64(1, &large).unwrap(), 0);
        assert!(ring_next_u64(0, &large).is_err());
    }

    #[test]
    fn bounded_priority_queue_keeps_best_values_in_sorted_order() {
        let mut queue = BoundedPriorityQueue::new(3).unwrap();
        assert!(queue.extend_values([5, 1, 3, 2, 9]));
        assert_eq!(queue.to_sorted_vec(), [1, 2, 3]);
        assert_eq!(queue.peek_worst(), Some(&3));
        assert_eq!(queue.pop_worst(), Some(3));
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.capacity(), 3);
        assert!(!queue.is_empty());
        assert!(!queue.extend_values([]));
        queue.clear();
        assert!(queue.is_empty());
        assert_eq!(queue.pop_worst(), None);
        assert!(BoundedPriorityQueue::<i32>::new(0).is_err());

        let mut reverse = BoundedPriorityQueue::with_comparator(2, |a: &i32, b| b.cmp(a)).unwrap();
        reverse.extend_values([1, 4, 2]);
        assert_eq!(reverse.to_sorted_vec(), [4, 2]);
        assert!(format!("{reverse:?}").contains("BoundedPriorityQueue"));
    }

    #[test]
    fn concurrent_hash_set_supports_shared_atomic_updates() {
        let values = Arc::new(ConcurrentHashSet::with_capacity(16));
        let threads: Vec<_> = (0..4)
            .map(|worker| {
                let values = Arc::clone(&values);
                thread::spawn(move || {
                    for value in 0..100 {
                        values.insert(worker * 100 + value);
                    }
                })
            })
            .collect();
        for worker in threads {
            worker.join().unwrap();
        }
        assert_eq!(values.len(), 400);
        assert!(values.contains(&123));
        assert!(values.remove(&123));
        assert!(!values.remove(&123));
        assert_eq!(values.snapshot().len(), 399);
        values.clear();
        assert!(values.is_empty());

        let collected: ConcurrentHashSet<_> = [1, 1, 2].into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert!(ConcurrentHashSet::<i32>::new().is_empty());
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Record {
        id: u32,
        name: &'static str,
    }

    #[test]
    fn unique_key_set_replaces_or_preserves_duplicates_as_requested() {
        let mut values = UniqueKeySet::with_capacity(2, |record: &Record| record.id);
        assert!(values.insert(Record { id: 1, name: "one" }));
        assert!(values.insert(Record { id: 2, name: "two" }));
        assert!(!values.insert(Record {
            id: 2,
            name: "replacement",
        }));
        assert_eq!(values.get(&2).unwrap().name, "replacement");
        assert!(!values.insert_if_absent(Record {
            id: 2,
            name: "ignored",
        }));
        assert!(values.extend_if_absent([
            Record { id: 2, name: "two" },
            Record {
                id: 3,
                name: "three",
            },
        ]));
        assert_eq!(
            values.iter().map(|record| record.id).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert!(values.contains_value(&Record {
            id: 1,
            name: "other",
        }));
        assert_eq!(
            values.remove_value(&Record { id: 2, name: "" }).unwrap().id,
            2
        );
        assert_eq!(values.len(), 2);

        let cloned = values.clone();
        assert_eq!(cloned.len(), 2);
        assert!(format!("{cloned:?}").contains("UniqueKeySet"));
        values.clear();
        assert!(values.is_empty());

        let built = UniqueKeySet::from_values(
            |record: &Record| record.id,
            [Record { id: 7, name: "old" }, Record { id: 7, name: "new" }],
        );
        assert_eq!(built.get(&7).unwrap().name, "new");
    }
}
