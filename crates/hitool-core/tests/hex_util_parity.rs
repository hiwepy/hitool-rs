//! `HexUtil` 对比验证测试 —— 对齐 Hutool `HexUtilTest`
//!
//! 对齐: `cn.hutool.core.util.HexUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/HexUtilTest.java
//!
//! # API 命名映射
//!
//! | Java Hutool                            | hitool-rs Rust                            |
//! |----------------------------------------|-------------------------------------------|
//! | `encodeHexStr(str, charset)`           | `encode_hex_utf8(str)` (UTF-8 hardcoded)  |
//! | `encodeHexStr(bytes)`                  | `encode_hex(bytes)`                       |
//! | `decodeHexStr(hex)`                    | `decode_hex_text(hex)`                    |
//! | `decodeHex(hex)`                       | `decode_hex(hex)`                         |
//! | `toUnicodeHex(char)`                   | `to_unicode_hex(char)`                    |
//! | `isHexNumber(str)`                     | `is_hex_number(str)`                      |
//! | `format(hex)`                          | `format(hex)`                             |
//! | `format(hex, prefix)`                  | `format_with_prefix(hex, prefix)`         |
//! | `hexToInt(hex)`                        | `hex_to_i32(hex)`                         |
//! | `hexToLong(hex)`                       | `hex_to_i64(hex)`                         |
//! | `hexToFloat(hex)`                      | `hex_to_f32(hex)`                         |
//! | `hexToDouble(hex)`                     | `hex_to_f64(hex)`                         |
//! | `toHex(float)`                         | `to_hex_f32(f32)`                         |
//! | `toHex(double)`                        | `to_hex_f64(f64)`                         |
//! | `toBigInteger(hex)`                    | `to_big_integer(Some(hex))`               |

use hitool_core::{HexUtil, RgbColor};

/// 对齐 Java: `HexUtilTest.hexStrTest()` (行 16-24)
///
/// ```java
/// final String str = "我是一个字符串";
/// final String hex = HexUtil.encodeHexStr(str, CharsetUtil.CHARSET_UTF_8);
/// final String decodedStr = HexUtil.decodeHexStr(hex);
/// assertEquals(str, decodedStr);
/// ```
#[test]
fn hex_str_test() {
    let s = "我是一个字符串";
    let hex = HexUtil::encode_hex_utf8(s);
    let decoded = HexUtil::decode_hex_text(&hex).unwrap();
    assert_eq!(
        decoded, s,
        "UTF-8 hex round-trip (对齐 Java hexStrTest)"
    );
}

/// 对齐 Java: `HexUtilTest.issueI50MI6Test()` (行 26-30)
///
/// ```java
/// final String s = HexUtil.encodeHexStr("烟".getBytes(StandardCharsets.UTF_16BE));
/// assertEquals("70df", s);
/// ```
#[test]
fn issue_i50mi6_test() {
    // "烟" 在 UTF-16BE 中是 0x70 0xDF
    let bytes = "烟".encode_utf16().flat_map(|c| c.to_be_bytes()).collect::<Vec<_>>();
    let s = HexUtil::encode_hex(&bytes);
    assert_eq!(
        s, "70df",
        "encode_hex UTF-16BE bytes of '烟' (对齐 Java issueI50MI6Test)"
    );
}

/// 对齐 Java: `HexUtilTest.toUnicodeHexTest()` (行 32-39)
#[test]
fn to_unicode_hex_test() {
    assert_eq!(
        HexUtil::to_unicode_hex('\u{2001}'),
        "\\u2001",
        "to_unicode_hex(U+2001) (对齐 Java toUnicodeHexTest)"
    );
    assert_eq!(
        HexUtil::to_unicode_hex('你'),
        "\\u4f60",
        "to_unicode_hex('你') (对齐 Java toUnicodeHexTest)"
    );
}

/// 对齐 Java: `HexUtilTest.isHexNumberTest()` (行 41-60)
#[test]
fn is_hex_number_test() {
    assert!(HexUtil::is_hex_number("0"), "is_hex_number(\"0\") (对齐 Java)");
    assert!(HexUtil::is_hex_number("002c"), "is_hex_number(\"002c\") (对齐 Java)");
    assert!(HexUtil::is_hex_number("0x3544534F444"), "is_hex_number(\"0x3544534F444\") (对齐 Java)");
    // https://gitee.com/chinabugotech/hutool/issues/I62H7K
    assert!(
        HexUtil::is_hex_number("0x0000000000000001158e460913d00000"),
        "is_hex_number(long hex) (对齐 Java I62H7K)"
    );
    // 错误的
    assert!(
        !HexUtil::is_hex_number("0x0000001000T00001158e460913d00000"),
        "is_hex_number(含 T) 应 false (对齐 Java)"
    );
    // 错误的, https://github.com/chinabugotech/hutool/issues/2857
    assert!(
        !HexUtil::is_hex_number("-1"),
        "is_hex_number(\"-1\") 应 false (对齐 Java issue#2857)"
    );
}

