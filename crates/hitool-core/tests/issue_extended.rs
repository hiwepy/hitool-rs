//! hutool-core Issue 回归测试扩展
//! 对齐: hutool-core Issue*Test 系列
//!
//! 覆盖更多 hutool Issue 场景，验证 hitool-rs 已编译模块的功能正确性。

use hitool_core::{
    CollUtil, ListUtil, BooleanUtil, HexUtil, HashUtil, IdcardUtil, PhoneUtil,
    DesensitizedUtil, VersionUtil, PageUtil, CreditCodeUtil, NumberUtil,
    RandomUtil, ReflectUtil, ReUtil, ArrayUtil, DictUtil, MapUtil, EscapeUtil,
};
use hitool_core::{Dict};
use serde_json::json;

// ── Issue: 数组操作 ──

#[test]
fn issue_array_is_empty() {
    assert!(ArrayUtil::is_empty(&[] as &[i32]));
    assert!(!ArrayUtil::is_empty(&[1, 2, 3]));
}

#[test]
fn issue_array_contains() {
    assert!(ArrayUtil::contains(&[1, 2, 3], &2));
    assert!(!ArrayUtil::contains(&[1, 2, 3], &5));
}

#[test]
fn issue_array_index_of() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3, 2], &2), Some(1));
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3, 2], &2), Some(3));
}

#[test]
fn issue_array_append() {
    let result = ArrayUtil::append(&[1, 2], &[3, 4]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn issue_array_insert() {
    let result = ArrayUtil::insert(&[1, 2, 3], 1, &[10, 20]);
    assert_eq!(result, vec![1, 10, 20, 2, 3]);
}

#[test]
fn issue_array_remove() {
    let result = ArrayUtil::remove(&[1, 2, 3, 4], 2);
    assert_eq!(result, vec![1, 2, 4]);
}

#[test]
fn issue_array_sub() {
    let result = ArrayUtil::sub(&[1, 2, 3, 4, 5], 1, 3);
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn issue_array_split() {
    let result = ArrayUtil::split(&[1, 2, 3, 4, 5], 2);
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn issue_array_sort() {
    let mut arr = vec![3, 1, 4, 1, 5];
    ArrayUtil::sort(&mut arr);
    assert_eq!(arr, vec![1, 1, 3, 4, 5]);
}

#[test]
fn issue_array_reverse() {
    let mut arr = vec![1, 2, 3, 4, 5];
    ArrayUtil::reverse(&mut arr);
    assert_eq!(arr, vec![5, 4, 3, 2, 1]);
}

#[test]
fn issue_array_join() {
    assert_eq!(ArrayUtil::join(&[1, 2, 3], ", "), "1, 2, 3");
}

#[test]
fn issue_array_to_string() {
    assert_eq!(ArrayUtil::to_string(&[1, 2, 3]), "[1, 2, 3]");
}

// ── Issue: 字典操作 ──

#[test]
fn issue_dict_create_set_get() {
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "name", json!("Alice"));
    DictUtil::set(&mut dict, "age", json!(30));
    assert_eq!(DictUtil::get_str(&dict, "name"), Some("Alice".to_string()));
    assert_eq!(DictUtil::get_int(&dict, "age"), Some(30));
}

#[test]
fn issue_dict_of() {
    let dict = DictUtil::of(&[
        ("a", json!(1)),
        ("b", json!(2)),
    ]);
    assert_eq!(dict.len(), 2);
}

#[test]
fn issue_dict_contains_key() {
    let dict = DictUtil::of(&[("name", json!("Alice"))]);
    assert!(DictUtil::contains_key(&dict, "name"));
    assert!(!DictUtil::contains_key(&dict, "missing"));
}

#[test]
fn issue_dict_remove() {
    let mut dict = DictUtil::of(&[("a", json!(1)), ("b", json!(2))]);
    DictUtil::remove(&mut dict, "a");
    assert_eq!(dict.len(), 1);
    assert!(!DictUtil::contains_key(&dict, "a"));
}

// ── Issue: Map 操作 ──

#[test]
fn issue_map_of() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("a"), Some(&1));
}

#[test]
fn issue_map_filter() {
    let map = MapUtil::of(&[("a", 1), ("b", 2), ("c", 3)]);
    let filtered = MapUtil::filter(&map, |_k, v| *v > 1);
    assert_eq!(filtered.len(), 2);
}

#[test]
fn issue_map_merge() {
    let left = MapUtil::of(&[("a", 1)]);
    let right = MapUtil::of(&[("b", 2)]);
    let merged = MapUtil::merge(left, right);
    assert_eq!(merged.len(), 2);
}

#[test]
fn issue_map_keys_values() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    let mut keys = MapUtil::keys(&map);
    keys.sort();
    assert_eq!(keys, vec!["a", "b"]);
}

// ── Issue: 转义操作 ──

#[test]
fn issue_escape_html() {
    assert_eq!(EscapeUtil::escape_html("<div>test</div>"), "&lt;div&gt;test&lt;/div&gt;");
    assert_eq!(EscapeUtil::unescape_html("&lt;div&gt;test&lt;/div&gt;"), "<div>test</div>");
}

#[test]
fn issue_escape_java() {
    let input = "line1\nline2\t\"hello\"";
    let escaped = EscapeUtil::escape_java(input);
    let unescaped = EscapeUtil::unescape_java(&escaped);
    assert_eq!(unescaped, input);
}

#[test]
fn issue_escape_sql() {
    assert_eq!(EscapeUtil::escape_sql("it's"), "it''s");
}

