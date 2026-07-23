//! Iterator and collector utilities corresponding to Hutool's `core.stream` package.

use crate::{CoreError, Result};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::hash::Hash;
use std::path::Path;

type Supplier<A> = dyn Fn() -> A;
type Accumulator<T, A> = dyn Fn(&mut A, T);
type Combiner<A> = dyn Fn(A, A) -> A;
type Finisher<A, R> = dyn Fn(A) -> R;

/// Behavioral flags exposed by [`SimpleCollector`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CollectorCharacteristic {
    /// A single accumulator may be shared by concurrent producers.
    Concurrent,
    /// Encounter order does not affect the result.
    Unordered,
    /// The accumulator is already the final result type.
    IdentityFinish,
}

/// A reusable, explicit collector made from supplier, accumulator, combiner,
/// and finisher functions.
pub struct SimpleCollector<T, A, R> {
    supplier: Box<Supplier<A>>,
    accumulator: Box<Accumulator<T, A>>,
    combiner: Box<Combiner<A>>,
    finisher: Box<Finisher<A, R>>,
    characteristics: BTreeSet<CollectorCharacteristic>,
}

impl<T, A, R> SimpleCollector<T, A, R> {
    /// Creates a collector with an explicit finishing operation.
    pub fn new<S, Acc, Comb, Fin>(
        supplier: S,
        accumulator: Acc,
        combiner: Comb,
        finisher: Fin,
        characteristics: impl IntoIterator<Item = CollectorCharacteristic>,
    ) -> Self
    where
        S: Fn() -> A + 'static,
        Acc: Fn(&mut A, T) + 'static,
        Comb: Fn(A, A) -> A + 'static,
        Fin: Fn(A) -> R + 'static,
    {
        Self {
            supplier: Box::new(supplier),
            accumulator: Box::new(accumulator),
            combiner: Box::new(combiner),
            finisher: Box::new(finisher),
            characteristics: characteristics.into_iter().collect(),
        }
    }

    /// Returns the accumulator supplier.
    pub fn supplier(&self) -> &(dyn Fn() -> A + '_) {
        &self.supplier
    }

    /// Returns the element accumulator.
    pub fn accumulator(&self) -> &(dyn Fn(&mut A, T) + '_) {
        &self.accumulator
    }

    /// Returns the partial-result combiner.
    pub fn combiner(&self) -> &(dyn Fn(A, A) -> A + '_) {
        &self.combiner
    }

    /// Returns the final conversion operation.
    pub fn finisher(&self) -> &(dyn Fn(A) -> R + '_) {
        &self.finisher
    }

    /// Returns the immutable collector characteristics.
    #[must_use]
    pub const fn characteristics(&self) -> &BTreeSet<CollectorCharacteristic> {
        &self.characteristics
    }

    /// Collects an iterator sequentially.
    pub fn collect(&self, values: impl IntoIterator<Item = T>) -> R {
        let mut result = (self.supplier)();
        for value in values {
            (self.accumulator)(&mut result, value);
        }
        (self.finisher)(result)
    }

    /// Combines two independently accumulated partial results.
    pub fn combine(&self, left: A, right: A) -> A {
        (self.combiner)(left, right)
    }
}

impl<T, Output> SimpleCollector<T, Output, Output> {
    /// Creates a collector whose accumulator is also its final result.
    pub fn identity<S, Acc, Comb>(supplier: S, accumulator: Acc, combiner: Comb) -> Self
    where
        S: Fn() -> Output + 'static,
        Acc: Fn(&mut Output, T) + 'static,
        Comb: Fn(Output, Output) -> Output + 'static,
    {
        Self::new(
            supplier,
            accumulator,
            combiner,
            |value| value,
            [CollectorCharacteristic::IdentityFinish],
        )
    }
}

/// Null-safe grouping, mapping, joining, and map-reduction operations.
pub struct CollectorUtil;

impl CollectorUtil {
    /// Joins displayable values with `delimiter`.
    #[must_use]
    pub fn joining<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String {
        Self::joining_by(values, delimiter, |value| value.to_string())
    }