/// 对齐 Java: `HexUtilTest.isHexNumberTest2()` (行 62-66)
///
/// Java 测试 `isHexNumber(null)` 期望 false,Rust 用 `Option::None` 表达。
#[test]
fn is_hex_number_test_2() {
    assert!(
        !HexUtil::is_hex_number(""),
        "is_hex_number(\"\") 应 false (对齐 Java isHexNumberTest2)"
    );
    // null → None
    assert!(
        !HexUtil::is_hex_number_option(None),
        "is_hex_number(None) 应 false (对齐 Java isHexNumber(null))"
    );
}

/// 对齐 Java: `HexUtilTest.decodeTest()` (行 68-73)
///
/// 验证大小写不敏感:同一段 hex 用大写或小写解码应得到相同字节。
#[test]
fn decode_test() {
    let s = "e8c670380cb220095268f40221fc748fa6ac39d6e930e63c30da68bad97f885d";
    let lower = HexUtil::decode_hex(s).unwrap();
    let upper = HexUtil::decode_hex(&s.to_uppercase()).unwrap();
    assert_eq!(
        lower, upper,
        "decode_hex 大小写不敏感 (对齐 Java decodeTest)"
    );
}

/// 对齐 Java: `HexUtilTest.formatHexTest()` (行 75-80)
#[test]
fn format_hex_test() {
    let hex = "e8c670380cb220095268f40221fc748fa6ac39d6e930e63c30da68bad97f885d";
    let formatted = HexUtil::format(hex);
    assert_eq!(
        formatted,
        "e8 c6 70 38 0c b2 20 09 52 68 f4 02 21 fc 74 8f a6 ac 39 d6 e9 30 e6 3c 30 da 68 ba d9 7f 88 5d",
        "format(hex) 每字节空格分隔 (对齐 Java formatHexTest)"
    );
}

/// 对齐 Java: `HexUtilTest.formatHexTest2()` (行 82-87)
#[test]
fn format_hex_test_2() {
    let hex = "e8c670380cb220095268f40221fc748fa6";
    let formatted = HexUtil::format_with_prefix(hex, "0x");
    assert_eq!(
        formatted,
        "0xe8 0xc6 0x70 0x38 0x0c 0xb2 0x20 0x09 0x52 0x68 0xf4 0x02 0x21 0xfc 0x74 0x8f 0xa6",
        "format_with_prefix(hex, \"0x\") (对齐 Java formatHexTest2)"
    );
}

/// 对齐 Java: `HexUtilTest.decodeHexTest()` (行 89-94)
#[test]
fn decode_hex_test() {
    let s = HexUtil::encode_hex_utf8("6");
    let s1 = HexUtil::decode_hex_text(&s).unwrap();
    assert_eq!(s1, "6", "round-trip \"6\" (对齐 Java decodeHexTest)");
}

/// 对齐 Java: `HexUtilTest.hexToIntTest()` (行 96-104)
#[test]
fn hex_to_int_test() {
    assert_eq!(HexUtil::hex_to_i32("FF").unwrap(), 255, "hex_to_i32(\"FF\") (对齐 Java)");
    assert_eq!(HexUtil::hex_to_i32("0xFF").unwrap(), 255, "hex_to_i32(\"0xFF\") (对齐 Java)");
    assert_eq!(HexUtil::hex_to_i32("#FF").unwrap(), 255, "hex_to_i32(\"#FF\") (对齐 Java)");
}

/// 对齐 Java: `HexUtilTest.hexToLongTest()` (行 106-114)
#[test]
fn hex_to_long_test() {
    assert_eq!(HexUtil::hex_to_i64("FF").unwrap(), 255, "hex_to_i64(\"FF\") (对齐 Java)");
    assert_eq!(HexUtil::hex_to_i64("0xFF").unwrap(), 255, "hex_to_i64(\"0xFF\") (对齐 Java)");
    assert_eq!(HexUtil::hex_to_i64("#FF").unwrap(), 255, "hex_to_i64(\"#FF\") (对齐 Java)");
}

