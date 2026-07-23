//! `ByteUtil` 对比验证测试 —— 对齐 Hutool `ByteUtilTest`
//!
//! 对齐: `cn.hutool.core.util.ByteUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/ByteUtilTest.java
//!
//! # API 命名映射
//! | Java                                       | Rust                                        |
//! |--------------------------------------------|---------------------------------------------|
//! | `intToBytes(int)`                          | `i32_to_bytes(i32) -> [u8; 4]`              |
//! | `intToBytes(int, ByteOrder)`               | `i32_to_bytes_with_order(i32, ByteOrder)`   |
//! | `bytesToInt(byte[], ByteOrder)`            | `bytes_to_i32_with_order(&[u8], ByteOrder)` |
//! | `longToBytes(long)`                        | `i64_to_bytes(i64) -> [u8; 8]`              |
//! | `longToBytes(long, ByteOrder)`             | `i64_to_bytes_with_order(i64, ByteOrder)`   |
//! | `bytesToLong(byte[], ByteOrder)`           | `bytes_to_i64_with_order(&[u8], ByteOrder)` |
//! | `shortToBytes(short)`                      | `i16_to_bytes(i16) -> [u8; 2]`              |
//! | `shortToBytes(short, ByteOrder)`           | `i16_to_bytes_with_order(i16, ByteOrder)`   |
//! | `bytesToShort(byte[], ByteOrder)`          | `bytes_to_i16_with_order(&[u8], ByteOrder)` |
//! | `floatToBytes(float, ByteOrder)`           | `f32_to_bytes_with_order(f32, ByteOrder)`   |
//! | `bytesToFloat(byte[], ByteOrder)`          | `bytes_to_f32_with_order(&[u8], ByteOrder)` |
//!
//! # 默认字节序
//!
//! Hutool 的 `intToBytes(int)` 无 ByteOrder 入参时默认 **小端序**(LITTLE_ENDIAN)。
//! hutool 的 `i32_to_bytes(i32)` 同样默认小端序。

use hutool_core::{ByteOrder, ByteUtil};

