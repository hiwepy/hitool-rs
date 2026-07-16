//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, IterUtil};

/// An iterator whose next value is supplied by a stateful computation.
pub struct ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    compute_next: F,
    next_item: Option<T>,
    finished: bool,
}

impl<T, F> ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    /// Creates a computed iterator.
    #[must_use]
    pub fn new(compute_next: F) -> Self {
        Self {
            compute_next,
            next_item: None,
            finished: false,
        }
    }

    /// Reports whether another computed value is available without consuming it.
    pub fn has_next(&mut self) -> bool {
        if self.next_item.is_some() {
            return true;
        }
        if self.finished {
            return false;
        }
        self.next_item = (self.compute_next)();
        if self.next_item.is_none() {
            self.finished = true;
            return false;
        }
        true
    }

    /// Manually finishes iteration and clears a cached value.
    pub fn finish(&mut self) {
        self.finished = true;
        self.next_item = None;
    }

    /// Resets the finished/cache state; caller-owned computation state is unchanged.
    pub fn reset_state(&mut self) {
        self.finished = false;
        self.next_item = None;
    }
}

impl<T, F> Iterator for ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.has_next().then(|| self.next_item.take()).flatten()
    }
}

/// A buffered reader exposed as a fallible line iterator.
pub struct LineIter<'a> {
    lines: io::Lines<Box<dyn BufRead + 'a>>,
    is_valid: Box<dyn FnMut(&str) -> bool + 'a>,
    finished: bool,
}

impl<'a> LineIter<'a> {
    /// Creates an iterator that returns every line.
    #[must_use]
    pub fn new(reader: impl BufRead + 'a) -> Self {
        Self::with_filter(reader, |_| true)
    }

    /// Creates an iterator that skips lines rejected by `is_valid`.
    #[must_use]
    pub fn with_filter(reader: impl BufRead + 'a, is_valid: impl FnMut(&str) -> bool + 'a) -> Self {
        let reader: Box<dyn BufRead + 'a> = Box::new(reader);
        Self {
            lines: reader.lines(),
            is_valid: Box::new(is_valid),
            finished: false,
        }
    }

    /// Stops reading. Dropping the iterator closes owned reader resources.
    pub fn close(&mut self) {
        self.finished = true;
    }
}

impl Iterator for LineIter<'_> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        loop {
            match self.lines.next() {
                Some(Ok(line)) if (self.is_valid)(&line) => return Some(Ok(line)),
                Some(Ok(_)) => {}
                Some(Err(error)) => {
                    self.finished = true;
                    return Some(Err(error));
                }
                None => {
                    self.finished = true;
                    return None;
                }
            }
        }
    }
}

/// XML node lists map to the same resettable borrowed-slice iterator in Rust.
pub type NodeListIter<'a, T> = ArrayIter<'a, T>;

/// A mutable collection view that transforms values only when observed.
pub struct TransCollection<'a, T, U, F>
where
    F: Fn(&T) -> U,
{
    source: &'a mut Vec<T>,
    transform: F,
    target: PhantomData<fn() -> U>,
}

impl<'a, T, U, F> TransCollection<'a, T, U, F>
where
    F: Fn(&T) -> U,
{
    /// Creates a live transforming view over `source`.
    pub fn new(source: &'a mut Vec<T>, transform: F) -> Self {
        Self {
            source,
            transform,
            target: PhantomData,
        }
    }

    /// Iterates lazily transformed values.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = U> + '_ {
        self.source.iter().map(&self.transform)
    }

    /// Clears the source collection.
    pub fn clear(&mut self) {
        self.source.clear();
    }

    /// Returns whether the source is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    /// Returns the source length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// Visits every lazily transformed value.
    pub fn for_each(&self, action: impl FnMut(U)) {
        self.iter().for_each(action);
    }

    /// Removes source values whose transformed form matches `predicate`.
    pub fn remove_if(&mut self, mut predicate: impl FnMut(&U) -> bool) -> bool {
        let original_len = self.source.len();
        let transform = &self.transform;
        self.source.retain(|value| !predicate(&transform(value)));
        self.source.len() != original_len
    }
}

