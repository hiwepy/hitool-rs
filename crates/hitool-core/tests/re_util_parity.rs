//! re_util parity tests
//! 对齐: hutool-core ReUtilTest

use hitool_core::ReUtil;

// ── 匹配判断 ──

#[test]
fn is_match_basic() {
    assert!(ReUtil::is_match(r"^\d+$", "12345"));
    assert!(!ReUtil::is_match(r"^\d+$", "abc"));
}

#[test]
fn is_match_partial() {
    assert!(ReUtil::is_match(r"\d+", "abc123def"));
}

#[test]
fn is_match_invalid_pattern() {
    assert!(!ReUtil::is_match("[invalid", "test"));
}

// ── 提取操作 ──

#[test]
fn find_basic() {
    assert_eq!(ReUtil::find(r"\d+", "abc123def"), Some("123".to_string()));
}

#[test]
fn find_no_match() {
    assert_eq!(ReUtil::find(r"\d+", "abcdef"), None);
}

#[test]
fn group0_basic() {
    assert_eq!(ReUtil::group0(r"(\d+)", "abc123def"), Some("123".to_string()));
}

#[test]
fn group1_basic() {
    assert_eq!(ReUtil::group1(r"(\d+)-(\d+)", "123-456"), Some("123".to_string()));
}

#[test]
fn group_no_match() {
    assert_eq!(ReUtil::group(r"(\d+)", "abcdef", 1), None);
}

// ── 提取所有匹配 ──

#[test]
fn find_all_basic() {
    let result = ReUtil::find_all(r"\d+", "abc123def456");
    assert_eq!(result, vec!["123", "456"]);
}

#[test]
fn find_all_no_match() {
    let result = ReUtil::find_all(r"\d+", "abcdef");
    assert!(result.is_empty());
}

#[test]
fn find_all_groups_basic() {
    let result = ReUtil::find_all_groups(r"(\d+)-(\w+)", "123-abc 456-def", 1);
    assert_eq!(result, vec!["123", "456"]);
}

// ── 替换操作 ──

#[test]
fn replace_all_basic() {
    assert_eq!(ReUtil::replace_all(r"\d+", "abc123def456", "X"), "abcXdefX");
}

#[test]
fn replace_first_basic() {
    assert_eq!(ReUtil::replace_first(r"\d+", "abc123def456", "X"), "abcXdef456");
}

#[test]
fn replace_no_match() {
    assert_eq!(ReUtil::replace_all(r"\d+", "abcdef", "X"), "abcdef");
}

// ── 分割操作 ──

#[test]
fn split_basic() {
    let result = ReUtil::split(r"\s+", "hello world  test");
    assert_eq!(result, vec!["hello", "world", "test"]);
}

// ── 常用正则 ──

#[test]
fn is_email_valid() {
    assert!(ReUtil::is_email("test@example.com"));
    assert!(ReUtil::is_email("user.name+tag@domain.co"));
}

#[test]
fn is_email_invalid() {
    assert!(!ReUtil::is_email("not-an-email"));
    assert!(!ReUtil::is_email("@no-user.com"));
}

#[test]
fn is_ipv4_valid() {
    assert!(ReUtil::is_ipv4("192.168.1.1"));
    assert!(ReUtil::is_ipv4("0.0.0.0"));
}

#[test]
fn is_ipv4_invalid() {
    
    assert!(!ReUtil::is_ipv4("not-an-ip"));
}

#[test]
fn is_url_valid() {
    assert!(ReUtil::is_url("https://example.com"));
    assert!(ReUtil::is_url("http://test.org/path"));
}

#[test]
fn is_url_invalid() {
    assert!(!ReUtil::is_url("not-a-url"));
    assert!(!ReUtil::is_url("ftp://example.com"));
}

#[test]
fn is_chinese_valid() {
    assert!(ReUtil::is_chinese("你好世界"));
}

#[test]
fn is_chinese_invalid() {
    assert!(!ReUtil::is_chinese("hello"));
    assert!(!ReUtil::is_chinese("你好world"));
}

#[test]
fn is_mobile_valid() {
    assert!(ReUtil::is_mobile("13800138000"));
    assert!(ReUtil::is_mobile("15912345678"));
}

#[test]
fn is_mobile_invalid() {
    assert!(!ReUtil::is_mobile("12345678901"));
    assert!(!ReUtil::is_mobile("1380013800"));
}

// ── 提取数字 ──

#[test]
fn extract_number_basic() {
    assert_eq!(ReUtil::extract_number("abc123def"), Some(123));
}

#[test]
fn extract_number_no_match() {
    assert_eq!(ReUtil::extract_number("abcdef"), None);
}

#[test]
fn extract_numbers_basic() {
    assert_eq!(ReUtil::extract_numbers("abc123def456"), vec![123, 456]);
}

// ── 转义 ──

#[test]
fn escape_special_basic() {
    assert_eq!(ReUtil::escape_special("a.b*c"), r"a\.b\*c");
}

#[test]
fn escape_special_no_special() {
    assert_eq!(ReUtil::escape_special("abc"), "abc");
}
