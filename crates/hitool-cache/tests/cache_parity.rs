use hitool_cache as hc;
use std::time::Duration;

#[test]
fn lru_cache_put_get_test() {
    let cache = hc::LRUCache::new(3);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    assert_eq!(cache.get(&"a").map(|v| v.to_string()), Some("1".to_string()));
    assert_eq!(cache.get(&"b").map(|v| v.to_string()), Some("2".to_string()));
    assert_eq!(cache.get(&"c").map(|v| v.to_string()), Some("3".to_string()));
}

#[test]
fn lru_cache_eviction_test() {
    let cache = hc::LRUCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    // a 应被驱逐
    assert!(cache.get(&"a").is_none(), "LRU 驱逐后 a 应不存在");
    assert_eq!(cache.get(&"b").map(|v| v.to_string()), Some("2".to_string()));
    assert_eq!(cache.get(&"c").map(|v| v.to_string()), Some("3".to_string()));
}

#[test]
fn lru_cache_remove_test() {
    let cache = hc::LRUCache::new(3);
    cache.put("a", "1".to_string());
    cache.remove(&"a");
    assert!(cache.get(&"a").is_none(), "remove 后应不存在");
}

#[test]
fn fifo_cache_put_get_test() {
    let cache = hc::FIFOCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    assert_eq!(cache.get(&"a").map(|v| v.to_string()), Some("1".to_string()));
    assert_eq!(cache.get(&"b").map(|v| v.to_string()), Some("2".to_string()));
}

#[test]
fn fifo_cache_eviction_test() {
    let cache = hc::FIFOCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    // FIFO: a 最先进入,应被驱逐
    assert!(cache.get(&"a").is_none(), "FIFO 驱逐后 a 应不存在");
}
