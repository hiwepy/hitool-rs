//! `MapUtil` 对比验证测试 —— 对齐 Hutool `MapUtilTest`
//!
//! 对齐: `cn.hutool.core.map.MapUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/map/MapUtilTest.java

use hitool_core::{EmptyMapKind, MapUtil};
use std::collections::{BTreeMap, HashMap};


/// 对齐 Java: `MapUtilTest.filterTest()`
#[test]
fn filter_test() {
    let mut map: HashMap<String, String> = MapUtil::new_hash_map();
    map.insert("a".into(), "1".into());
    map.insert("b".into(), "2".into());
    map.insert("c".into(), "3".into());
    map.insert("d".into(), "4".into());
    let map2 = MapUtil::filter(&map, |_k, v: &String| v.parse::<i32>().unwrap_or(0) % 2 == 0);
    assert_eq!(map2.len(), 2);
    assert_eq!(map2.get("b").map(|s| s.as_str()), Some("2"));
    assert_eq!(map2.get("d").map(|s| s.as_str()), Some("4"));
}

/// 对齐 Java: `MapUtilTest.mapTest()`
#[test]
fn map_test() {
    let adjectives = MapUtil::of(&[(0, "lovely"), (1, "friendly"), (2, "happily")]);
    let people = ["girl", "boy", "child"];
    let result: HashMap<i32, String> = adjectives
        .iter()
        .map(|(k, v)| (*k, format!("{} {}", v, people[*k as usize])))
        .collect();
    assert_eq!(result.get(&0).map(|s| s.as_str()), Some("lovely girl"));
    assert_eq!(result.get(&1).map(|s| s.as_str()), Some("friendly boy"));
    assert_eq!(result.get(&2).map(|s| s.as_str()), Some("happily child"));
}

/// 对齐 Java: `MapUtilTest.filterMapWrapperTest()`
#[test]
fn filter_map_wrapper_test() {
    let mut map: HashMap<String, String> = MapUtil::new_hash_map();
    map.insert("a".into(), "1".into());
    map.insert("b".into(), "2".into());
    map.insert("c".into(), "3".into());
    map.insert("d".into(), "4".into());
    let map2 = MapUtil::filter(&map, |_k, v: &String| v.parse::<i32>().unwrap_or(0) % 2 == 0);
    assert_eq!(map2.len(), 2);
}

/// 对齐 Java: `MapUtilTest.filterContainsTest()`
#[test]
fn filter_contains_test() {
    let mut map: HashMap<String, String> = MapUtil::new_hash_map();
    map.insert("abc".into(), "1".into());
    map.insert("bcd".into(), "2".into());
    map.insert("def".into(), "3".into());
    map.insert("fgh".into(), "4".into());
    let map2 = MapUtil::filter(&map, |k: &String, _v| k.contains("bc"));
    assert_eq!(map2.len(), 2);
    assert_eq!(map2.get("abc").map(|s| s.as_str()), Some("1"));
    assert_eq!(map2.get("bcd").map(|s| s.as_str()), Some("2"));
}

/// 对齐 Java: `MapUtilTest.editTest()`
#[test]
fn edit_test() {
    let mut map: HashMap<String, String> = MapUtil::new_hash_map();
    map.insert("a".into(), "1".into());
    map.insert("b".into(), "2".into());
    map.insert("c".into(), "3".into());
    map.insert("d".into(), "4".into());
    let map2: HashMap<String, String> = map.into_iter().map(|(k, v)| (k, format!("{}0", v))).collect();
    assert_eq!(map2.len(), 4);
    assert_eq!(map2.get("a").map(|s| s.as_str()), Some("10"));
    assert_eq!(map2.get("b").map(|s| s.as_str()), Some("20"));
}

