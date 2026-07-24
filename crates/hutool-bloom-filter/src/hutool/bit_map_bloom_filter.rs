use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

use super::string_bloom_filter::StringBloomFilter;

/// An AND-composed group of independently hashed bitmap filters.
pub struct BitMapBloomFilter {
    filters: Vec<Box<dyn StringBloomFilter>>,
}

impl BitMapBloomFilter {
    /// Builds Hutool's default five-filter layout from a size in MiB.
    ///
    /// # Panics
    ///
    /// Panics only if an already validated internal bitmap size is rejected.
    pub fn new(mebibytes: usize) -> Result<Self, BloomFilterError> {
        let per_filter = mebibytes / 5;
        let size = u64::try_from(per_filter)
            .ok()
            .and_then(|value| value.checked_mul(1024 * 1024 * 8))
            .ok_or(BloomFilterError::CapacityOverflow)?;
        if size == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if size > i32::MAX as u64 {
            return Err(BloomFilterError::CapacityOverflow);
        }
        Ok(Self {
            filters: vec![
                Box::new(DefaultFilter::new(size).expect("validated filter size")),
                Box::new(ELFFilter::new(size).expect("validated filter size")),
                Box::new(JSFilter::new(size).expect("validated filter size")),
                Box::new(PJWFilter::new(size).expect("validated filter size")),
                Box::new(SDBMFilter::new(size).expect("validated filter size")),
            ],
        })
    }

    /// Builds a composed filter from explicit hash filters.
    pub fn with_filters(
        filters: Vec<Box<dyn StringBloomFilter>>,
    ) -> Result<Self, BloomFilterError> {
        if filters.is_empty() {
            return Err(BloomFilterError::EmptyCapacity);
        }
        Ok(Self { filters })
    }
}

impl StringBloomFilter for BitMapBloomFilter {
    fn contains(&self, value: &str) -> bool {
        self.filters.iter().all(|filter| filter.contains(value))
    }

    fn add(&mut self, value: &str) -> bool {
        let mut added = false;
        for filter in &mut self.filters {
            added |= filter.add(value);
        }
        added
    }
}