    /// Joins mapped values with `delimiter`.
    #[must_use]
    pub fn joining_by<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        mapper: impl Fn(T) -> String,
    ) -> String {
        Self::joining_wrapped(values, delimiter, "", "", mapper)
    }

    /// Joins mapped values and surrounds the result with a prefix and suffix.
    #[must_use]
    pub fn joining_wrapped<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
        mapper: impl Fn(T) -> String,
    ) -> String {
        let mut output = String::from(prefix);
        let mut first = true;
        for value in values {
            if !first {
                output.push_str(delimiter);
            }
            first = false;
            output.push_str(&mapper(value));
        }
        output.push_str(suffix);
        output
    }

    /// Groups values in encounter order.
    #[must_use]
    pub fn grouping_by<T, K>(
        values: impl IntoIterator<Item = T>,
        classifier: impl Fn(&T) -> K,
    ) -> IndexMap<K, Vec<T>>
    where
        K: Eq + Hash,
    {
        let mut grouped = IndexMap::new();
        for value in values {
            grouped
                .entry(classifier(&value))
                .or_insert_with(Vec::new)
                .push(value);
        }
        grouped
    }

    /// Groups optional values without invoking `classifier` for `None`.
    #[must_use]
    pub fn grouping_by_nullable<T, K>(
        values: impl IntoIterator<Item = Option<T>>,
        classifier: impl Fn(&T) -> K,
    ) -> IndexMap<Option<K>, Vec<Option<T>>>
    where
        K: Eq + Hash,
    {
        let mut grouped = IndexMap::new();
        for value in values {
            let key = value.as_ref().map(&classifier);
            grouped.entry(key).or_insert_with(Vec::new).push(value);
        }
        grouped
    }

    /// Groups mapped values in encounter order.
    #[must_use]
    pub fn grouping_map_by<T, K, V>(
        values: impl IntoIterator<Item = T>,
        classifier: impl Fn(&T) -> K,
        mapper: impl Fn(T) -> V,
    ) -> IndexMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut grouped = IndexMap::new();
        for value in values {
            let key = classifier(&value);
            grouped
                .entry(key)
                .or_insert_with(Vec::new)
                .push(mapper(value));
        }
        grouped
    }

    /// Maps values by key and merges duplicate values.
    #[must_use]
    pub fn to_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        key_mapper: impl Fn(&T) -> K,
        value_mapper: impl Fn(T) -> V,
        merge: impl Fn(V, V) -> V,
    ) -> IndexMap<K, V>
    where
        K: Eq + Hash,
    {
        let mut output = IndexMap::new();
        for value in values {
            let key = key_mapper(&value);
            let mapped = value_mapper(value);
            if let Some(previous) = output.shift_remove(&key) {
                output.insert(key, merge(previous, mapped));
            } else {
                output.insert(key, mapped);
            }
        }
        output
    }

    /// Merges `right` into `left`, combining values for duplicate keys.
    #[must_use]
    pub fn map_merger<K, V>(
        mut left: IndexMap<K, V>,
        right: IndexMap<K, V>,
        merge: impl Fn(V, V) -> V,
    ) -> IndexMap<K, V>
    where
        K: Eq + Hash,
    {
        for (key, value) in right {
            if let Some(previous) = left.shift_remove(&key) {
                left.insert(key, merge(previous, value));
            } else {
                left.insert(key, value);
            }
        }
        left
    }

    /// Reduces maps into lists of values grouped by key.
    #[must_use]
    pub fn reduce_list_map<K, V>(
        maps: impl IntoIterator<Item = IndexMap<K, V>>,
    ) -> IndexMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut output = IndexMap::new();
        for map in maps {
            for (key, value) in map {
                output.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
        output
    }
}

/// Constructors and terminal operations for Rust iterators.
pub struct StreamUtil;

impl StreamUtil {
    /// Returns the array's owning iterator.
    pub fn of_array<T, const N: usize>(values: [T; N]) -> std::array::IntoIter<T, N> {
        values.into_iter()
    }

    /// Converts any `IntoIterator` value into its iterator.
    pub fn of<T>(values: T) -> T::IntoIter
    where
        T: IntoIterator,
    {
        values.into_iter()
    }

    /// Generates at most `limit` values, beginning with `seed`.
    pub fn iterate<T>(
        seed: T,
        mut next: impl FnMut(&T) -> T,
        limit: usize,
    ) -> impl Iterator<Item = T> {
        std::iter::successors(Some(seed), move |value| Some(next(value))).take(limit)
    }

    /// Reads UTF-8 text as owned lines.
    pub fn lines(path: impl AsRef<Path>) -> Result<std::vec::IntoIter<String>> {
        Self::lines_with_encoding(path, encoding_rs::UTF_8)
    }

    /// Reads text using a specified encoding as owned lines.
    pub fn lines_with_encoding(
        path: impl AsRef<Path>,
        encoding: &'static Encoding,
    ) -> Result<std::vec::IntoIter<String>> {
        let bytes = std::fs::read(path)?;
        let decoded = encoding
            .decode_without_bom_handling_and_without_replacement(&bytes)
            .ok_or_else(|| CoreError::Codec(format!("input is not valid {}", encoding.name())))?;
        Ok(decoded
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>()
            .into_iter())
    }

    /// Joins displayable iterator values.
    #[must_use]
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String {
        CollectorUtil::joining(values, delimiter)
    }

