use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

use super::string_bloom_filter::StringBloomFilter;

/// Hutool's fixed one-to-eight-hash `BitSet` implementation.
#[derive(Debug, Clone)]
pub struct BitSetBloomFilter {
    words: Vec<u64>,
    bit_count: usize,
    expected_elements: u32,
    hash_count: u8,
}

impl BitSetBloomFilter {
    /// Creates a filter with `capacity * hash_count` bits.
    pub fn new(
        capacity: usize,
        expected_elements: usize,
        hash_count: usize,
    ) -> Result<Self, BloomFilterError> {
        if capacity == 0 || expected_elements == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if !(1..=8).contains(&hash_count) {
            return Err(BloomFilterError::InvalidHashFunctionCount);
        }
        let bit_count = capacity
            .checked_mul(hash_count)
            .ok_or(BloomFilterError::CapacityOverflow)?;
        if bit_count > i32::MAX as usize {
            return Err(BloomFilterError::CapacityOverflow);
        }
        let expected_elements =
            u32::try_from(expected_elements).map_err(|_| BloomFilterError::CapacityOverflow)?;
        #[allow(clippy::cast_possible_truncation)]
        let hash_count = hash_count as u8;
        let words = bit_count.div_ceil(64);
        Ok(Self {
            words: vec![0; words],
            bit_count,
            expected_elements,
            hash_count,
        })
    }

    /// Loads one value per line as UTF-8.
    pub fn init(&mut self, path: impl AsRef<Path>) -> Result<(), BloomFilterError> {
        let content = fs::read_to_string(path)?;
        self.add_lines(&content);
        Ok(())
    }

    /// Loads one value per line with an explicit `encoding_rs` decoder.
    pub fn init_with_encoding(
        &mut self,
        path: impl AsRef<Path>,
        encoding: &'static Encoding,
    ) -> Result<(), BloomFilterError> {
        let bytes = fs::read(path)?;
        let (content, had_errors) = encoding.decode_without_bom_handling(&bytes);
        if had_errors {
            return Err(BloomFilterError::InvalidEncoding(encoding.name()));
        }
        self.add_lines(&content);
        Ok(())
    }

    fn add_lines(&mut self, content: &str) {
        for line in content.lines() {
            self.add(line);
        }
    }

    /// Returns Hutool's configured false-positive estimate.
    #[must_use]
    pub fn false_positive_probability(&self) -> f64 {
        let bit_count = u32::try_from(self.bit_count).unwrap_or(u32::MAX);
        (1.0 - (-f64::from(self.hash_count) * f64::from(self.expected_elements)
            / f64::from(bit_count))
        .exp())
        .powi(i32::from(self.hash_count))
    }

    /// Computes the selected number of Hutool hashes.
    #[must_use]
    pub fn create_hashes(value: &str, hash_count: usize) -> Vec<i32> {
        (0..hash_count)
            .map(|index| hashes::indexed_hash(value, index))
            .collect()
    }

    /// Computes one indexed Hutool hash, returning zero outside `0..8`.
    #[must_use]
    pub fn hash(value: &str, index: usize) -> i32 {
        hashes::indexed_hash(value, index)
    }

    fn positions(&self, value: &str) -> impl Iterator<Item = usize> + '_ {
        Self::create_hashes(value, usize::from(self.hash_count))
            .into_iter()
            .map(|hash| {
                let bit_count = i32::try_from(self.bit_count).unwrap_or(i32::MAX);
                usize::try_from((hash % bit_count).unsigned_abs()).unwrap_or(usize::MAX)
            })
    }
}

impl StringBloomFilter for BitSetBloomFilter {
    fn contains(&self, value: &str) -> bool {
        self.positions(value)
            .all(|position| self.words[position / 64] & (1_u64 << (position % 64)) != 0)
    }

    fn add(&mut self, value: &str) -> bool {
        if self.contains(value) {
            return false;
        }
        for position in self.positions(value).collect::<Vec<_>>() {
            self.words[position / 64] |= 1_u64 << (position % 64);
        }
        true
    }
}