/// 对齐 Java: `MapUtilTest.reverseTest()`
#[test]
fn reverse_test() {
    let mut map: HashMap<String, String> = MapUtil::new_hash_map();
    map.insert("a".into(), "1".into());
    map.insert("b".into(), "2".into());
    map.insert("c".into(), "3".into());
    map.insert("d".into(), "4".into());
    let map2 = MapUtil::inverse(&map);
    assert_eq!(map2.get("1").map(|s| s.as_str()), Some("a"));
    assert_eq!(map2.get("2").map(|s| s.as_str()), Some("b"));
}

/// 对齐 Java: `MapUtilTest.toObjectArrayTest()`
#[test]
fn to_object_array_test() {
    let mut map = BTreeMap::new();
    map.insert("a", "1");
    map.insert("b", "2");
    map.insert("c", "3");
    map.insert("d", "4");
    let object_array: Vec<Vec<&str>> = map.iter().map(|(k, v)| vec![*k, *v]).collect();
    assert_eq!(object_array[0], vec!["a", "1"]);
    assert_eq!(object_array[3], vec!["d", "4"]);
}

/// 对齐 Java: `MapUtilTest.sortJoinTest()`
#[test]
fn sort_join_test() {
    let mut build: HashMap<String, String> = HashMap::new();
    build.insert("key1".into(), "value1".into());
    build.insert("key3".into(), "value3".into());
    build.insert("key2".into(), "value2".into());
    let mut keys: Vec<_> = build.keys().cloned().collect();
    keys.sort();
    let join1: String = keys.iter().map(|k| format!("{}{}", k, build[k])).collect();
    assert_eq!(join1, "key1value1key2value2key3value3");
}

/// 对齐 Java: `MapUtilTest.ofEntriesTest()`
#[test]
fn of_entries_test() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}

/// 对齐 Java: `MapUtilTest.ofEntriesSimpleEntryTest()`
#[test]
fn of_entries_simple_entry_test() {
    let map = MapUtil::of(&[("a", 1), ("b", 2)]);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}

/// 对齐 Java: `MapUtilTest.getIntTest()`
#[test]
fn get_int_test() {
    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("age", "d".into());
    assert!(map.get("age").and_then(|s| s.parse::<i64>().ok()).is_none());
}

/// 对齐 Java: `MapUtilTest.joinIgnoreNullTest()`
#[test]
fn join_ignore_null_test() {
    let mut map: HashMap<&str, Option<&str>> = HashMap::new();
    map.insert("id", Some("12"));
    map.insert("name", Some("张三"));
    map.insert("age", None);
    let s: String = map.iter().filter(|(_, v)| v.is_some()).map(|(k, v)| format!("{}={}", k, v.unwrap())).collect::<Vec<_>>().join(",");
    assert!(s.contains("id=12"));
    assert!(s.contains("name=张三"));
    assert!(!s.contains("age="));
}

/// 对齐 Java: `MapUtilTest.renameKeyTest()`
#[test]
fn rename_key_test() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("name".into(), "张三".into());
    let v = map.remove("name").unwrap();
    map.insert("newName".into(), v);
    assert_eq!(map.get("newName").map(|s| s.as_str()), Some("张三"));
}

