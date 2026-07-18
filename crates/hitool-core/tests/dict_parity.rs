//! dict parity tests
//! 对齐: hutool-core DictTest

use hitool_core::{Dict, DictUtil};
use serde_json::json;

#[test]
fn dict_create() {
    let dict = DictUtil::create();
    assert!(dict.is_empty());
}

#[test]
fn dict_of() {
    let dict = DictUtil::of(&[
        ("name", json!("Alice")),
        ("age", json!(30)),
        ("active", json!(true)),
    ]);
    assert_eq!(dict.len(), 3);
    assert_eq!(dict.get("name").unwrap().as_str().unwrap(), "Alice");
    assert_eq!(dict.get("age").unwrap().as_i64().unwrap(), 30);
    assert!(dict.get("active").unwrap().as_bool().unwrap());
}

#[test]
fn dict_get_str() {
    let dict = DictUtil::of(&[("name", json!("Alice"))]);
    assert_eq!(DictUtil::get_str(&dict, "name"), Some("Alice".to_string()));
    assert_eq!(DictUtil::get_str(&dict, "missing"), None);
}

#[test]
fn dict_get_int() {
    let dict = DictUtil::of(&[("age", json!(30))]);
    assert_eq!(DictUtil::get_int(&dict, "age"), Some(30));
    assert_eq!(DictUtil::get_int(&dict, "missing"), None);
}

#[test]
fn dict_get_float() {
    let dict = DictUtil::of(&[("score", json!(95.5))]);
    assert_eq!(DictUtil::get_float(&dict, "score"), Some(95.5));
    assert_eq!(DictUtil::get_float(&dict, "missing"), None);
}

#[test]
fn dict_get_bool() {
    let dict = DictUtil::of(&[("active", json!(true))]);
    assert_eq!(DictUtil::get_bool(&dict, "active"), Some(true));
    assert_eq!(DictUtil::get_bool(&dict, "missing"), None);
}

#[test]
fn dict_set() {
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "name", json!("Alice"));
    assert_eq!(dict.len(), 1);
    assert_eq!(dict.get("name").unwrap().as_str().unwrap(), "Alice");
}

#[test]
fn dict_contains_key() {
    let dict = DictUtil::of(&[("name", json!("Alice"))]);
    assert!(DictUtil::contains_key(&dict, "name"));
    assert!(!DictUtil::contains_key(&dict, "missing"));
}

#[test]
fn dict_remove() {
    let mut dict = DictUtil::of(&[("name", json!("Alice")), ("age", json!(30))]);
    let removed = DictUtil::remove(&mut dict, "name");
    assert_eq!(removed.unwrap().as_str().unwrap(), "Alice");
    assert_eq!(dict.len(), 1);
    assert!(!DictUtil::contains_key(&dict, "name"));
}

#[test]
fn dict_remove_missing() {
    let mut dict = DictUtil::create();
    let removed = DictUtil::remove(&mut dict, "missing");
    assert!(removed.is_none());
}

#[test]
fn dict_chaining() {
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "a", json!(1));
    DictUtil::set(&mut dict, "b", json!(2));
    DictUtil::set(&mut dict, "c", json!(3));
    assert_eq!(dict.len(), 3);
    assert_eq!(DictUtil::get_int(&dict, "a"), Some(1));
    assert_eq!(DictUtil::get_int(&dict, "b"), Some(2));
    assert_eq!(DictUtil::get_int(&dict, "c"), Some(3));
}

#[test]
fn dict_nested_values() {
    let dict = DictUtil::of(&[
        ("user", json!({"name": "Alice", "age": 30})),
        ("tags", json!(["admin", "user"])),
    ]);
    let user = dict.get("user").unwrap();
    assert_eq!(user.get("name").unwrap().as_str().unwrap(), "Alice");
    let tags = dict.get("tags").unwrap();
    assert_eq!(tags.as_array().unwrap().len(), 2);
}

#[test]
fn dict_overwrite() {
    let mut dict = DictUtil::of(&[("name", json!("Alice"))]);
    DictUtil::set(&mut dict, "name", json!("Bob"));
    assert_eq!(DictUtil::get_str(&dict, "name"), Some("Bob".to_string()));
}
