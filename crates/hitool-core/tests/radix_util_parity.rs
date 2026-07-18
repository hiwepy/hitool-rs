//! radix_util module parity tests
//! 对齐: hutool-core RadixUtilTest

use hitool_core::RadixUtil;

// ── encode_i32 / decode ──

#[test]
fn radix_encode_decode_hex() {
    let alphabet = "0123456789ABCDEF";
    let encoded = RadixUtil::encode_i32(alphabet, 255).unwrap();
    assert_eq!(encoded, "FF");
    let decoded = RadixUtil::decode(alphabet, &encoded).unwrap();
    assert_eq!(decoded, 255);
}

#[test]
fn radix_encode_decode_base36() {
    let alphabet = "0123456789abcdefghijklmnopqrstuvwxyz";
    let encoded = RadixUtil::encode_i32(alphabet, 1000000).unwrap();
    let decoded = RadixUtil::decode(alphabet, &encoded).unwrap();
    assert_eq!(decoded, 1000000);
}

#[test]
fn radix_encode_zero() {
    let alphabet = "0123456789";
    let encoded = RadixUtil::encode_i32(alphabet, 0).unwrap();
    assert_eq!(encoded, "0");
}

#[test]
fn radix_encode_i32_roundtrip() {
    let alphabet = "0123456789ABCDEF";
    for value in [1, 42, 255, 1024, 65535] {
        let encoded = RadixUtil::encode_i32(alphabet, value).unwrap();
        let decoded = RadixUtil::decode_to_i32(alphabet, &encoded).unwrap();
        assert_eq!(decoded, value, "roundtrip failed for {}", value);
    }
}

// ── encode_i64 / decode ──

#[test]
fn radix_encode_i64_large() {
    let alphabet = "0123456789ABCDEF";
    let encoded = RadixUtil::encode_i64(alphabet, 123456789012345i64).unwrap();
    let decoded = RadixUtil::decode(alphabet, &encoded).unwrap();
    assert_eq!(decoded, 123456789012345i64);
}

#[test]
fn radix_encode_i64_negative_error() {
    let alphabet = "0123456789";
    let result = RadixUtil::encode_i64(alphabet, -1);
    assert!(result.is_err());
}

// ── decode_to_i32 ──

#[test]
fn radix_decode_to_i32() {
    let alphabet = "0123456789ABCDEF";
    let result = RadixUtil::decode_to_i32(alphabet, "FF").unwrap();
    assert_eq!(result, 255);
}

#[test]
fn radix_decode_invalid_char() {
    let alphabet = "0123456789";
    let result = RadixUtil::decode(alphabet, "XYZ");
    assert!(result.is_err());
}
