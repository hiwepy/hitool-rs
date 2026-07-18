//! map_util parity tests
//! 对齐: hutool-core MapUtilTest

use hitool_core::MapUtil;

#[test]
fn map_util_is_empty() {
    let map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    assert!(MapUtil::is_empty(&map));
}

#[test]
fn map_util_is_not_empty() {
    let map = MapUtil::of(&[("a", 1)]);
    assert!(MapUtil::is_not_empty(&map));
}

#[test]
fn map_util_of() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}

#[test]
fn map_util_new_hash_map() {
    let map: std::collections::HashMap<i32, i32> = MapUtil::new_hash_map();
    assert!(map.is_empty());
}

#[test]
fn map_util_get_str() {
    let mut map = std::collections::HashMap::new();
    map.insert("name".to_string(), "Alice".to_string());
    assert_eq!(MapUtil::get_str(&map, &"name".to_string()), Some("Alice"));
    assert_eq!(MapUtil::get_str(&map, &"missing".to_string()), None);
}

#[test]
fn map_util_get_int() {
    let mut map = std::collections::HashMap::new();
    map.insert("age", 30);
    assert_eq!(MapUtil::get_int(&map, &"age"), Some(30));
    assert_eq!(MapUtil::get_int(&map, &"missing"), None);
}

#[test]
fn map_util_get_bool() {
    let mut map = std::collections::HashMap::new();
    map.insert("active", true);
    assert_eq!(MapUtil::get_bool(&map, &"active"), Some(true));
    assert_eq!(MapUtil::get_bool(&map, &"missing"), None);
}

#[test]
fn map_util_put_all() {
    let mut target = MapUtil::of(&[("a", 1)]);
    let source = MapUtil::of(&[("b", 2), ("c", 3)]);
    MapUtil::put_all(&mut target, source);
    assert_eq!(target.len(), 3);
    assert_eq!(target.get("a"), Some(&1));
    assert_eq!(target.get("b"), Some(&2));
    assert_eq!(target.get("c"), Some(&3));
}

#[test]
fn map_util_join() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    let result = MapUtil::join(&map, ", ", "=");
    assert!(result.contains("a=1"));
    assert!(result.contains("b=2"));
}

#[test]
fn map_util_filter() {
    let map = MapUtil::of(&[("a", 1), ("b", 2), ("c", 3)]);
    let filtered = MapUtil::filter(&map, |_k, v| *v > 1);
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered.get("b"), Some(&2));
    assert_eq!(filtered.get("c"), Some(&3));
}

#[test]
fn map_util_merge() {
    let left = MapUtil::of(&[("a", 1)]);
    let right = MapUtil::of(&[("b", 2)]);
    let merged = MapUtil::merge(left, right);
    assert_eq!(merged.len(), 2);
    assert_eq!(merged.get("a"), Some(&1));
    assert_eq!(merged.get("b"), Some(&2));
}

#[test]
fn map_util_keys() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    let mut keys = MapUtil::keys(&map);
    keys.sort();
    assert_eq!(keys, vec!["a", "b"]);
}

#[test]
fn map_util_values() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    let mut values = MapUtil::values(&map);
    values.sort();
    assert_eq!(values, vec![1, 2]);
}

#[test]
fn map_util_inverse() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    let inverse = MapUtil::inverse(&map);
    assert_eq!(inverse.get(&1), Some(&"a"));
    assert_eq!(inverse.get(&2), Some(&"b"));
}
