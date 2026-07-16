//! Probabilistic membership testing aligned with Hutool's bloom-filter module.
//!
//! This crate delegates hashing and bit storage to the audited `bloomfilter`
//! crate and adds a small, typed `HiTool` facade.

#![forbid(unsafe_code)]

use bloomfilter::Bloom;
use std::hash::Hash;
use thiserror::Error;

/// Errors returned while constructing a bloom filter.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum BloomFilterError {
    /// The expected item count must be greater than zero.
    #[error("expected_items must be greater than zero")]
    EmptyCapacity,
    /// The false-positive rate must be strictly between zero and one.
    #[error("false_positive_rate must be between 0 and 1")]
    InvalidFalsePositiveRate,
    /// The underlying filter rejected the computed layout.
    #[error("failed to construct bloom filter: {0}")]
    Engine(&'static str),
}

/// A bloom filter sized from an expected cardinality and false-positive rate.
#[derive(Debug)]
pub struct BloomFilter<T: ?Sized> {
    inner: Bloom<T>,
}

impl<T: Hash + ?Sized> BloomFilter<T> {
    /// Creates a filter for the expected item count and false-positive rate.
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Result<Self, BloomFilterError> {
        if expected_items == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if !(0.0..1.0).contains(&false_positive_rate) || false_positive_rate == 0.0 {
            return Err(BloomFilterError::InvalidFalsePositiveRate);
        }
        Ok(Self {
            inner: Bloom::new_for_fp_rate(expected_items, false_positive_rate)
                .map_err(BloomFilterError::Engine)?,
        })
    }

    /// Inserts a value and returns whether it was probably already present.
    pub fn insert(&mut self, value: &T) -> bool {
        self.inner.check_and_set(value)
    }

    /// Returns `false` when the value is definitely absent and `true` when it
    /// is probably present.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.inner.check(value)
    }

    /// Returns the number of bits allocated by the underlying filter.
    #[must_use]
    pub fn bit_count(&self) -> u64 {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_configuration_and_tracks_membership() {
        assert_eq!(
            BloomFilter::<str>::new(0, 0.01).unwrap_err(),
            BloomFilterError::EmptyCapacity
        );
        assert!(BloomFilter::<str>::new(10, 1.0).is_err());

        let mut filter = BloomFilter::<str>::new(100, 0.001).unwrap();
        assert!(!filter.contains("hutool"));
        assert!(!filter.insert("hutool"));
        assert!(filter.contains("hutool"));
        assert!(filter.insert("hutool"));
        assert!(filter.bit_count() > 0);
    }
}
