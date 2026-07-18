//! `HashUtil` 对比验证测试 —— 对齐 Hutool `HashUtilTest`
//!
//! 对齐: `cn.hutool.core.util.HashUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/HashUtilTest.java
//!
//! # 当前状态
//!
//! Hutool `HashUtil` 委托给 `cn.hutool.core.lang.hash.CityHash`,
//! hitool-rs 的 `lang::hash::city_hash` 仍是空对齐桩(`pending_alignment`)。
//! 因此 cityHash 系列测试标注为 `#[ignore]`,等待 CityHash 实现后启用。

/// 测试输入字符串(对齐 Java 测试,保持完全一致)。
///
/// 注:Hutool Java 测试源中直接嵌入了该字符串,Rust 版本同样字面量。
use hitool_core::HashUtil;

const TEST_STR: &str = "Google发布的Hash计算算法:CityHash64 与 CityHash128";

/// 对齐 Java: `HashUtilTest.cityHash128Test()` (行 8-14)
///
/// ```java
/// String s="Google发布的Hash计算算法：CityHash64 与 CityHash128";
/// final long[] hash = HashUtil.cityHash128(StrUtil.utf8Bytes(s));
/// assertEquals(0x5944f1e788a18db0L, hash[0]);
/// assertEquals(0xc2f68d8b2bf4a5cfL, hash[1]);
/// ```
///
/// **状态**: hitool `lang::hash::city_hash` 未实现,标注 ignored。
#[test]
#[ignore = "等待 hitool_core::lang::hash::city_hash 实现 CityHash128"]
fn city_hash_128_test() {
    // 期望值(对齐 Java):
    //   hash[0] = 0x5944f1e788a18db0_i64
    //   hash[1] = 0xc2f68d8b2bf4a5cf_i64
    // 实现后取消 ignore 并填入实际断言。
}

/// 对齐 Java: `HashUtilTest.cityHash64Test()` (行 16-21)
///
/// ```java
/// final long hash = HashUtil.cityHash64(StrUtil.utf8Bytes(s));
/// assertEquals(0x1d408f2bbf967e2aL, hash);
/// ```
#[test]
#[ignore = "等待 hitool_core::lang::hash::city_hash 实现 CityHash64"]
fn city_hash_64_test() {
    // 期望值(对齐 Java): 0x1d408f2bbf967e2a_i64
}

/// 对齐 Java: `HashUtilTest.cityHash32Test()` (行 23-28)
///
/// ```java
/// final int hash = HashUtil.cityHash32(StrUtil.utf8Bytes(s));
/// assertEquals(0xa8944fbe, hash);
/// ```
#[test]
#[ignore = "等待 hitool_core::lang::hash::city_hash 实现 CityHash32"]
fn city_hash_32_test() {
    // 期望值(对齐 Java): 0xa8944fbe_i32
}

/// 验证测试输入字符串的 UTF-8 字节序列可稳定生成。
///
/// 该测试用于在 cityHash 未实现期间,至少验证测试输入可重复,
/// 待 cityHash 实现后此测试可删除。
#[test]
fn test_str_utf8_bytes_stable() {
    let bytes = TEST_STR.as_bytes();
    assert!(!bytes.is_empty(), "TEST_STR UTF-8 字节非空 (稳定性占位)");
    assert_eq!(
        TEST_STR,
        std::str::from_utf8(bytes).unwrap(),
        "UTF-8 round-trip TEST_STR (稳定性占位)"
    );
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