// ── Issue: 正则表达式 ──

#[test]
fn issue_re_find_all() {
    let result = ReUtil::find_all(r"\d+", "abc123def456");
    assert_eq!(result, vec!["123", "456"]);
}

#[test]
fn issue_re_replace() {
    assert_eq!(ReUtil::replace_all(r"\d+", "abc123def456", "X"), "abcXdefX");
    assert_eq!(ReUtil::replace_first(r"\d+", "abc123def456", "X"), "abcXdef456");
}

#[test]
fn issue_re_split() {
    let result = ReUtil::split(r"\s+", "hello world  test");
    assert_eq!(result, vec!["hello", "world", "test"]);
}

#[test]
fn issue_re_is_email() {
    assert!(ReUtil::is_email("test@example.com"));
    assert!(!ReUtil::is_email("not-email"));
}

#[test]
fn issue_re_is_mobile() {
    assert!(ReUtil::is_mobile("13800138000"));
    assert!(!ReUtil::is_mobile("12345678901"));
}

// ── Issue: 数字工具 ──

#[test]
fn issue_number_div() {
    assert_eq!(NumberUtil::div(10.0, 2.0).unwrap(), 5.0);
    assert!(NumberUtil::div(1.0, 0.0).is_err());
}

#[test]
fn issue_number_is_number() {
    assert!(NumberUtil::is_number("123"));
    assert!(NumberUtil::is_number("3.14"));
    assert!(!NumberUtil::is_number("abc"));
}

#[test]
fn issue_number_parse() {
    assert_eq!(NumberUtil::parse_int("42", 0), 42);
    assert_eq!(NumberUtil::parse_int("abc", 0), 0);
    assert_eq!(NumberUtil::parse_double("3.14", 0.0), 3.14);
}

// ── Issue: 随机工具 ──

#[test]
fn issue_random_int_range() {
    for _ in 0..100 {
        let val = RandomUtil::random_int_range(5, 10);
        assert!(val >= 5 && val < 10);
    }
}

#[test]
fn issue_random_string() {
    let s = RandomUtil::random_string(10);
    assert_eq!(s.len(), 10);
    assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
}

// ── Issue: 反射工具 ──

#[test]
fn issue_reflect_is_basic_type() {
    assert!(ReflectUtil::is_basic_type::<i32>());
    assert!(ReflectUtil::is_basic_type::<f64>());
    assert!(!ReflectUtil::is_basic_type::<String>());
}

#[test]
fn issue_reflect_type_eq() {
    assert!(ReflectUtil::type_eq::<i32, i32>());
    assert!(!ReflectUtil::type_eq::<i32, f64>());
}

// ── Issue: 版本比较 ──

#[test]
fn issue_version_compare() {
    assert!(VersionUtil::is_less_than("1.0.0", "2.0.0"));
    assert!(VersionUtil::is_greater_than("2.0.0", "1.0.0"));
    assert!(VersionUtil::is_greater_than_or_equal("1.0.0", "1.0.0"));
}

// ── Issue: 分页工具 ──

#[test]
fn issue_page_total() {
    assert_eq!(PageUtil::total_page_i32(100, 10).unwrap(), 10);
    assert_eq!(PageUtil::total_page_i32(101, 10).unwrap(), 11);
    assert_eq!(PageUtil::total_page_i32(0, 10).unwrap(), 0);
}

// ── Issue: 身份证验证 ──

#[test]
fn issue_idcard_valid() {
    assert!(IdcardUtil::is_valid_card_18("11010519491231002X"));
    assert!(!IdcardUtil::is_valid_card_18("invalid"));
}

#[test]
fn issue_idcard_gender() {
    let gender = IdcardUtil::get_gender_by_id_card("11010519491231002X").unwrap();
    assert_eq!(gender, 0);
}

// ── Issue: 手机号验证 ──

#[test]
fn issue_phone_mobile() {
    assert!(PhoneUtil::is_mobile("13800138000"));
    assert!(!PhoneUtil::is_mobile("12345678901"));
}

#[test]
fn issue_phone_hide() {
    let hidden = PhoneUtil::hide_before("13800138000");
    assert!(hidden.starts_with("***"));
}

// ── Issue: 脱敏处理 ──

#[test]
fn issue_desensitize_phone() {
    let result = DesensitizedUtil::mobile_phone(Some("13800138000"));
    assert!(result.contains("***"));
}

// ── Issue: 信用代码 ──

#[test]
fn issue_credit_code() {
    assert!(CreditCodeUtil::is_credit_code_simple("91350100M000100Y43"));
    assert!(!CreditCodeUtil::is_credit_code_simple("invalid"));
}

// ── Issue: 布尔工具 ──

#[test]
fn issue_boolean_is_true() {
    assert!(BooleanUtil::is_true(Some(true)));
    assert!(!BooleanUtil::is_true(Some(false)));
    assert!(!BooleanUtil::is_true(None));
}

// ── Issue: 十六进制 ──

#[test]
fn issue_hex_encode_decode() {
    let encoded = HexUtil::encode_hex(b"Hello");
    let decoded = HexUtil::decode_hex(&encoded).unwrap();
    assert_eq!(decoded, b"Hello");
}

// ── Issue: 哈希 ──

#[test]
fn issue_hash_fnv() {
    let h1 = HashUtil::fnv_hash("hello");
    let h2 = HashUtil::fnv_hash("hello");
    assert_eq!(h1, h2);
    let h3 = HashUtil::fnv_hash("world");
    assert_ne!(h1, h3);
}