/// 对齐 Java: `ByteUtilTest.intAndBytesLittleEndianTest()` (行 10-33)
#[test]
fn int_and_bytes_little_endian_test() {
    let int1: i32 = rand::random::<u32>() as i32 & i32::MAX; // 等价 Java randomInt(MAX_VALUE)
    let bytes = ByteUtil::i32_to_bytes_with_order(int1, ByteOrder::LittleEndian);
    let int2 = ByteUtil::bytes_to_i32_with_order(&bytes, ByteOrder::LittleEndian).unwrap();
    assert_eq!(int1, int2, "i32 LE round-trip (对齐 Java)");

    // 二次往返
    let bytes2 = ByteUtil::i32_to_bytes_with_order(int1, ByteOrder::LittleEndian);
    let int3 = ByteUtil::bytes_to_i32_with_order(&bytes2, ByteOrder::LittleEndian).unwrap();
    assert_eq!(int1, int3, "i32 LE round-trip 2 (对齐 Java)");

    let bytes3 = ByteUtil::i32_to_bytes_with_order(int1, ByteOrder::LittleEndian);
    let int4 = ByteUtil::bytes_to_i32_with_order(&bytes3, ByteOrder::LittleEndian).unwrap();
    assert_eq!(int1, int4, "i32 LE round-trip 3 (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.intAndBytesBigEndianTest()` (行 35-50)
#[test]
fn int_and_bytes_big_endian_test() {
    let int2: i32 = rand::random::<u32>() as i32 & i32::MAX;
    let bytes = ByteUtil::i32_to_bytes_with_order(int2, ByteOrder::BigEndian);
    let int3 = ByteUtil::bytes_to_i32_with_order(&bytes, ByteOrder::BigEndian).unwrap();
    assert_eq!(int2, int3, "i32 BE round-trip (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.longAndBytesLittleEndianTest()` (行 52-75)
#[test]
fn long_and_bytes_little_endian_test() {
    let long1: i64 = rand::random::<u64>() as i64 & i64::MAX;

    let bytes = ByteUtil::i64_to_bytes_with_order(long1, ByteOrder::LittleEndian);
    let long2 = ByteUtil::bytes_to_i64_with_order(&bytes, ByteOrder::LittleEndian).unwrap();
    assert_eq!(long1, long2, "i64 LE round-trip (对齐 Java)");

    // longToBytes(long1) 默认 LE
    let bytes2 = ByteUtil::i64_to_bytes(long1);
    let long3 = ByteUtil::bytes_to_i64_with_order(&bytes2, ByteOrder::LittleEndian).unwrap();
    assert_eq!(long1, long3, "i64 LE round-trip default→explicit (对齐 Java)");

    let bytes3 = ByteUtil::i64_to_bytes_with_order(long1, ByteOrder::LittleEndian);
    // bytesToLong(bytes3) 默认 LE
    let long4 = ByteUtil::bytes_to_i64(&bytes3).unwrap();
    assert_eq!(long1, long4, "i64 LE round-trip explicit→default (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.longAndBytesBigEndianTest()` (行 77-91)
#[test]
fn long_and_bytes_big_endian_test() {
    let long1: i64 = rand::random::<u64>() as i64 & i64::MAX;
    let bytes = ByteUtil::i64_to_bytes_with_order(long1, ByteOrder::BigEndian);
    let long2 = ByteUtil::bytes_to_i64_with_order(&bytes, ByteOrder::BigEndian).unwrap();
    assert_eq!(long1, long2, "i64 BE round-trip (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.floatAndBytesLittleEndianTest()` (行 93-101)
#[test]
fn float_and_bytes_little_endian_test() {
    let f1: f32 = rand::random::<f32>();
    let bytes = ByteUtil::f32_to_bytes_with_order(f1, ByteOrder::LittleEndian);
    let f2 = ByteUtil::bytes_to_f32_with_order(&bytes, ByteOrder::LittleEndian).unwrap();
    assert_eq!(f1, f2, "f32 LE round-trip (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.floatAndBytesBigEndianTest()` (行 103-112)
#[test]
fn float_and_bytes_big_endian_test() {
    let f1: f32 = rand::random::<f32>();
    let bytes = ByteUtil::f32_to_bytes_with_order(f1, ByteOrder::BigEndian);
    let f2 = ByteUtil::bytes_to_f32_with_order(&bytes, ByteOrder::BigEndian).unwrap();
    assert_eq!(f1, f2, "f32 BE round-trip (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.shortAndBytesLittleEndianTest()` (行 114-129)
#[test]
fn short_and_bytes_little_endian_test() {
    let short1: i16 = rand::random::<i16>();

    let bytes = ByteUtil::i16_to_bytes_with_order(short1, ByteOrder::LittleEndian);
    let short2 = ByteUtil::bytes_to_i16_with_order(&bytes, ByteOrder::LittleEndian).unwrap();
    assert_eq!(short2, short1, "i16 LE round-trip (对齐 Java)");

    // shortToBytes(short1) 默认 LE
    let bytes2 = ByteUtil::i16_to_bytes(short1);
    let short3 = ByteUtil::bytes_to_i16_with_order(&bytes2, ByteOrder::LittleEndian).unwrap();
    assert_eq!(short3, short1, "i16 LE default→explicit (对齐 Java)");

    let bytes3 = ByteUtil::i16_to_bytes_with_order(short1, ByteOrder::LittleEndian);
    let short4 = ByteUtil::bytes_to_i16(&bytes3).unwrap();
    assert_eq!(short4, short1, "i16 LE explicit→default (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.shortAndBytesBigEndianTest()` (行 131-138)
#[test]
fn short_and_bytes_big_endian_test() {
    let short1: i16 = 122;
    let bytes = ByteUtil::i16_to_bytes_with_order(short1, ByteOrder::BigEndian);
    let short2 = ByteUtil::bytes_to_i16_with_order(&bytes, ByteOrder::BigEndian).unwrap();
    assert_eq!(short2, short1, "i16 BE round-trip 122 (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.bytesToLongTest()` (行 140-152)
///
/// 用 `longToBytes(long)` 默认 LE,再用 ByteBuffer LE 解码,应一致。
/// Rust 没有 ByteBuffer,直接用 i64_to_bytes / bytes_to_i64_with_order 验证 round-trip。
#[test]
fn bytes_to_long_test() {
    let a: i64 = rand::random::<u64>() as i64 & i64::MAX;
    let bytes_le = ByteUtil::i64_to_bytes(a); // 默认 LE
    let decoded_le = ByteUtil::bytes_to_i64_with_order(&bytes_le, ByteOrder::LittleEndian).unwrap();
    assert_eq!(a, decoded_le, "bytes_to_long LE (对齐 Java)");

    let bytes_be = ByteUtil::i64_to_bytes_with_order(a, ByteOrder::BigEndian);
    let decoded_be = ByteUtil::bytes_to_i64_with_order(&bytes_be, ByteOrder::BigEndian).unwrap();
    assert_eq!(a, decoded_be, "bytes_to_long BE (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.bytesToIntTest()` (行 154-166)
#[test]
fn bytes_to_int_test() {
    let a: i32 = rand::random::<u32>() as i32 & i32::MAX;
    let bytes_le = ByteUtil::i32_to_bytes(a);
    let decoded_le = ByteUtil::bytes_to_i32_with_order(&bytes_le, ByteOrder::LittleEndian).unwrap();
    assert_eq!(a, decoded_le, "bytes_to_int LE (对齐 Java)");

    let bytes_be = ByteUtil::i32_to_bytes_with_order(a, ByteOrder::BigEndian);
    let decoded_be = ByteUtil::bytes_to_i32_with_order(&bytes_be, ByteOrder::BigEndian).unwrap();
    assert_eq!(a, decoded_be, "bytes_to_int BE (对齐 Java)");
}

/// 对齐 Java: `ByteUtilTest.bytesToShortTest()` (行 168-181)
#[test]
fn bytes_to_short_test() {
    let a: i16 = (rand::random::<u16>() as i32 % (i16::MAX as i32 + 1)) as i16;
    let bytes_le = ByteUtil::i16_to_bytes(a);
    let decoded_le = ByteUtil::bytes_to_i16_with_order(&bytes_le, ByteOrder::LittleEndian).unwrap();
    assert_eq!(a, decoded_le, "bytes_to_short LE (对齐 Java)");

    let bytes_be = ByteUtil::i16_to_bytes_with_order(a, ByteOrder::BigEndian);
    let decoded_be = ByteUtil::bytes_to_i16_with_order(&bytes_be, ByteOrder::BigEndian).unwrap();
    assert_eq!(a, decoded_be, "bytes_to_short BE (对齐 Java)");
}