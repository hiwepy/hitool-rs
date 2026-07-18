//! object_util parity tests
//! 对齐: hutool-core ObjectUtilTest

use hitool_core::ObjectUtil;

// ── 空值判断 ──

#[test]
fn is_null_none() {
    assert!(ObjectUtil::is_null(None::<&i32>));
}

#[test]
fn is_null_some() {
    assert!(!ObjectUtil::is_null(Some(&42)));
}

#[test]
fn is_not_null_some() {
    assert!(ObjectUtil::is_not_null(Some(&42)));
}

#[test]
fn is_not_null_none() {
    assert!(!ObjectUtil::is_not_null(None::<&i32>));
}

// ── 默认值 ──

#[test]
fn default_if_null_some() {
    assert_eq!(ObjectUtil::default_if_null(Some(&42), &0), 42);
}

#[test]
fn default_if_null_none() {
    assert_eq!(ObjectUtil::default_if_null(None::<&i32>, &0), 0);
}

// ── 相等判断 ──

#[test]
fn equal_both_some() {
    assert!(ObjectUtil::equal(Some(&42), Some(&42)));
    assert!(!ObjectUtil::equal(Some(&42), Some(&43)));
}

#[test]
fn equal_both_none() {
    assert!(ObjectUtil::equal(None::<&i32>, None::<&i32>));
}

#[test]
fn equal_one_none() {
    assert!(!ObjectUtil::equal(Some(&42), None::<&i32>));
    assert!(!ObjectUtil::equal(None::<&i32>, Some(&42)));
}

#[test]
fn not_equal_both_some() {
    assert!(ObjectUtil::not_equal(Some(&42), Some(&43)));
    assert!(!ObjectUtil::not_equal(Some(&42), Some(&42)));
}

// ── 比较操作 ──

#[test]
fn compare_both_some() {
    assert_eq!(ObjectUtil::compare(Some(&1), Some(&2)), -1);
    assert_eq!(ObjectUtil::compare(Some(&2), Some(&2)), 0);
    assert_eq!(ObjectUtil::compare(Some(&3), Some(&2)), 1);
}

#[test]
fn compare_none_values() {
    assert_eq!(ObjectUtil::compare(None::<&i32>, Some(&1)), -1);
    assert_eq!(ObjectUtil::compare(Some(&1), None::<&i32>), 1);
    assert_eq!(ObjectUtil::compare(None::<&i32>, None::<&i32>), 0);
}

// ── 类型判断 ──

#[test]
fn is_basic_type_i32() {
    let val = 42i32;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_f64() {
    let val = 3.14f64;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_bool() {
    let val = true;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_string() {
    let val = "hello".to_string();
    assert!(!ObjectUtil::is_basic_type(&val));
}

// ── 克隆操作 ──

#[test]
fn clone_if_some_some() {
    assert_eq!(ObjectUtil::clone_if_some(Some(&42)), Some(42));
}

#[test]
fn clone_if_some_none() {
    assert_eq!(ObjectUtil::clone_if_some(None::<&i32>), None);
}

// ── 序列化辅助 ──

#[test]
fn to_string_some() {
    assert_eq!(ObjectUtil::to_string(Some(&42)), "42");
    assert_eq!(ObjectUtil::to_string(Some(&"hello")), "hello");
}

#[test]
fn to_string_none() {
    assert_eq!(ObjectUtil::to_string(None::<&i32>), "null");
}

// ── 集合判断 ──

#[test]
fn is_empty_str_none() {
    assert!(ObjectUtil::is_empty_str(None));
}

#[test]
fn is_empty_str_empty() {
    assert!(ObjectUtil::is_empty_str(Some("")));
}

#[test]
fn is_empty_str_non_empty() {
    assert!(!ObjectUtil::is_empty_str(Some("hello")));
}

#[test]
fn is_not_empty_str_some() {
    assert!(ObjectUtil::is_not_empty_str(Some("hello")));
}

#[test]
fn is_not_empty_str_none() {
    assert!(!ObjectUtil::is_not_empty_str(None));
}

#[test]
fn is_not_empty_str_empty() {
    assert!(!ObjectUtil::is_not_empty_str(Some("")));
}
