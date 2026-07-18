//! coll_stream_util module parity tests
//! 对齐: hutool-core CollStreamUtilTest

use hitool_core::CollStreamUtil;

// ── to_identity_map ──

#[test]
fn to_identity_map_basic() {
    let items = vec!["apple", "banana", "cherry"];
    let map = CollStreamUtil::to_identity_map(items, |s: &&str| s.to_string());
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("apple"), Some(&"apple"));
}

// ── to_map ──

#[test]
fn to_map_basic() {
    let items = vec![("a", 1), ("b", 2)];
    let map = CollStreamUtil::to_map(items, |(k, _): &(&str, i32)| k.to_string(), |(_, v): &(&str, i32)| *v);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
}

// ── group_by_key ──

#[test]
fn group_by_key_basic() {
    let items = vec!["apple", "avocado", "banana", "blueberry"];
    let groups = CollStreamUtil::group_by_key(items, |s: &&str| s.chars().next().unwrap());
    assert_eq!(groups.get(&'a').unwrap().len(), 2);
    assert_eq!(groups.get(&'b').unwrap().len(), 2);
}

// ── filter_map_to_list ──

#[test]
fn filter_map_to_list_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let result = CollStreamUtil::filter_map_to_list(items, |x: i32| {
        if x % 2 == 0 { Some(x * 10) } else { None }
    });
    assert_eq!(result, vec![20, 40]);
}

// ── filter_map_to_set ──

#[test]
fn filter_map_to_set_basic() {
    let items = vec![1, 2, 2, 3, 3];
    let result = CollStreamUtil::filter_map_to_set(items, |x: i32| {
        if x > 1 { Some(x * 10) } else { None }
    });
    assert_eq!(result.len(), 2);
    assert!(result.contains(&20));
    assert!(result.contains(&30));
}
