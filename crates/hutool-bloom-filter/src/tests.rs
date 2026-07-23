use super::*;
use encoding_rs::UTF_8;
use std::{fs, io::ErrorKind};

#[test]
fn mature_engine_validates_and_tracks_generic_values() {
    assert_eq!(
        BloomFilter::<str>::new(0, 0.01).unwrap_err(),
        BloomFilterError::EmptyCapacity
    );
    for probability in [0.0, 1.0, -0.1, f64::NAN] {
        assert_eq!(
            BloomFilter::<str>::new(10, probability).unwrap_err(),
            BloomFilterError::InvalidFalsePositiveRate
        );
    }
    let mut filter = BloomFilter::<str>::new(100, 0.001).unwrap();
    assert!(!filter.contains("hutool"));
    assert!(!filter.insert("hutool"));
    assert!(filter.contains("hutool"));
    assert!(filter.insert("hutool"));
    assert!(filter.bit_count() > 0);
}

fn exercise_bitmap(bitmap: &mut dyn BitMap, last: u64) {
    assert!(!bitmap.contains(0).unwrap());
    bitmap.remove(0).unwrap();
    bitmap.add(0).unwrap();
    bitmap.add(1).unwrap();
    bitmap.add(last).unwrap();
    assert!(bitmap.contains(0).unwrap());
    assert!(bitmap.contains(last).unwrap());
    bitmap.remove(0).unwrap();
    assert!(!bitmap.contains(0).unwrap());
    assert!(bitmap.contains(1).unwrap());
    bitmap.remove(1).unwrap();
    let error = bitmap.add(bitmap.capacity()).unwrap_err();
    assert_eq!(
        error,
        BloomFilterError::IndexOutOfBounds {
            index: bitmap.capacity(),
            capacity: bitmap.capacity()
        }
    );
    assert!(bitmap.contains(bitmap.capacity()).is_err());
    assert!(bitmap.remove(bitmap.capacity()).is_err());
}

#[test]
fn checked_sparse_bitmaps_cover_default_32_and_64_bit_layouts() {
    let mut int_map = IntMap::with_size(2);
    assert_eq!(int_map.capacity(), 64);
    exercise_bitmap(&mut int_map, 63);
    assert_eq!(IntMap::new().capacity(), 3_000_000_000);

    let mut long_map = LongMap::with_size(2);
    assert_eq!(long_map.capacity(), 128);
    exercise_bitmap(&mut long_map, 127);
    assert_eq!(LongMap::new().capacity(), 6_000_000_000);

    let empty = IntMap::with_size(0);
    assert!(empty.contains(0).is_err());
}

#[test]
fn hutool_hashes_follow_utf16_wrapping_and_special_length_paths() {
    assert_eq!(hashes::java_default_hash("hutool"), -1_205_917_659);
    assert_eq!(hashes::java_default_hash("😀"), 1_772_899);
    assert_eq!(hashes::rs_hash("hutool"), 143_503_175);
    assert_eq!(hashes::js_hash("hutool"), 566_323_225);
    assert_eq!(hashes::pjw_hash("hutool"), 117_225_052);
    assert_eq!(hashes::elf_hash("hutool"), 117_225_052);
    assert_eq!(hashes::bkdr_hash("hutool"), 1_126_919_705);
    assert_eq!(hashes::sdbm_hash("hutool"), 1_497_192_517);
    assert_eq!(hashes::djb_hash("hutool"), 50_215_936);
    assert_eq!(hashes::ap_hash("hutool"), -1_450_816_438);
    assert_eq!(hashes::fnv_hash("hutool"), 1_100_848_044);
    assert!(hashes::pjw_hash(&"z".repeat(32)) >= 0);
    assert!(hashes::elf_hash(&"z".repeat(32)) >= 0);
    assert_eq!(hashes::tianl_hash(""), 0);
    assert_eq!(hashes::tianl_hash("A"), hashes::tianl_hash("a"));
    assert!(hashes::tianl_hash(&"x".repeat(97)) > 0);
    assert!(hashes::tianl_hash(&"x".repeat(257)) > 0);
    assert_eq!(hashes::hf_hash("abcd"), 1_788);
    assert_eq!(hashes::hf_ip_hash("abc"), 0);
    assert!(hashes::hf_ip_hash("abcde") > 0);
    assert_eq!(hashes::indexed_hash("x", 8), 0);
}

#[test]
fn function_and_named_filters_share_hutool_add_semantics() {
    let mut function = FuncFilter::new(128, |_| -7).unwrap();
    assert_eq!(function.hash("any"), 7);
    assert!(!function.contains("any"));
    assert!(function.add("any"));
    assert!(!function.add("other"));

    let custom = FuncFilter::with_machine(128, MachineWord::Bits64, |_| 9).unwrap();
    assert_eq!(custom.hash("value"), 9);
    assert!(matches!(
        FuncFilter::new(0, |_| 0),
        Err(BloomFilterError::EmptyCapacity)
    ));
    assert!(matches!(
        FuncFilter::new(i32::MAX as u64 + 1, |_| 0),
        Err(BloomFilterError::CapacityOverflow)
    ));

    macro_rules! check_filter {
        ($type:ty) => {{
            let mut filter = <$type>::with_machine(256, MachineWord::Bits64).unwrap();
            let index = filter.hash("hutool");
            assert!(index < 256);
            assert!(filter.add("hutool"));
            assert!(filter.contains("hutool"));
            assert!(!filter.add("hutool"));
            let default = <$type>::new(256).unwrap();
            assert!(default.hash("rust") < 256);
            assert!(<$type>::new(0).is_err());
        }};
    }
    check_filter!(DefaultFilter);
    check_filter!(ELFFilter);
    check_filter!(FNVFilter);
    check_filter!(JSFilter);
    check_filter!(PJWFilter);
    check_filter!(RSFilter);
    check_filter!(SDBMFilter);
    check_filter!(HfFilter);
    check_filter!(HfIpFilter);
    check_filter!(TianlFilter);
}