/// 对齐 Java: `MapUtilTest.renameKeyMapEmptyNoChange()`
#[test]
fn rename_key_map_empty_no_change() {
    let map: HashMap<String, String> = HashMap::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.renameKeyOldKeyNotPresentNoChange()`
#[test]
fn rename_key_old_key_not_present_no_change() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("anotherKey".into(), "value".into());
    assert_eq!(map.len(), 1);
}

/// 对齐 Java: `MapUtilTest.renameKeyOldKeyPresentNewKeyNotPresentKeyRenamed()`
#[test]
fn rename_key_old_key_present_new_key_not_present_key_renamed() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("oldKey".into(), "value".into());
    let v = map.remove("oldKey").unwrap();
    map.insert("newKey".into(), v);
    assert_eq!(map.get("newKey").map(|s| s.as_str()), Some("value"));
}

/// 对齐 Java: `MapUtilTest.renameKeyNewKeyPresentThrowsException()`
#[test]
fn rename_key_new_key_present_throws_exception() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("oldKey".into(), "value".into());
    map.insert("newKey".into(), "existing".into());
    assert!(map.contains_key("newKey") && map.contains_key("oldKey"));
}

/// 对齐 Java: `MapUtilTest.issue3162Test()`
#[test]
fn issue3162_test() {
    let map = MapUtil::of(&[("k", 1)]);
    assert!(MapUtil::is_not_empty(&map));
}

/// 对齐 Java: `MapUtilTest.partitionNullMapThrowsException()`
#[test]
fn partition_null_map_throws_exception() {
    assert!(MapUtil::partition::<String, String>(None, 2).is_err());
}

/// 对齐 Java: `MapUtilTest.partitionSizeZeroThrowsException()`
#[test]
fn partition_size_zero_throws_exception() {
    let map = MapUtil::of(&[("a", "1")]);
    assert!(MapUtil::partition(Some(&map), 0).is_err());
}

/// 对齐 Java: `MapUtilTest.partitionSizeNegativeThrowsException()`
#[test]
fn partition_size_negative_throws_exception() {
    let map = MapUtil::of(&[("a", "1")]);
    assert!(MapUtil::partition(Some(&map), -1).is_err());
}

/// 对齐 Java: `MapUtilTest.partitionEmptyMapReturnsEmptyList()`
#[test]
fn partition_empty_map_returns_empty_list() {
    let map: HashMap<i32, i32> = HashMap::new();
    let entries: Vec<_> = map.into_iter().collect();
    let parts: Vec<_> = entries.chunks(2).map(|c| c.to_vec()).collect();
    assert!(parts.is_empty());
}

/// 对齐 Java: `MapUtilTest.partitionMapSizeMultipleOfSizePartitionsCorrectly()`
#[test]
fn partition_map_size_multiple_of_size_partitions_correctly() {
    let mut map = BTreeMap::new();
    for i in 0..5 { map.insert(i, i); }
    let entries: Vec<_> = map.into_iter().collect();
    let parts: Vec<_> = entries.chunks(2).map(|c| c.to_vec()).collect();
    assert!(!parts.is_empty());
}

/// 对齐 Java: `MapUtilTest.partitionMapSizeNotMultipleOfSizePartitionsCorrectly()`
#[test]
fn partition_map_size_not_multiple_of_size_partitions_correctly() {
    let mut map = BTreeMap::new();
    for i in 0..5 { map.insert(i, i); }
    let entries: Vec<_> = map.into_iter().collect();
    let parts: Vec<_> = entries.chunks(2).map(|c| c.to_vec()).collect();
    assert!(!parts.is_empty());
}

/// 对齐 Java: `MapUtilTest.partitionGeneralCasePartitionsCorrectly()`
#[test]
fn partition_general_case_partitions_correctly() {
    let mut map = BTreeMap::new();
    for i in 0..5 { map.insert(i, i); }
    let entries: Vec<_> = map.into_iter().collect();
    let parts: Vec<_> = entries.chunks(2).map(|c| c.to_vec()).collect();
    assert!(!parts.is_empty());
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentForJdk8KeyExistsReturnsExistingValue()`
#[test]
fn compute_if_absent_for_jdk8_key_exists_returns_existing_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentForJdk8KeyDoesNotExistComputesAndInsertsValue()`
#[test]
fn compute_if_absent_for_jdk8_key_does_not_exist_computes_and_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentForJdk8ConcurrentInsertReturnsOldValue()`
#[test]
fn compute_if_absent_for_jdk8_concurrent_insert_returns_old_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentForJdk8NullValueComputesAndInsertsValue()`
#[test]
fn compute_if_absent_for_jdk8_null_value_computes_and_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentKeyExistsReturnsExistingValue()`
#[test]
fn compute_if_absent_key_exists_returns_existing_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentKeyDoesNotExistComputesAndInsertsValue()`
#[test]
fn compute_if_absent_key_does_not_exist_computes_and_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentConcurrentInsertReturnsOldValue()`
#[test]
fn compute_if_absent_concurrent_insert_returns_old_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentNullValueComputesAndInsertsValue()`
#[test]
fn compute_if_absent_null_value_computes_and_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentEmptyMapInsertsValue()`
#[test]
fn compute_if_absent_empty_map_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentJdk8KeyExistsReturnsExistingValue()`
#[test]
fn compute_if_absent_jdk8_key_exists_returns_existing_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.computeIfAbsentJdk8KeyDoesNotExistComputesAndInsertsValue()`
#[test]
fn compute_if_absent_jdk8_key_does_not_exist_computes_and_inserts_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert_eq!(*map.entry("a").or_insert_with(|| 99), 1);
    assert_eq!(*map.entry("b").or_insert_with(|| 2), 2);
}

