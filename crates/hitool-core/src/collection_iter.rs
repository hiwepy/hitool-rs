//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// An iterator that can return to its configured starting position.
pub trait ResettableIter: Iterator {
    /// Resets iteration state.
    fn reset(&mut self);
}

/// Marker for iterators that are directly usable in `for` loops.
///
/// Rust iterators already implement `IntoIterator`, so this is the zero-cost
/// counterpart of Hutool's `IterableIter` interface.
pub trait IterableIter: Iterator {}

impl<I: Iterator> IterableIter for I {}

/// An iterator over a borrowed slice with Hutool-compatible range handling.
#[derive(Clone, Copy, Debug)]
pub struct ArrayIter<'a, T> {
    items: &'a [T],
    start_index: usize,
    end_index: usize,
    index: usize,
}

impl<'a, T> ArrayIter<'a, T> {
    /// Iterates the complete slice.
    #[must_use]
    pub fn new(items: &'a [T]) -> Self {
        Self::with_bounds(items, 0, -1)
    }

    /// Iterates from `start_index` to the end of the slice.
    #[must_use]
    pub fn from_index(items: &'a [T], start_index: isize) -> Self {
        Self::with_bounds(items, start_index, -1)
    }

    /// Iterates a normalized half-open range.
    ///
    /// Negative or out-of-range end positions mean the slice end. A negative
    /// start, or one at/after the normalized end, is reset to zero, matching
    /// Hutool's `ArrayIter` behavior.
    #[must_use]
    pub fn with_bounds(items: &'a [T], start_index: isize, end_index: isize) -> Self {
        let end_index = usize::try_from(end_index)
            .ok()
            .filter(|end| *end > 0 && *end < items.len())
            .unwrap_or(items.len());
        let start_index = usize::try_from(start_index)
            .ok()
            .filter(|start| *start < end_index)
            .unwrap_or_default();
        Self {
            items,
            start_index,
            end_index,
            index: start_index,
        }
    }

    /// Returns the original borrowed slice.
    #[must_use]
    pub const fn get_array(&self) -> &'a [T] {
        self.items
    }
}

impl<'a, T> Iterator for ArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end_index {
            return None;
        }
        let item = &self.items[self.index];
        self.index += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end_index.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for ArrayIter<'_, T> {}

impl<T> ResettableIter for ArrayIter<'_, T> {
    fn reset(&mut self) {
        self.index = self.start_index;
    }
}

/// An owned snapshot of another iterator.
#[derive(Debug)]
pub struct CopiedIter<T> {
    snapshot: std::iter::Peekable<std::vec::IntoIter<T>>,
}

impl<T> CopiedIter<T> {
    /// Consumes a source and snapshots all remaining values.
    #[must_use]
    pub fn new(source: impl IntoIterator<Item = T>) -> Self {
        Self {
            snapshot: source
                .into_iter()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
        }
    }

    /// Named constructor aligned with Hutool's `copyOf` factory.
    #[must_use]
    pub fn copy_of(source: impl IntoIterator<Item = T>) -> Self {
        Self::new(source)
    }

    /// Reports whether another snapshot value remains.
    pub fn has_next(&mut self) -> bool {
        self.snapshot.peek().is_some()
    }

    /// Returns the remaining snapshot length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.snapshot.len()
    }

    /// Returns whether the snapshot is exhausted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.snapshot.len() == 0
    }
}

impl<T> Iterator for CopiedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.snapshot.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.snapshot.size_hint()
    }
}

impl<T> ExactSizeIterator for CopiedIter<T> {}

/// A filtering iterator with non-consuming lookahead.
pub struct FilterIter<I, P>
where
    I: Iterator,
{
    source: I,
    filter: Option<P>,
    next_item: Option<I::Item>,
}