#[test]
fn bitmap_composition_uses_or_for_add_and_and_for_contains() {
    assert!(matches!(
        BitMapBloomFilter::new(4),
        Err(BloomFilterError::EmptyCapacity)
    ));
    assert!(matches!(
        BitMapBloomFilter::new(usize::MAX),
        Err(BloomFilterError::CapacityOverflow)
    ));
    assert!(matches!(
        BitMapBloomFilter::new(1_300),
        Err(BloomFilterError::CapacityOverflow)
    ));
    assert!(matches!(
        BitMapBloomFilter::with_filters(Vec::new()),
        Err(BloomFilterError::EmptyCapacity)
    ));

    let mut default = BitMapBloomFilter::new(5).unwrap();
    assert!(!default.contains("hutool"));
    assert!(default.add("hutool"));
    assert!(default.contains("hutool"));
    assert!(!default.add("hutool"));

    let mut explicit = BitMapBloomFilter::with_filters(vec![
        Box::new(DefaultFilter::new(128).unwrap()),
        Box::new(FNVFilter::new(128).unwrap()),
    ])
    .unwrap();
    assert!(explicit.add("rust"));
    assert!(explicit.contains("rust"));
}

#[test]
fn bitset_filter_validates_hashes_files_encoding_and_probability() {
    assert_eq!(
        BitSetBloomFilter::new(0, 1, 1).unwrap_err(),
        BloomFilterError::EmptyCapacity
    );
    assert_eq!(
        BitSetBloomFilter::new(1, 0, 1).unwrap_err(),
        BloomFilterError::EmptyCapacity
    );
    for count in [0, 9] {
        assert_eq!(
            BitSetBloomFilter::new(1, 1, count).unwrap_err(),
            BloomFilterError::InvalidHashFunctionCount
        );
    }
    assert_eq!(
        BitSetBloomFilter::new(usize::MAX, 1, 8).unwrap_err(),
        BloomFilterError::CapacityOverflow
    );
    assert_eq!(
        BitSetBloomFilter::new(i32::MAX as usize, 1, 2).unwrap_err(),
        BloomFilterError::CapacityOverflow
    );
    assert_eq!(
        BitSetBloomFilter::new(1, usize::MAX, 1).unwrap_err(),
        BloomFilterError::CapacityOverflow
    );

    let mut filter = BitSetBloomFilter::new(1_000, 100, 8).unwrap();
    assert_eq!(BitSetBloomFilter::create_hashes("hutool", 8).len(), 8);
    assert_eq!(BitSetBloomFilter::hash("hutool", 8), 0);
    assert!((0.0..1.0).contains(&filter.false_positive_probability()));
    assert!(filter.add("hutool"));
    assert!(filter.contains("hutool"));
    assert!(!filter.add("hutool"));

    let directory = tempfile::tempdir().unwrap();
    let utf8 = directory.path().join("words.txt");
    fs::write(&utf8, "rust\n工具\n").unwrap();
    filter.init(&utf8).unwrap();
    assert!(filter.contains("rust"));
    filter.init_with_encoding(&utf8, UTF_8).unwrap();
    assert!(filter.contains("工具"));

    let invalid = directory.path().join("invalid.txt");
    fs::write(&invalid, [0xff]).unwrap();
    assert_eq!(
        filter.init_with_encoding(&invalid, UTF_8).unwrap_err(),
        BloomFilterError::InvalidEncoding("UTF-8")
    );
    let missing = directory.path().join("missing.txt");
    let error = filter.init(&missing).unwrap_err();
    assert_eq!(error, BloomFilterError::Io(ErrorKind::NotFound.into()));
    assert_eq!(
        filter.init_with_encoding(&missing, UTF_8).unwrap_err(),
        BloomFilterError::Io(ErrorKind::NotFound.into())
    );
    assert_ne!(
        BloomFilterError::Io(ErrorKind::NotFound.into()),
        BloomFilterError::Io(ErrorKind::PermissionDenied.into())
    );
    assert_ne!(
        BloomFilterError::EmptyCapacity,
        BloomFilterError::CapacityOverflow
    );
}

#[test]
fn utility_constructors_delegate_to_concrete_filters() {
    let mut bit_set = BloomFilterUtil::create_bit_set(100, 10, 3).unwrap();
    assert!(bit_set.add("hutool"));
    let mut bit_map = BloomFilterUtil::create_bit_map(5).unwrap();
    assert!(bit_map.add("hutool"));
}
