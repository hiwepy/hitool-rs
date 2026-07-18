use hitool_bloom_filter as hb;

#[test]
fn bloom_filter_contains_empty_test() {
    let filter = hb::BloomFilter::new(1000, 0.01).unwrap();
    assert!(!filter.contains("test"), "空 filter 应不含值");
}

#[test]
fn bloom_filter_insert_contains_test() {
    let mut filter = hb::BloomFilter::new(1000, 0.01).unwrap();
    filter.insert("hello");
    assert!(filter.contains("hello"), "已 insert 应 contains");
}

#[test]
fn bloom_filter_false_positive_test() {
    let mut filter = hb::BloomFilter::new(10, 0.1).unwrap();
    filter.insert("a");
    filter.insert("b");
    assert!(filter.contains("a"));
    assert!(filter.contains("b"));
}