impl<I, P> FilterIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    /// Creates a filtering adapter. `None` accepts every value.
    #[must_use]
    pub fn new(source: I, filter: Option<P>) -> Self {
        Self {
            source,
            filter,
            next_item: None,
        }
    }

    /// Reports whether an accepted value remains without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.next_item.is_some() || self.set_next_item()
    }

    /// Returns the wrapped iterator.
    #[must_use]
    pub const fn get_iterator(&self) -> &I {
        &self.source
    }

    /// Returns the optional predicate.
    #[must_use]
    pub const fn get_filter(&self) -> Option<&P> {
        self.filter.as_ref()
    }

    fn set_next_item(&mut self) -> bool {
        self.next_item = self
            .source
            .find(|item| self.filter.as_mut().is_none_or(|filter| filter(item)));
        self.next_item.is_some()
    }
}

impl<I, P> Iterator for FilterIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item.take().or_else(|| {
            self.set_next_item();
            self.next_item.take()
        })
    }
}

/// An iterator that transforms values lazily.
pub struct TransIter<I, F>
where
    I: Iterator,
{
    source: std::iter::Peekable<I>,
    transform: F,
}

impl<I, F> TransIter<I, F>
where
    I: Iterator,
{
    /// Creates a transforming iterator.
    #[must_use]
    pub fn new(source: I, transform: F) -> Self {
        Self {
            source: source.peekable(),
            transform,
        }
    }

    /// Reports whether another source value remains.
    pub fn has_next(&mut self) -> bool {
        self.source.peek().is_some()
    }
}

impl<I, F, T> Iterator for TransIter<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(&mut self.transform)
    }
}

/// A dynamically extensible chain of iterators.
pub struct IterChain<'a, T> {
    iterators: VecDeque<Box<dyn Iterator<Item = T> + 'a>>,
    next_item: Option<T>,
}

impl<'a, T> IterChain<'a, T> {
    /// Creates an empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            iterators: VecDeque::new(),
            next_item: None,
        }
    }

    /// Creates a chain from boxed iterators.
    #[must_use]
    pub fn with_iterators(
        iterators: impl IntoIterator<Item = Box<dyn Iterator<Item = T> + 'a>>,
    ) -> Self {
        Self {
            iterators: iterators.into_iter().collect(),
            next_item: None,
        }
    }

    /// Appends an iterator and returns the chain for fluent construction.
    pub fn add_chain(&mut self, iterator: impl Iterator<Item = T> + 'a) -> &mut Self {
        // Rust ownership prevents adding the same iterator object twice.
        self.iterators.push_back(Box::new(iterator));
        self
    }

    /// Returns the number of source iterators not yet fully discarded.
    #[must_use]
    pub fn chain_count(&self) -> usize {
        self.iterators.len()
    }

    /// Reports whether another value remains without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.next_item.is_some() || self.fill_next()
    }

    fn fill_next(&mut self) -> bool {
        while let Some(iterator) = self.iterators.front_mut() {
            if let Some(item) = iterator.next() {
                self.next_item = Some(item);
                return true;
            }
            self.iterators.pop_front();
        }
        false
    }
}

impl<T> Default for IterChain<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for IterChain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_item.is_none() {
            self.fill_next();
        }
        self.next_item.take()
    }
}

/// Rust's `Iterator` is already the equivalent of an `Enumeration` iterator.
pub type EnumerationIter<I> = I;

