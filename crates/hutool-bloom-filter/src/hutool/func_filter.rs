use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

use super::string_bloom_filter::StringBloomFilter;

/// One Hutool-compatible hash function backed by a checked bitmap.
pub struct FuncFilter {
    bitmap: Box<dyn BitMap>,
    size: u64,
    hash: Arc<HashFunction>,
}

impl FuncFilter {
    /// Creates a 32-bit bitmap filter.
    pub fn new(
        max_value: u64,
        hash: impl Fn(&str) -> i64 + Send + Sync + 'static,
    ) -> Result<Self, BloomFilterError> {
        Self::with_machine(max_value, MachineWord::Bits32, hash)
    }

    /// Creates a filter using the selected bitmap word width.
    pub fn with_machine(
        max_value: u64,
        machine: MachineWord,
        hash: impl Fn(&str) -> i64 + Send + Sync + 'static,
    ) -> Result<Self, BloomFilterError> {
        Ok(Self {
            bitmap: create_bitmap(max_value, machine)?,
            size: max_value,
            hash: Arc::new(hash),
        })
    }

    /// Computes the normalized index used by this filter.
    #[must_use]
    pub fn hash(&self, value: &str) -> u64 {
        ((self.hash)(value) % i64::try_from(self.size).unwrap_or(i64::MAX)).unsigned_abs()
    }
}

impl StringBloomFilter for FuncFilter {
    fn contains(&self, value: &str) -> bool {
        self.bitmap
            .contains(self.hash(value))
            .expect("normalized hash is inside the configured bitmap")
    }

    fn add(&mut self, value: &str) -> bool {
        let index = self.hash(value);
        if self
            .bitmap
            .contains(index)
            .expect("normalized hash is inside the configured bitmap")
        {
            return false;
        }
        self.bitmap
            .add(index)
            .expect("normalized hash is inside the configured bitmap");
        true
    }
}

macro_rules! filter {
    ($name:ident, $hash:path) => {
        /// A Hutool-compatible named hash filter.
        pub struct $name(FuncFilter);

        impl $name {
            /// Creates the filter with 32-bit bitmap words.
            pub fn new(max_value: u64) -> Result<Self, BloomFilterError> {
                Self::with_machine(max_value, MachineWord::Bits32)
            }

            /// Creates the filter with an explicit bitmap word width.
            pub fn with_machine(
                max_value: u64,
                machine: MachineWord,
            ) -> Result<Self, BloomFilterError> {
                FuncFilter::with_machine(max_value, machine, |value| i64::from($hash(value)))
                    .map(Self)
            }

            /// Computes the normalized bit index.
            #[must_use]
            pub fn hash(&self, value: &str) -> u64 {
                self.0.hash(value)
            }
        }

        impl StringBloomFilter for $name {
            fn contains(&self, value: &str) -> bool {
                self.0.contains(value)
            }

            fn add(&mut self, value: &str) -> bool {
                self.0.add(value)
            }
        }
    };
}

macro_rules! wide_filter {
    ($name:ident, $hash:path) => {
        /// A Hutool-compatible named 64-bit hash filter.
        pub struct $name(FuncFilter);

        impl $name {
            /// Creates the filter with 32-bit bitmap words.
            pub fn new(max_value: u64) -> Result<Self, BloomFilterError> {
                Self::with_machine(max_value, MachineWord::Bits32)
            }

            /// Creates the filter with an explicit bitmap word width.
            pub fn with_machine(
                max_value: u64,
                machine: MachineWord,
            ) -> Result<Self, BloomFilterError> {
                FuncFilter::with_machine(max_value, machine, $hash).map(Self)
            }

            /// Computes the normalized bit index.
            #[must_use]
            pub fn hash(&self, value: &str) -> u64 {
                self.0.hash(value)
            }
        }

        impl StringBloomFilter for $name {
            fn contains(&self, value: &str) -> bool {
                self.0.contains(value)
            }

            fn add(&mut self, value: &str) -> bool {
                self.0.add(value)
            }
        }
    };
}

type HashFunction = dyn Fn(&str) -> i64 + Send + Sync;
