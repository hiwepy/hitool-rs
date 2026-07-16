//! Borrowed and streaming collection partitions aligned with Hutool.

use std::iter::Peekable;

use crate::{CoreError, Result};

/// A borrowed view that divides a slice into fixed-size partitions.
///
/// This is the Rust-native counterpart of Hutool's `Partition` and
/// `RandomAccessPartition`: slices already provide constant-time random access.
#[derive(Clone, Copy, Debug)]
pub struct Partition<'a, T> {
    items: &'a [T],
    partition_size: usize,
}

impl<'a, T> Partition<'a, T> {
    /// Creates a partition view.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `partition_size` is zero.
    pub fn new(items: &'a [T], partition_size: usize) -> Result<Self> {
        validate_partition_size(partition_size)?;
        Ok(Self {
            items,
            partition_size,
        })
    }

    /// Returns the number of partitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len().div_ceil(self.partition_size)
    }

    /// Returns whether this view has no partitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns one partition by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        if index >= self.len() {
            return None;
        }
        let start = index * self.partition_size;
        let end = (start + self.partition_size).min(self.items.len());
        Some(&self.items[start..end])
    }

    /// Iterates all partitions without allocating their elements.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'a [T]> + ExactSizeIterator + '_ {
        self.items.chunks(self.partition_size)
    }
}

/// `Partition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessPartition<'a, T> = Partition<'a, T>;

/// A borrowed view that distributes a slice across exactly `limit` partitions.
///
/// Remainder elements are assigned one each to the earliest partitions. When
/// `limit` is greater than the slice length, trailing partitions are empty.
#[derive(Clone, Copy, Debug)]
pub struct AvgPartition<'a, T> {
    items: &'a [T],
    limit: usize,
}

impl<'a, T> AvgPartition<'a, T> {
    /// Creates an average partition view.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `limit` is zero.
    pub fn new(items: &'a [T], limit: usize) -> Result<Self> {
        if limit == 0 {
            return Err(CoreError::InvalidArgument {
                name: "limit",
                reason: "must be greater than zero",
            });
        }
        Ok(Self { items, limit })
    }

    /// Returns the requested partition count, including empty trailing views.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.limit
    }

    /// Average partitions are never empty because `limit` must be positive.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns one average partition by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        if index >= self.limit {
            return None;
        }
        let base_size = self.items.len() / self.limit;
        let remainder = self.items.len() % self.limit;
        let start = index * base_size + index.min(remainder);
        let end = start + base_size + usize::from(index < remainder);
        Some(&self.items[start..end])
    }

    /// Iterates exactly `limit` partitions.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'a [T]> + ExactSizeIterator + '_ {
        (0..self.limit).map(|index| {
            // Every value in this range is valid by construction.
            self.get(index).unwrap_or(&self.items[0..0])
        })
    }
}

/// `AvgPartition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessAvgPartition<'a, T> = AvgPartition<'a, T>;

/// An iterator adapter that collects source items into fixed-size vectors.
pub struct PartitionIter<I>
where
    I: Iterator,
{
    source: Peekable<I>,
    partition_size: usize,
}

impl<I> PartitionIter<I>
where
    I: Iterator,
{
    /// Creates a streaming partition adapter.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `partition_size` is zero.
    pub fn new(source: I, partition_size: usize) -> Result<Self> {
        validate_partition_size(partition_size)?;
        Ok(Self {
            source: source.peekable(),
            partition_size,
        })
    }

    /// Reports whether another partition is available without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.source.peek().is_some()
    }
}

impl<I> Iterator for PartitionIter<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.source.next()?;
        let mut partition = Vec::with_capacity(self.partition_size);
        partition.push(first);
        partition.extend(self.source.by_ref().take(self.partition_size - 1));
        Some(partition)
    }
}

fn validate_partition_size(partition_size: usize) -> Result<()> {
    if partition_size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "partition_size",
            reason: "must be greater than zero",
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_exposes_borrowed_random_access_chunks() {
        let values = [1, 2, 3, 4, 5];
        let partitions = Partition::new(&values, 2).unwrap();
        assert_eq!(partitions.len(), 3);
        assert!(!partitions.is_empty());
        assert_eq!(partitions.get(0), Some(&[1, 2][..]));
        assert_eq!(partitions.get(2), Some(&[5][..]));
        assert_eq!(partitions.get(3), None);
        assert_eq!(
            partitions.iter().collect::<Vec<_>>(),
            [&[1, 2][..], &[3, 4], &[5]]
        );

        let random: RandomAccessPartition<'_, _> = partitions;
        assert_eq!(random.get(1), Some(&[3, 4][..]));

        let empty = Partition::<i32>::new(&[], 3).unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(Partition::<i32>::new(&[], 0).is_err());
    }

    #[test]
    fn average_partition_distributes_remainder_and_empty_tails() {
        let values = [1, 2, 3, 4];
        let average = AvgPartition::new(&values, 3).unwrap();
        assert_eq!(average.len(), 3);
        assert!(!average.is_empty());
        assert_eq!(
            average.iter().collect::<Vec<_>>(),
            [&[1, 2][..], &[3], &[4]]
        );
        assert_eq!(average.get(3), None);

        let random: RandomAccessAvgPartition<'_, _> = AvgPartition::new(&values, 5).unwrap();
        assert_eq!(
            random.iter().collect::<Vec<_>>(),
            [&[1][..], &[2], &[3], &[4], &[]]
        );
        assert_eq!(
            AvgPartition::<i32>::new(&[], 2)
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            [&[][..], &[]]
        );
        assert!(AvgPartition::<i32>::new(&[], 0).is_err());
    }

    #[test]
    fn partition_iter_preserves_order_tail_and_lookahead() {
        let mut partitions = PartitionIter::new(1..=5, 2).unwrap();
        assert!(partitions.has_next());
        assert_eq!(partitions.next(), Some(vec![1, 2]));
        assert_eq!(partitions.next(), Some(vec![3, 4]));
        assert_eq!(partitions.next(), Some(vec![5]));
        assert!(!partitions.has_next());
        assert_eq!(partitions.next(), None);
        assert!(PartitionIter::new(1..=5, 0).is_err());
    }
}
