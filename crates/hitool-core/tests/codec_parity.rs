//! codec module parity tests
//! 对齐: hutool-core Base64Test/HexTest

use hitool_core::{base64_encode, base64_decode, base64_url_encode, base64_url_decode, hex_encode, hex_decode, percent_encode_component, percent_decode};

// ── base64 ──

#[test]
fn base64_encode_decode() {
    let encoded = base64_encode("hello world");
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

#[test]
fn base64_empty() {
    let encoded = base64_encode("");
    assert_eq!(encoded, "");
}

#[test]
fn base64_special_chars() {
    let input = "hello\nworld\t!";
    let encoded = base64_encode(input);
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), input);
}

// ── base64_url ──

#[test]
fn base64_url_encode_decode() {
    let encoded = base64_url_encode("hello+world/test");
    let decoded = base64_url_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello+world/test");
}

// ── hex ──

#[test]
fn hex_encode_decode() {
    let encoded = hex_encode("hello");
    let decoded = hex_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello");
}

#[test]
fn hex_encode_empty() {
    let encoded = hex_encode("");
    assert_eq!(encoded, "");
}

#[test]
fn hex_encode_bytes() {
    let encoded = hex_encode(&[0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(encoded.to_lowercase(), "cafebabe");
}

// ── percent ──

#[test]
fn percent_encode_decode() {
    let encoded = percent_encode_component("hello world");
    assert_eq!(encoded, "hello%20world");
    let decoded = percent_decode(&encoded).unwrap();
    assert_eq!(decoded, "hello world");
}

#[test]
fn percent_encode_special_chars() {
    let encoded = percent_encode_component("a=b&c=d");
    assert!(encoded.contains("%3D"));
    assert!(encoded.contains("%26"));
}
