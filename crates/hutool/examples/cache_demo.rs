//! Hutool 能力对照：`hutool-cache` → `hutool-cache`（feature `cache`）。
//!
//! 演示基于 Moka 的本地缓存：容量上限、写入与读取。

use hutool::cache::{Cache, CacheConfig};

fn main() {
    let cache = Cache::new(CacheConfig {
        max_capacity: 128,
        ..CacheConfig::default()
    });

    cache.insert("greeting", "hello".to_owned());
    let value = cache.get(&"greeting").expect("cache hit");
    println!("cache.get(greeting) = {value}");

    assert!(cache.get(&"missing").is_none());
    println!("cache miss for missing key as expected");
}
