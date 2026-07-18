//! type_util parity tests
//! 对齐: hutool-core TypeUtilTest

use hitool_core::TypeUtil;

// ── 类型名称 ──

#[test]
fn type_name_i32() {
    assert_eq!(TypeUtil::type_name::<i32>(), "i32");
}

#[test]
fn type_name_i64() {
    assert_eq!(TypeUtil::type_name::<i64>(), "i64");
}

#[test]
fn type_name_f64() {
    assert_eq!(TypeUtil::type_name::<f64>(), "f64");
}

#[test]
fn type_name_bool() {
    assert_eq!(TypeUtil::type_name::<bool>(), "bool");
}

#[test]
fn type_name_string() {
    assert_eq!(TypeUtil::type_name::<String>(), "alloc::string::String");
}

#[test]
fn type_name_vec() {
    let name = TypeUtil::type_name::<Vec<i32>>();
    assert!(name.contains("Vec"));
}

#[test]
fn type_name_of_value() {
    let val = 42i32;
    assert_eq!(TypeUtil::type_name_of(&val), "i32");
}

// ── 类型比较 ──

#[test]
fn is_assignable_from_same() {
    assert!(TypeUtil::is_assignable_from::<i32, i32>());
}

#[test]
fn is_assignable_from_different() {
    assert!(!TypeUtil::is_assignable_from::<i32, f64>());
}

#[test]
fn is_instance_of_i32() {
    let val = 42i32;
    assert!(TypeUtil::is_instance_of::<i32>(&val));
    assert!(!TypeUtil::is_instance_of::<f64>(&val));
}

// ── 基础类型判断 ──

#[test]
fn is_basic_type_integers() {
    assert!(TypeUtil::is_basic_type::<i8>());
    assert!(TypeUtil::is_basic_type::<i16>());
    assert!(TypeUtil::is_basic_type::<i32>());
    assert!(TypeUtil::is_basic_type::<i64>());
    assert!(TypeUtil::is_basic_type::<u8>());
    assert!(TypeUtil::is_basic_type::<u16>());
    assert!(TypeUtil::is_basic_type::<u32>());
    assert!(TypeUtil::is_basic_type::<u64>());
}

#[test]
fn is_basic_type_floats() {
    assert!(TypeUtil::is_basic_type::<f32>());
    assert!(TypeUtil::is_basic_type::<f64>());
}

#[test]
fn is_basic_type_bool_char() {
    assert!(TypeUtil::is_basic_type::<bool>());
    assert!(TypeUtil::is_basic_type::<char>());
}

#[test]
fn is_basic_type_not_string() {
    assert!(!TypeUtil::is_basic_type::<String>());
}

#[test]
fn is_basic_type_not_vec() {
    assert!(!TypeUtil::is_basic_type::<Vec<i32>>());
}

#[test]
fn is_simple_type_includes_string() {
    assert!(TypeUtil::is_simple_type::<String>());
    assert!(TypeUtil::is_simple_type::<i32>());
    assert!(!TypeUtil::is_simple_type::<Vec<i32>>());
}

// ── 数值类型判断 ──

#[test]
fn is_number_integers() {
    assert!(TypeUtil::is_number::<i32>());
    assert!(TypeUtil::is_number::<i64>());
    assert!(TypeUtil::is_number::<u32>());
    assert!(TypeUtil::is_number::<u64>());
}

#[test]
fn is_number_floats() {
    assert!(TypeUtil::is_number::<f32>());
    assert!(TypeUtil::is_number::<f64>());
}

#[test]
fn is_number_not_bool() {
    assert!(!TypeUtil::is_number::<bool>());
}

#[test]
fn is_number_not_string() {
    assert!(!TypeUtil::is_number::<String>());
}

#[test]
fn is_integer_i32() {
    assert!(TypeUtil::is_integer::<i32>());
    assert!(TypeUtil::is_integer::<i64>());
    assert!(TypeUtil::is_integer::<u32>());
}

#[test]
fn is_integer_not_float() {
    assert!(!TypeUtil::is_integer::<f32>());
    assert!(!TypeUtil::is_integer::<f64>());
}

#[test]
fn is_float_f32() {
    assert!(TypeUtil::is_float::<f32>());
    assert!(TypeUtil::is_float::<f64>());
}

#[test]
fn is_float_not_int() {
    assert!(!TypeUtil::is_float::<i32>());
    assert!(!TypeUtil::is_float::<u64>());
}

// ── 容器类型判断 ──

#[test]
fn is_collection_vec() {
    assert!(TypeUtil::is_collection::<Vec<i32>>());
}

#[test]
fn is_collection_not_i32() {
    assert!(!TypeUtil::is_collection::<i32>());
}

#[test]
fn is_map_hashmap() {
    assert!(TypeUtil::is_map::<std::collections::HashMap<String, i32>>());
}

#[test]
fn is_map_not_i32() {
    assert!(!TypeUtil::is_map::<i32>());
}