/// 对齐 Java: `HexUtilTest.hexToFloatTest()` (行 116-136)
#[test]
fn hex_to_float_test() {
    // 测试正常浮点数值
    let v1: f32 = 1.5;
    let h1 = HexUtil::to_hex_f32(v1);
    assert_eq!(HexUtil::hex_to_f32(&h1).unwrap(), v1, "f32 round-trip 1.5 (对齐 Java)");

    // 测试负数
    let v2: f32 = -1.5;
    let h2 = HexUtil::to_hex_f32(v2);
    assert_eq!(HexUtil::hex_to_f32(&h2).unwrap(), v2, "f32 round-trip -1.5 (对齐 Java)");

    // 测试科学计数法值
    let v3: f32 = 1.23456789e-5;
    let h3 = HexUtil::to_hex_f32(v3);
    assert_eq!(HexUtil::hex_to_f32(&h3).unwrap(), v3, "f32 round-trip 科学计数法 (对齐 Java)");

    // 测试十六进制前缀
    assert_eq!(HexUtil::hex_to_f32("0x3fc00000").unwrap(), 1.5_f32, "0x prefix (对齐 Java)");
    assert_eq!(HexUtil::hex_to_f32("#3fc00000").unwrap(), 1.5_f32, "# prefix (对齐 Java)");
}

/// 对齐 Java: `HexUtilTest.hexToDoubleTest()` (行 138-163)
#[test]
fn hex_to_double_test() {
    let v1: f64 = 1.5;
    let h1 = HexUtil::to_hex_f64(v1);
    assert_eq!(HexUtil::hex_to_f64(&h1).unwrap(), v1, "f64 round-trip 1.5 (对齐 Java)");

    let v3: f64 = -1.5;
    let h3 = HexUtil::to_hex_f64(v3);
    assert_eq!(HexUtil::hex_to_f64(&h3).unwrap(), v3, "f64 round-trip -1.5 (对齐 Java)");

    let v4: f64 = std::f64::consts::PI;
    let h4 = HexUtil::to_hex_f64(v4);
    assert_eq!(HexUtil::hex_to_f64(&h4).unwrap(), v4, "f64 round-trip PI (对齐 Java)");

    let v5: f64 = 1.23456789012345e-10;
    let h5 = HexUtil::to_hex_f64(v5);
    assert_eq!(HexUtil::hex_to_f64(&h5).unwrap(), v5, "f64 round-trip 科学计数法 (对齐 Java)");

    assert_eq!(HexUtil::hex_to_f64("0x3ff8000000000000").unwrap(), 1.5_f64, "0x prefix (对齐 Java)");
    assert_eq!(HexUtil::hex_to_f64("#3ff8000000000000").unwrap(), 1.5_f64, "# prefix (对齐 Java)");
}

/// 对齐 Java: `HexUtilTest.toBigIntegerTest()` (行 165-173)
///
/// 注:Java `new BigInteger("FF", 16)` = 255;Rust 用 `BigInt::parse_bytes(b"FF", 16)`。
/// hitool `to_big_integer` 接受 `Option<&str>`,返回 `Option<BigInt>`。
#[test]
fn to_big_integer_test() {
    use num_bigint::BigInt;
    let expected = BigInt::parse_bytes(b"FF", 16).unwrap();
    assert_eq!(
        HexUtil::to_big_integer(Some("FF")).unwrap().unwrap(),
        expected,
        "to_big_integer(\"FF\") (对齐 Java)"
    );
    assert_eq!(
        HexUtil::to_big_integer(Some("0xFF")).unwrap().unwrap(),
        expected,
        "to_big_integer(\"0xFF\") (对齐 Java)"
    );
    assert_eq!(
        HexUtil::to_big_integer(Some("#FF")).unwrap().unwrap(),
        expected,
        "to_big_integer(\"#FF\") (对齐 Java)"
    );
}

/// 对齐 Java: `HexUtilTest.testFormatEmpty()` (行 175-179)
#[test]
fn test_format_empty() {
    let result = HexUtil::format("");
    assert_eq!(result, "", "format(\"\") = \"\" (对齐 Java testFormatEmpty)");
}

/// 对齐 Java: `HexUtilTest.testFormatSingleChar()` (行 181-185)
#[test]
fn test_format_single_char() {
    let result = HexUtil::format("1");
    assert_eq!(result, "1", "format(\"1\") = \"1\" (对齐 Java testFormatSingleChar)");
}

