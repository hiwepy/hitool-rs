use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

/// Common string filter contract matching Hutool's `BloomFilter` interface.
pub trait StringBloomFilter: Send {
    /// Returns false only when the string is definitely absent.
    fn contains(&self, value: &str) -> bool;
    /// Adds a string and returns false when it was probably already present.
    fn add(&mut self, value: &str) -> bool;
}