/// Rust's lazy `Map` iterator is the counterpart of Hutool's spliterator view.
pub type TransSpliterator<I, F> = std::iter::Map<I, F>;

/// Factory for transformed spliterator/iterator views.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpliteratorUtil;

impl SpliteratorUtil {
    /// Lazily transforms an iterator while preserving its size hint and splitting traits.
    pub fn trans<I, F, U>(source: I, transform: F) -> TransSpliterator<I, F>
    where
        I: Iterator,
        F: FnMut(I::Item) -> U,
    {
        source.map(transform)
    }
}

/// Legacy alias retained for Hutool's `CollectionUtil extends CollUtil` surface.
pub type CollectionUtil = IterUtil;

#[cfg(test)]
mod tests {
    use std::{cell::Cell, io::Cursor, rc::Rc};

    use crate::ResettableIter;

    use super::*;

    #[test]
    fn compute_iter_caches_finishes_and_resets_state() {
        let state = Rc::new(Cell::new(0));
        let compute_state = Rc::clone(&state);
        let mut values = ComputeIter::new(move || {
            let value = compute_state.get();
            compute_state.set(value + 1);
            (value < 2).then_some(value)
        });
        assert!(values.has_next());
        assert!(values.has_next());
        assert_eq!(values.next(), Some(0));
        values.finish();
        assert!(!values.has_next());
        state.set(0);
        values.reset_state();
        assert_eq!(values.next(), Some(0));
        assert_eq!(values.next(), Some(1));
        assert_eq!(values.next(), None);
        assert!(!values.has_next());
    }

    #[test]
    fn line_iter_filters_closes_and_propagates_utf8_errors() {
        let reader = Cursor::new("one\n\ntwo\n");
        let mut lines = LineIter::with_filter(reader, |line: &str| !line.is_empty());
        assert_eq!(lines.next().unwrap().unwrap(), "one");
        assert_eq!(lines.next().unwrap().unwrap(), "two");
        assert!(lines.next().is_none());

        let mut all = LineIter::new(Cursor::new("\nvalue"));
        assert_eq!(all.next().unwrap().unwrap(), "");
        all.close();
        assert!(all.next().is_none());

        let mut invalid = LineIter::new(Cursor::new(vec![0xff, b'\n']));
        assert!(invalid.next().unwrap().is_err());
        assert!(invalid.next().is_none());
    }

    #[test]
    fn node_transforming_and_spliterator_views_preserve_source_semantics() {
        let nodes = ["a", "b"];
        let mut node_iter: NodeListIter<'_, _> = ArrayIter::new(&nodes);
        assert_eq!(node_iter.next(), Some(&"a"));
        node_iter.reset();
        assert_eq!(node_iter.collect::<Vec<_>>(), [&"a", &"b"]);

        let mut source = vec![1, 2, 3];
        {
            let mut transformed = TransCollection::new(&mut source, |value| value * 10);
            assert_eq!(transformed.len(), 3);
            assert!(!transformed.is_empty());
            assert_eq!(transformed.iter().collect::<Vec<_>>(), [10, 20, 30]);
            let mut total = 0;
            transformed.for_each(|value| total += value);
            assert_eq!(total, 60);
            assert!(transformed.remove_if(|value| *value == 20));
            assert!(!transformed.remove_if(|value| *value == 99));
            transformed.clear();
            assert!(transformed.is_empty());
        }
        assert!(source.is_empty());

        assert_eq!(
            SpliteratorUtil::trans(1..=3, |value| value.to_string()).collect::<Vec<_>>(),
            ["1", "2", "3"]
        );
        assert!(CollectionUtil::is_empty::<i32>(&[]));
        assert!(format!("{SpliteratorUtil:?}").contains("SpliteratorUtil"));
    }
}
