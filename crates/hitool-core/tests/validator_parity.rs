//! validator parity tests
//! 对齐: hutool-core ValidatorTest

use hitool_core::Validator;

// ── 空值验证 ──

#[test]
fn is_empty_none() {
    assert!(Validator::is_empty(None));
}

#[test]
fn is_empty_empty_string() {
    assert!(Validator::is_empty(Some("")));
}

#[test]
fn is_empty_whitespace() {
    assert!(!Validator::is_empty(Some("   ")));
}

#[test]
fn is_empty_non_empty() {
    assert!(!Validator::is_empty(Some("hello")));
}

#[test]
fn is_not_empty_some() {
    assert!(Validator::is_not_empty(Some("hello")));
}

#[test]
fn is_not_empty_none() {
    assert!(!Validator::is_not_empty(None));
}

// ── 长度验证 ──

#[test]
fn is_between_valid() {
    assert!(Validator::is_between("hello", 3, 10));
    assert!(Validator::is_between("hello", 5, 5));
}

#[test]
fn is_between_too_short() {
    assert!(!Validator::is_between("hi", 3, 10));
}

#[test]
fn is_between_too_long() {
    assert!(!Validator::is_between("hello world", 3, 5));
}

#[test]
fn min_length_valid() {
    assert!(Validator::min_length("hello", 3));
    assert!(Validator::min_length("hello", 5));
}

#[test]
fn min_length_invalid() {
    assert!(!Validator::min_length("hi", 3));
}

#[test]
fn max_length_valid() {
    assert!(Validator::max_length("hello", 5));
    assert!(Validator::max_length("hello", 10));
}

#[test]
fn max_length_invalid() {
    assert!(!Validator::max_length("hello world", 5));
}

// ── 格式验证 ──

#[test]
fn is_email_valid() {
    assert!(Validator::is_email("test@example.com"));
    assert!(Validator::is_email("user.name+tag@domain.co"));
}

#[test]
fn is_email_invalid() {
    assert!(!Validator::is_email("not-an-email"));
    assert!(!Validator::is_email("@no-user.com"));
    assert!(!Validator::is_email(""));
}

#[test]
fn is_mobile_valid() {
    assert!(Validator::is_mobile("13800138000"));
    assert!(Validator::is_mobile("15912345678"));
    assert!(Validator::is_mobile("18688889999"));
}

#[test]
fn is_mobile_invalid() {
    assert!(!Validator::is_mobile("12345678901"));
    assert!(!Validator::is_mobile("1380013800"));
    assert!(!Validator::is_mobile(""));
}

#[test]
fn is_ipv4_valid() {
    assert!(Validator::is_ipv4("192.168.1.1"));
    assert!(Validator::is_ipv4("0.0.0.0"));
    assert!(Validator::is_ipv4("255.255.255.255"));
}

#[test]
fn is_ipv4_invalid() {
    assert!(!Validator::is_ipv4("not-an-ip"));
    assert!(!Validator::is_ipv4(""));
}

#[test]
fn is_url_valid() {
    assert!(Validator::is_url("https://example.com"));
    assert!(Validator::is_url("http://test.org/path"));
}

#[test]
fn is_url_invalid() {
    assert!(!Validator::is_url("not-a-url"));
    assert!(!Validator::is_url(""));
}

#[test]
fn is_chinese_valid() {
    assert!(Validator::is_chinese("你好世界"));
}

#[test]
fn is_chinese_invalid() {
    assert!(!Validator::is_chinese("hello"));
    assert!(!Validator::is_chinese("你好world"));
}

// ── 数字验证 ──

#[test]
fn is_number_valid() {
    assert!(Validator::is_number("123"));
    assert!(Validator::is_number("3.14"));
    assert!(Validator::is_number("-1"));
}

#[test]
fn is_number_invalid() {
    assert!(!Validator::is_number("abc"));
    assert!(!Validator::is_number(""));
}

#[test]
fn is_integer_valid() {
    assert!(Validator::is_integer("123"));
    assert!(Validator::is_integer("-1"));
}

#[test]
fn is_integer_invalid() {
    assert!(!Validator::is_integer("3.14"));
    assert!(!Validator::is_integer("abc"));
}

// ── 范围验证 ──

#[test]
fn is_between_i64_valid() {
    assert!(Validator::is_between_i64(5, 1, 10));
    assert!(Validator::is_between_i64(1, 1, 10));
    assert!(Validator::is_between_i64(10, 1, 10));
}

#[test]
fn is_between_i64_invalid() {
    assert!(!Validator::is_between_i64(0, 1, 10));
    assert!(!Validator::is_between_i64(11, 1, 10));
}

#[test]
fn is_between_f64_valid() {
    assert!(Validator::is_between_f64(5.0, 1.0, 10.0));
}

#[test]
fn is_between_f64_invalid() {
    assert!(!Validator::is_between_f64(0.0, 1.0, 10.0));
}

// ── 正则验证 ──

#[test]
fn is_match_regex_valid() {
    assert!(Validator::is_match_regex(r"^\d+$", "12345"));
}

#[test]
fn is_match_regex_invalid() {
    assert!(!Validator::is_match_regex(r"^\d+$", "abc"));
}

// ── 集合验证 ──

#[test]
fn is_not_empty_collection_valid() {
    assert!(Validator::is_not_empty_collection(&[1, 2, 3]));
}

#[test]
fn is_not_empty_collection_empty() {
    assert!(!Validator::is_not_empty_collection(&[] as &[i32]));
}

#[test]
fn is_empty_collection_empty() {
    assert!(Validator::is_empty_collection(&[] as &[i32]));
}

#[test]
fn is_empty_collection_not_empty() {
    assert!(!Validator::is_empty_collection(&[1, 2, 3]));
}

// ── 通用验证 ──

#[test]
fn validate_not_empty_valid() {
    assert!(Validator::validate_not_empty(Some("hello"), "name").is_ok());
}

#[test]
fn validate_not_empty_invalid() {
    assert!(Validator::validate_not_empty(None, "name").is_err());
    assert!(Validator::validate_not_empty(Some(""), "name").is_err());
}

#[test]
fn validate_between_valid() {
    assert!(Validator::validate_between("hello", 3, 10, "name").is_ok());
}

#[test]
fn validate_between_invalid() {
    assert!(Validator::validate_between("hi", 3, 10, "name").is_err());
    assert!(Validator::validate_between("hello world", 3, 5, "name").is_err());
}