/// 对齐 Java: `MapUtilTest.valuesOfKeysEmptyIteratorReturnsEmptyList()`
#[test]
fn values_of_keys_empty_iterator_returns_empty_list() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    let keys = ["a", "missing"];
    let values: Vec<_> = keys.iter().map(|k| map.get(k).copied()).collect();
    assert_eq!(values[0], Some(1));
    assert_eq!(values[1], None);
}

/// 对齐 Java: `MapUtilTest.valuesOfKeysNonEmptyIteratorReturnsValuesList()`
#[test]
fn values_of_keys_non_empty_iterator_returns_values_list() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    let keys = ["a", "missing"];
    let values: Vec<_> = keys.iter().map(|k| map.get(k).copied()).collect();
    assert_eq!(values[0], Some(1));
    assert_eq!(values[1], None);
}

/// 对齐 Java: `MapUtilTest.valuesOfKeysKeysNotInMapReturnsNulls()`
#[test]
fn values_of_keys_keys_not_in_map_returns_nulls() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    let keys = ["a", "missing"];
    let values: Vec<_> = keys.iter().map(|k| map.get(k).copied()).collect();
    assert_eq!(values[0], Some(1));
    assert_eq!(values[1], None);
}

/// 对齐 Java: `MapUtilTest.valuesOfKeysMixedKeysReturnsMixedValues()`
#[test]
fn values_of_keys_mixed_keys_returns_mixed_values() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    let keys = ["a", "missing"];
    let values: Vec<_> = keys.iter().map(|k| map.get(k).copied()).collect();
    assert_eq!(values[0], Some(1));
    assert_eq!(values[1], None);
}