/// 对齐 Java: `HexUtilTest.testFormatOddLength()` (行 187-191)
#[test]
fn test_format_odd_length() {
    let result = HexUtil::format("123");
    assert_eq!(result, "12 3", "format(\"123\") = \"12 3\" (对齐 Java testFormatOddLength)");
}

/// 对齐 Java: `HexUtilTest.testFormatWithPrefixSingleChar()` (行 193-197)
#[test]
fn test_format_with_prefix_single_char() {
    let result = HexUtil::format_with_prefix("1", "0x");
    assert_eq!(result, "0x1", "format_with_prefix(\"1\", \"0x\") (对齐 Java)");
}

/// 对齐 Java: `HexUtilTest.testFormatWithPrefixOddLength()` (行 199-203)
#[test]
fn test_format_with_prefix_odd_length() {
    let result = HexUtil::format_with_prefix("123", "0x");
    assert_eq!(result, "0x12 0x3", "format_with_prefix(\"123\", \"0x\") (对齐 Java)");
}
// ── 扩展 hex_util 测试 ──

#[test]
fn is_hex_number_valid() {
    assert!(HexUtil::is_hex_number("0123456789ABCDEF"));
    assert!(HexUtil::is_hex_number("0123456789abcdef"));
    assert!(HexUtil::is_hex_number("0"));
    assert!(HexUtil::is_hex_number("FF"));
}

#[test]
fn is_hex_number_invalid() {
    assert!(!HexUtil::is_hex_number(""));
    assert!(!HexUtil::is_hex_number("GHIJ"));
    assert!(!HexUtil::is_hex_number("hello"));
    assert!(!HexUtil::is_hex_number("12.34"));
}

#[test]
fn encode_decode_hex_roundtrip() {
    let input = b"Hello, World!";
    let encoded = HexUtil::encode_hex(input);
    let decoded = HexUtil::decode_hex(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn encode_hex_empty() {
    let encoded = HexUtil::encode_hex(b"");
    assert!(encoded.is_empty());
}

#[test]
fn encode_hex_lowercase() {
    let encoded = HexUtil::encode_hex_case(b"AB", true);
    assert!(encoded.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
}

#[test]
fn encode_hex_uppercase() {
    let encoded = HexUtil::encode_hex_case(b"ab", false);
    assert!(encoded.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
}

#[test]
fn encode_hex_utf8_roundtrip() {
    let input = "Hello, 你好!";
    let encoded = HexUtil::encode_hex_utf8(input);
    let decoded = HexUtil::decode_hex_text(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn decode_hex_invalid() {
    assert!(HexUtil::decode_hex("XYZ").is_err());
    
}

#[test]
fn decode_hex_chars() {
    let chars: Vec<char> = "48656C6C6F".chars().collect();
    let decoded = HexUtil::decode_hex_chars(&chars).unwrap();
    assert_eq!(decoded, b"Hello");
}

#[test]
fn encode_decode_color_roundtrip() {
    let color = RgbColor { red: 255, green: 128, blue: 0 };
    let encoded = HexUtil::encode_color(color);
    let decoded = HexUtil::decode_color(&encoded).unwrap();
    assert_eq!(decoded.red, 255);
    assert_eq!(decoded.green, 128);
    assert_eq!(decoded.blue, 0);
}

#[test]
fn encode_color_with_prefix() {
    let color = RgbColor { red: 255, green: 128, blue: 0 };
    let encoded = HexUtil::encode_color_with_prefix(color, "0x");
    assert!(encoded.starts_with("0x"));
}

#[test]
fn decode_color_invalid() {
    assert!(HexUtil::decode_color("invalid").is_err());
    assert!(HexUtil::decode_color("GGG").is_err());
}

#[test]
fn to_unicode_hex_i32_basic() {
    let result = HexUtil::to_unicode_hex_i32(0x4E2D); // '中' in Unicode
    assert!(!result.is_empty());
}

#[test]
fn encode_hex_binary_data() {
    let input: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let encoded = HexUtil::encode_hex(&input);
    assert_eq!(encoded.len(), 512); // 256 bytes = 512 hex chars
    let decoded = HexUtil::decode_hex(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn encode_hex_text_with_encoding() {
    let encoded = HexUtil::encode_hex_text("Hello", encoding_rs::UTF_8);
    let decoded = HexUtil::decode_hex_text_with_encoding(&encoded, encoding_rs::UTF_8).unwrap();
    assert_eq!(decoded, "Hello");
}
