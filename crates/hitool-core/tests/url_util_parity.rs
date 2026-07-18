//! url_util parity tests
//! 对齐: hutool-core URLUtilTest

use hitool_core::UrlUtil;

#[test]
fn is_url_http() {
    assert!(UrlUtil::is_url("http://example.com"));
    assert!(UrlUtil::is_url("https://example.com"));
    assert!(UrlUtil::is_url("ftp://example.com"));
}

#[test]
fn is_url_invalid() {
    assert!(!UrlUtil::is_url("not-a-url"));
    assert!(!UrlUtil::is_url(""));
}

#[test]
fn is_http_valid() {
    assert!(UrlUtil::is_http("http://example.com"));
    assert!(!UrlUtil::is_http("https://example.com"));
}

#[test]
fn is_https_valid() {
    assert!(UrlUtil::is_https("https://example.com"));
    assert!(!UrlUtil::is_https("http://example.com"));
}

#[test]
fn get_host_basic() {
    assert_eq!(UrlUtil::get_host("http://example.com/path"), Some("example.com"));
    assert_eq!(UrlUtil::get_host("https://example.com"), Some("example.com"));
}

#[test]
fn get_host_with_port() {
    assert_eq!(UrlUtil::get_host("http://example.com:8080/path"), Some("example.com:8080"));
}

#[test]
fn get_path_basic() {
    assert_eq!(UrlUtil::get_path("http://example.com/path/to/resource"), "/path/to/resource");
}

#[test]
fn get_path_root() {
    assert_eq!(UrlUtil::get_path("http://example.com"), "/");
}

#[test]
fn get_protocol_http() {
    assert_eq!(UrlUtil::get_protocol("http://example.com"), Some("http"));
}

#[test]
fn get_protocol_https() {
    assert_eq!(UrlUtil::get_protocol("https://example.com"), Some("https"));
}

#[test]
fn get_protocol_ftp() {
    assert_eq!(UrlUtil::get_protocol("ftp://example.com"), Some("ftp"));
}

#[test]
fn get_protocol_none() {
    assert_eq!(UrlUtil::get_protocol("example.com"), None);
}

#[test]
fn encode_basic() {
    assert_eq!(UrlUtil::encode("hello world"), "hello+world");
    assert_eq!(UrlUtil::encode("hello"), "hello");
}

#[test]
fn encode_special_chars() {
    assert_eq!(UrlUtil::encode("a=b&c=d"), "a%3Db%26c%3Dd");
}

#[test]
fn decode_basic() {
    assert_eq!(UrlUtil::decode("hello+world"), "hello world");
    assert_eq!(UrlUtil::decode("hello"), "hello");
}

#[test]
fn decode_special_chars() {
    assert_eq!(UrlUtil::decode("a%3Db%26c%3Dd"), "a=b&c=d");
}

#[test]
fn encode_decode_roundtrip() {
    for input in ["hello world", "a=b&c=d", "test@example.com"] {
        let encoded = UrlUtil::encode(input);
        let decoded = UrlUtil::decode(&encoded);
        assert_eq!(decoded, input);
    }
}

#[test]
fn normalize_with_protocol() {
    assert_eq!(UrlUtil::normalize("http://example.com"), "http://example.com");
    assert_eq!(UrlUtil::normalize("https://example.com"), "https://example.com");
}

#[test]
fn normalize_without_protocol() {
    assert_eq!(UrlUtil::normalize("example.com"), "http://example.com");
}

#[test]
fn complete_url_basic() {
    assert_eq!(UrlUtil::complete_url("http://example.com", "/path"), "http://example.com/path");
    assert_eq!(UrlUtil::complete_url("http://example.com/", "path"), "http://example.com/path");
}
