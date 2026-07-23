//! `HashUtil` 对比验证测试 —— 对齐 Hutool `HashUtilTest`
//!
//! 对齐: `cn.hutool.core.util.HashUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/HashUtilTest.java

use hutool_core::HashUtil;

/// 测试输入字符串(对齐 Java 测试,含全角冒号 `：`)。
const TEST_STR: &str = "Google发布的Hash计算算法：CityHash64 与 CityHash128";

/// 对齐 Java: `HashUtilTest.cityHash128Test()` (行 8-14)
#[test]
fn city_hash_128_test() {
    let hash = HashUtil::city_hash128(TEST_STR.as_bytes());
    assert_eq!(hash[0], 0x5944_f1e7_88a1_8db0_u64 as i64);
    assert_eq!(hash[1], 0xc2f6_8d8b_2bf4_a5cf_u64 as i64);
}

/// 对齐 Java: `HashUtilTest.cityHash64Test()` (行 16-21)
#[test]
fn city_hash_64_test() {
    let hash = HashUtil::city_hash64(TEST_STR.as_bytes());
    assert_eq!(hash, 0x1d40_8f2b_bf96_7e2a_u64 as i64);
}

/// 对齐 Java: `HashUtilTest.cityHash32Test()` (行 23-28)
#[test]
fn city_hash_32_test() {
    let hash = HashUtil::city_hash32(TEST_STR.as_bytes());
    assert_eq!(hash, 0xa894_4fbe_u32 as i32);
}

// ── 扩展 hash_util 测试 ──

#[test]
fn additive_hash_basic() {
    let result = HashUtil::additive_hash("hello", 31).unwrap();
    assert_ne!(result, 0);
}

#[test]
fn rotating_hash_basic() {
    let result = HashUtil::rotating_hash("hello", 31).unwrap();
    assert_ne!(result, 0);
}

#[test]
fn one_by_one_hash_basic() {
    let result = HashUtil::one_by_one_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn bernstein_hash_basic() {
    let result = HashUtil::bernstein("hello");
    assert_ne!(result, 0);
}

#[test]
fn fnv_hash_basic() {
    let result = HashUtil::fnv_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn fnv_hash_bytes_basic() {
    let result = HashUtil::fnv_hash_bytes(b"hello");
    assert_ne!(result, 0);
}

#[test]
fn int_hash_basic() {
    let result = HashUtil::int_hash(42);
    assert_ne!(result, 0);
}

#[test]
fn rs_hash_basic() {
    let result = HashUtil::rs_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn js_hash_basic() {
    let result = HashUtil::js_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn pjw_hash_basic() {
    let result = HashUtil::pjw_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn elf_hash_basic() {
    let result = HashUtil::elf_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn bkdr_hash_basic() {
    let result = HashUtil::bkdr_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn sdbm_hash_basic() {
    let result = HashUtil::sdbm_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn djb_hash_basic() {
    let result = HashUtil::djb_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn dek_hash_basic() {
    let result = HashUtil::dek_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn ap_hash_basic() {
    let result = HashUtil::ap_hash("hello");
    assert_ne!(result, 0);
}

#[test]
fn consistent_hash_basic() {
    let hash1 = HashUtil::fnv_hash("hello");
    let hash2 = HashUtil::fnv_hash("hello");
    assert_eq!(hash1, hash2);
}

#[test]
fn different_strings_different_hash() {
    let hash1 = HashUtil::fnv_hash("hello");
    let hash2 = HashUtil::fnv_hash("world");
    assert_ne!(hash1, hash2);
}