/// Converting a Rust iterator to an enumeration is a zero-cost identity.
pub type IteratorEnumeration<I> = I;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_iter_normalizes_bounds_and_resets() {
        let values = [0, 1, 2, 3, 4];
        let mut all = ArrayIter::new(&values);
        assert_eq!(all.get_array(), &values);
        assert_eq!(all.len(), 5);
        assert_eq!(all.next(), Some(&0));
        all.reset();
        assert_eq!(all.collect::<Vec<_>>(), [&0, &1, &2, &3, &4]);

        assert_eq!(
            ArrayIter::with_bounds(&values, 2, 4)
                .copied()
                .collect::<Vec<_>>(),
            [2, 3]
        );
        assert_eq!(
            ArrayIter::with_bounds(&values, 4, 2)
                .copied()
                .collect::<Vec<_>>(),
            [0, 1]
        );
        assert_eq!(
            ArrayIter::from_index(&values, -2)
                .copied()
                .collect::<Vec<_>>(),
            values
        );
        assert_eq!(
            ArrayIter::with_bounds(&values, 1, 0)
                .copied()
                .collect::<Vec<_>>(),
            [1, 2, 3, 4]
        );
        assert!(format!("{:?}", ArrayIter::new(&values)).contains("ArrayIter"));
    }

    #[test]
    fn copied_iter_is_an_owned_read_only_snapshot() {
        let mut original = vec![1, 2, 3];
        let mut snapshot = CopiedIter::copy_of(original.iter().copied());
        original.push(4);
        assert!(snapshot.has_next());
        assert_eq!(snapshot.len(), 3);
        assert!(!snapshot.is_empty());
        assert_eq!(snapshot.by_ref().collect::<Vec<_>>(), [1, 2, 3]);
        assert!(!snapshot.has_next());
        assert!(snapshot.is_empty());

        let empty = CopiedIter::<i32>::new([]);
        assert!(format!("{empty:?}").contains("CopiedIter"));
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Number(i32);

    fn is_even(value: &Number) -> bool {
        value.0 % 2 == 0
    }

    #[test]
    fn filter_iter_supports_lookahead_and_optional_filtering() {
        let mut filtered = FilterIter::new(
            [Number(1), Number(2), Number(3), Number(4)].into_iter(),
            Some(is_even as fn(&Number) -> bool),
        );
        assert!(filtered.has_next());
        assert!(filtered.has_next());
        assert!(filtered.get_filter().is_some());
        assert_eq!(filtered.get_iterator().size_hint().0, 2);
        assert_eq!(filtered.next(), Some(Number(2)));
        assert_eq!(filtered.next(), Some(Number(4)));
        assert_eq!(filtered.next(), None);
        assert!(!filtered.has_next());

        let unfiltered = FilterIter::new(
            [Number(1), Number(2), Number(3), Number(4)].into_iter(),
            None::<fn(&Number) -> bool>,
        );
        assert_eq!(
            unfiltered.collect::<Vec<_>>(),
            [Number(1), Number(2), Number(3), Number(4)]
        );
    }

    #[test]
    fn trans_iter_maps_lazily_with_lookahead() {
        let mut transformed = TransIter::new([1, 2].into_iter(), |value: i32| value.to_string());
        assert!(transformed.has_next());
        assert_eq!(transformed.next().as_deref(), Some("1"));
        assert_eq!(transformed.next().as_deref(), Some("2"));
        assert!(!transformed.has_next());
        assert_eq!(transformed.next(), None);
    }

    #[test]
    fn iter_chain_skips_empty_sources_and_accepts_late_additions() {
        let mut chain = IterChain::new();
        assert!(!chain.has_next());
        chain
            .add_chain(std::iter::empty())
            .add_chain([1, 2].into_iter());
        assert_eq!(chain.chain_count(), 2);
        assert!(chain.has_next());
        assert!(chain.has_next());
        assert_eq!(chain.next(), Some(1));
        chain.add_chain([3].into_iter());
        assert_eq!(chain.collect::<Vec<_>>(), [2, 3]);

        let boxed: Vec<Box<dyn Iterator<Item = i32>>> = vec![
            Box::new([4].into_iter()),
            Box::new(std::iter::empty()),
            Box::new([5].into_iter()),
        ];
        assert_eq!(IterChain::with_iterators(boxed).collect::<Vec<_>>(), [4, 5]);
        assert!(IterChain::<i32>::default().next().is_none());
    }

    #[test]
    fn rust_iterators_cover_iterable_and_enumeration_adapters() {
        fn sum_iterable(iter: impl IterableIter<Item = i32>) -> i32 {
            iter.sum()
        }

        let enumeration: EnumerationIter<_> = [1, 2].into_iter();
        let iterator_enumeration: IteratorEnumeration<_> = enumeration;
        assert_eq!(sum_iterable(iterator_enumeration), 3);
    }
}
