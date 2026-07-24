use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

use super::bit_map_bloom_filter::BitMapBloomFilter;
use super::bit_set_bloom_filter::BitSetBloomFilter;

/// Constructors corresponding to Hutool's `BloomFilterUtil`.
pub struct BloomFilterUtil;

impl BloomFilterUtil {
    /// Creates a `BitSetBloomFilter`.
    pub fn create_bit_set(
        capacity: usize,
        expected_elements: usize,
        hash_count: usize,
    ) -> Result<BitSetBloomFilter, BloomFilterError> {
        BitSetBloomFilter::new(capacity, expected_elements, hash_count)
    }

    /// Creates Hutool's default bitmap composition.
    pub fn create_bit_map(mebibytes: usize) -> Result<BitMapBloomFilter, BloomFilterError> {
        BitMapBloomFilter::new(mebibytes)
    }
}
