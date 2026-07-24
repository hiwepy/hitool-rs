use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

mod string_bloom_filter;
mod func_filter;
mod bit_map_bloom_filter;
mod bit_set_bloom_filter;
mod bloom_filter_util;

pub use string_bloom_filter::StringBloomFilter;
pub use func_filter::FuncFilter;
pub use bit_map_bloom_filter::BitMapBloomFilter;
pub use bit_set_bloom_filter::BitSetBloomFilter;
pub use bloom_filter_util::BloomFilterUtil;