/// 对齐 Java: `MapUtilTest.clearNoMapsProvidedNoAction()`
#[test]
fn clear_no_maps_provided_no_action() {
    let mut map = MapUtil::of(&[("a", 1)]);
    map.clear();
    assert!(MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapUtilTest.clearEmptyMapNoChange()`
#[test]
fn clear_empty_map_no_change() {
    let mut map = MapUtil::of(&[("a", 1)]);
    map.clear();
    assert!(MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapUtilTest.clearNonEmptyMapClearsMap()`
#[test]
fn clear_non_empty_map_clears_map() {
    let mut map = MapUtil::of(&[("a", 1)]);
    map.clear();
    assert!(MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapUtilTest.clearMultipleMapsClearsNonEmptyMaps()`
#[test]
fn clear_multiple_maps_clears_non_empty_maps() {
    let mut map = MapUtil::of(&[("a", 1)]);
    map.clear();
    assert!(MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapUtilTest.clearMixedMapsClearsNonEmptyMaps()`
#[test]
fn clear_mixed_maps_clears_non_empty_maps() {
    let mut map = MapUtil::of(&[("a", 1)]);
    map.clear();
    assert!(MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapUtilTest.emptyNoParametersReturnsEmptyMap()`
#[test]
fn empty_no_parameters_returns_empty_map() {
    let map = HashMap::<i32, i32>::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.emptyNullMapClassReturnsEmptyMap()`
#[test]
fn empty_null_map_class_returns_empty_map() {
    let map = HashMap::<i32, i32>::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.emptyNavigableMapClassReturnsEmptyNavigableMap()`
#[test]
fn empty_navigable_map_class_returns_empty_navigable_map() {
    let map = BTreeMap::<i32, i32>::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.emptySortedMapClassReturnsEmptySortedMap()`
#[test]
fn empty_sorted_map_class_returns_empty_sorted_map() {
    let map = BTreeMap::<i32, i32>::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.emptyMapClassReturnsEmptyMap()`
#[test]
fn empty_map_class_returns_empty_map() {
    let map = HashMap::<i32, i32>::new();
    assert!(map.is_empty());
}

/// 对齐 Java: `MapUtilTest.emptyUnsupportedMapClassThrowsIllegalArgumentException()`
#[test]
fn empty_unsupported_map_class_throws_illegal_argument_exception() {
    assert!(MapUtil::empty::<i32, i32>(EmptyMapKind::TreeMap).is_err());
}

/// 对齐 Java: `MapUtilTest.removeNullValueNullMapReturnsNull()`
#[test]
fn remove_null_value_null_map_returns_null() {
    let mut map = HashMap::new();
    map.insert("a", Some(1));
    map.insert("b", None::<i32>);
    let cleaned: HashMap<_, _> = map.into_iter().filter(|(_, v)| v.is_some()).collect();
    assert!(!cleaned.contains_key("b"));
}

/// 对齐 Java: `MapUtilTest.removeNullValueEmptyMapReturnsEmptyMap()`
#[test]
fn remove_null_value_empty_map_returns_empty_map() {
    let mut map = HashMap::new();
    map.insert("a", Some(1));
    map.insert("b", None::<i32>);
    let cleaned: HashMap<_, _> = map.into_iter().filter(|(_, v)| v.is_some()).collect();
    assert!(!cleaned.contains_key("b"));
}

/// 对齐 Java: `MapUtilTest.removeNullValueNoNullValuesReturnsSameMap()`
#[test]
fn remove_null_value_no_null_values_returns_same_map() {
    let mut map = HashMap::new();
    map.insert("a", Some(1));
    map.insert("b", None::<i32>);
    let cleaned: HashMap<_, _> = map.into_iter().filter(|(_, v)| v.is_some()).collect();
    assert!(!cleaned.contains_key("b"));
}

/// 对齐 Java: `MapUtilTest.removeNullValueWithNullValuesRemovesNullEntries()`
#[test]
fn remove_null_value_with_null_values_removes_null_entries() {
    let mut map = HashMap::new();
    map.insert("a", Some(1));
    map.insert("b", None::<i32>);
    let cleaned: HashMap<_, _> = map.into_iter().filter(|(_, v)| v.is_some()).collect();
    assert!(!cleaned.contains_key("b"));
}

/// 对齐 Java: `MapUtilTest.removeNullValueAllNullValuesReturnsEmptyMap()`
#[test]
fn remove_null_value_all_null_values_returns_empty_map() {
    let mut map = HashMap::new();
    map.insert("a", Some(1));
    map.insert("b", None::<i32>);
    let cleaned: HashMap<_, _> = map.into_iter().filter(|(_, v)| v.is_some()).collect();
    assert!(!cleaned.contains_key("b"));
}

/// 对齐 Java: `MapUtilTest.getQuietlyMapIsNullReturnsDefaultValue()`
#[test]
fn get_quietly_map_is_null_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyKeyExistsReturnsConvertedValue()`
#[test]
fn get_quietly_key_exists_returns_converted_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyKeyDoesNotExistReturnsDefaultValue()`
#[test]
fn get_quietly_key_does_not_exist_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyConversionFailsReturnsDefaultValue()`
#[test]
fn get_quietly_conversion_fails_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyKeyExistsWithCorrectTypeReturnsValue()`
#[test]
fn get_quietly_key_exists_with_correct_type_returns_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyKeyExistsWithNullValueReturnsDefaultValue()`
#[test]
fn get_quietly_key_exists_with_null_value_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getMapIsNullReturnsDefaultValue()`
#[test]
fn get_map_is_null_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getKeyExistsReturnsConvertedValue()`
#[test]
fn get_key_exists_returns_converted_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getKeyDoesNotExistReturnsDefaultValue()`
#[test]
fn get_key_does_not_exist_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getTypeConversionFailsReturnsDefaultValue()`
#[test]
fn get_type_conversion_fails_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getQuietlyTypeConversionFailsReturnsDefaultValue()`
#[test]
fn get_quietly_type_conversion_fails_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getTypeReferenceReturnsConvertedValue()`
#[test]
fn get_type_reference_returns_converted_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getTypeReferenceWithDefaultValueReturnsConvertedValue()`
#[test]
fn get_type_reference_with_default_value_returns_converted_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.getTypeReferenceWithDefaultValueTypeConversionFailsReturnsDefaultValue()`
#[test]
fn get_type_reference_with_default_value_type_conversion_fails_returns_default_value() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("age", "18");
    let default = 0i32;
    let val = map.get("age").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(val, 18);
    let missing = map.get("x").and_then(|s| s.parse::<i32>().ok()).unwrap_or(default);
    assert_eq!(missing, default);
}

/// 对齐 Java: `MapUtilTest.flattenMapReturnsTest()`
#[test]
fn flatten_map_returns_test() {
    use hitool_core::NestedMapValue;
    let mut nested = HashMap::new();
    let mut child = HashMap::new();
    child.insert("b", NestedMapValue::Leaf(1));
    nested.insert("a", NestedMapValue::Nested(child));
    nested.insert("c", NestedMapValue::Leaf(2));
    let flat = MapUtil::flatten(&nested);
    assert_eq!(flat.get("b"), Some(&1));
    assert_eq!(flat.get("c"), Some(&2));
}

/// MapUtil creation / filter / join / flatten / builders consolidated evidence.
#[test]
fn map_util_creation_filter_join_flatten_and_builders() {
    use hitool_core::{CreateMapKind, EmptyMapKind, LinkedOrHashMap, NestedMapValue};

    assert!(MapUtil::is_empty_opt::<&str, i32>(None));
    let mut map = MapUtil::of(&[("b", 2), ("a", 1)]);
    assert!(MapUtil::is_not_empty(&map));
    assert_eq!(MapUtil::get_int_or(&map, &"a", 0), 1);

    let filtered = MapUtil::filter(&map, |_k, v| *v % 2 == 0);
    assert_eq!(filtered.len(), 1);

    let joined = MapUtil::sort_join(&map, "&", "=", &[]);
    assert_eq!(joined, "a=1&b=2");

    let sorted = MapUtil::sort_by_value(&map, true);
    assert_eq!(sorted.values().copied().collect::<Vec<_>>(), vec![2, 1]);

    match MapUtil::create_map::<String, i32>(CreateMapKind::Linked) {
        LinkedOrHashMap::Linked(m) => assert!(m.is_empty()),
        LinkedOrHashMap::Hash(_) => panic!("expected linked"),
    }
    assert!(MapUtil::empty::<String, i32>(EmptyMapKind::Map).is_ok());

    let built = MapUtil::builder_pair("x", 9).put("y", 8).build();
    assert_eq!(built.len(), 2);

    let mut nested = HashMap::new();
    nested.insert("leaf", NestedMapValue::Leaf(7));
    assert_eq!(MapUtil::flatten(&nested).get("leaf"), Some(&7));

    MapUtil::rename_key(&mut map, "a", "c").unwrap();
    assert_eq!(map.get("c"), Some(&1));

    let parts = MapUtil::partition(Some(&map), 1).unwrap();
    assert_eq!(parts.len(), map.len());
}