    /// Joins mapped iterator values.
    #[must_use]
    pub fn join_by<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        mapper: impl Fn(T) -> String,
    ) -> String {
        CollectorUtil::joining_by(values, delimiter, mapper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn stringify(value: i32) -> String {
        value.to_string()
    }

    fn append_x(value: &String) -> String {
        format!("{value}x")
    }

    #[test]
    fn simple_collector_exposes_and_executes_every_stage() {
        let collector = SimpleCollector::new(
            Vec::<i32>::new,
            Vec::push,
            |mut left, right| {
                left.extend(right);
                left
            },
            |values| values.into_iter().sum::<i32>(),
            [
                CollectorCharacteristic::Unordered,
                CollectorCharacteristic::Concurrent,
            ],
        );
        assert_eq!((collector.supplier())(), Vec::<i32>::new());
        let mut partial = vec![1];
        (collector.accumulator())(&mut partial, 2);
        assert_eq!(partial, [1, 2]);
        assert_eq!((collector.combiner())(vec![1], vec![2]), [1, 2]);
        assert_eq!((collector.finisher())(vec![1, 2]), 3);
        assert!(
            collector
                .characteristics()
                .contains(&CollectorCharacteristic::Concurrent)
        );
        assert_eq!(collector.collect([1, 2, 3]), 6);
        assert_eq!(collector.combine(vec![1], vec![2, 3]), [1, 2, 3]);

        let identity = SimpleCollector::identity(Vec::<i32>::new, Vec::push, |mut left, right| {
            left.extend(right);
            left
        });
        assert_eq!(identity.collect([4, 5]), [4, 5]);
        assert_eq!(identity.combine(vec![4], vec![5]), [4, 5]);
        assert!(
            identity
                .characteristics()
                .contains(&CollectorCharacteristic::IdentityFinish)
        );
    }

    #[test]
    fn collector_util_joins_groups_maps_and_reduces() {
        assert_eq!(CollectorUtil::joining([1, 2, 3], ","), "1,2,3");
        assert_eq!(
            CollectorUtil::joining_by([1, 2], ":", |v| (v * 2).to_string()),
            "2:4"
        );
        assert_eq!(
            CollectorUtil::joining_wrapped([1], ",", "[", "]", stringify),
            "[1]"
        );
        assert_eq!(
            CollectorUtil::joining_wrapped(Vec::<i32>::new(), ",", "[", "]", stringify),
            "[]"
        );

        let grouped = CollectorUtil::grouping_by(["a", "bb", "c"], |value| value.len());
        assert_eq!(grouped[&1], ["a", "c"]);
        let calls = Cell::new(0);
        let nullable = CollectorUtil::grouping_by_nullable([None, Some("x")], |value| {
            calls.set(calls.get() + 1);
            value.len()
        });
        assert_eq!(calls.get(), 1);
        assert_eq!(nullable[&None], [None]);
        let mapped = CollectorUtil::grouping_map_by(["a", "bb"], |v| v.len(), str::to_uppercase);
        assert_eq!(mapped[&2], ["BB"]);

        let map = CollectorUtil::to_map(
            ["a", "ant", "bee"],
            |v| v.len(),
            str::to_owned,
            |a, b| format!("{a}+{b}"),
        );
        assert_eq!(map[&3], "ant+bee");
        let left = IndexMap::from([(1, "a".to_owned()), (2, "b".to_owned())]);
        let right = IndexMap::from([(2, "c".to_owned()), (3, "d".to_owned())]);
        let merged = CollectorUtil::map_merger(left, right, |a, b| a + &b);
        assert_eq!(
            merged.values().cloned().collect::<Vec<_>>(),
            ["a", "bc", "d"]
        );
        let reduced = CollectorUtil::reduce_list_map([
            IndexMap::from([(1, "a"), (2, "b")]),
            IndexMap::from([(1, "c")]),
        ]);
        assert_eq!(reduced[&1], ["a", "c"]);
    }

    #[test]
    fn stream_util_builds_reads_and_joins_iterators() {
        assert_eq!(StreamUtil::of_array([1, 2]).collect::<Vec<_>>(), [1, 2]);
        assert_eq!(StreamUtil::of(vec![3, 4]).collect::<Vec<_>>(), [3, 4]);
        assert_eq!(
            StreamUtil::iterate(1, |value| value * 2, 4).collect::<Vec<_>>(),
            [1, 2, 4, 8]
        );
        assert_eq!(StreamUtil::iterate(String::new(), append_x, 0).count(), 0);
        assert_eq!(
            StreamUtil::iterate(String::new(), append_x, 2).collect::<Vec<_>>(),
            ["", "x"]
        );
        assert_eq!(StreamUtil::join([1, 2], "-"), "1-2");
        assert_eq!(
            StreamUtil::join_by([1, 2], "/", |v| (v + 1).to_string()),
            "2/3"
        );

        let directory = tempfile::tempdir().expect("temporary directory");
        let utf8 = directory.path().join("utf8.txt");
        std::fs::write(&utf8, "first\r\nsecond\n").expect("write UTF-8 fixture");
        assert_eq!(
            StreamUtil::lines(&utf8)
                .expect("UTF-8 lines")
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
        let latin1 = directory.path().join("latin1.txt");
        std::fs::write(&latin1, b"caf\xe9").expect("write Latin-1 fixture");
        assert_eq!(
            StreamUtil::lines_with_encoding(&latin1, encoding_rs::WINDOWS_1252)
                .expect("Windows-1252 lines")
                .collect::<Vec<_>>(),
            ["caf\u{e9}"]
        );
        StreamUtil::lines(&latin1).expect_err("invalid UTF-8 must fail");
        let missing = directory.path().join("missing");
        StreamUtil::lines(&missing).expect_err("missing file must fail");
    }
}
