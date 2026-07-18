use hitool_setting as hs;
use std::time::Duration;

#[test]
fn grouped_map_put_get_test() {
    let mut map = hs::GroupedMap::new();
    map.put("group", "key1", "value1");
    assert_eq!(map.get("group", "key1"), Some("value1"));
    assert!(map.get("group", "nonexistent").is_none());
}

#[test]
fn grouped_map_groups_test() {
    let mut map = hs::GroupedMap::new();
    map.put("db", "host", "localhost");
    map.put("app", "name", "test");
    let groups: Vec<&str> = map.groups().collect();
    assert!(groups.contains(&"db"));
    assert!(groups.contains(&"app"));
}

#[test]
fn grouped_map_remove_test() {
    let mut map = hs::GroupedMap::new();
    map.put("g", "k", "v");
    assert_eq!(map.remove("g", "k"), Some("v".to_string()));
    assert!(map.get("g", "k").is_none());
}

#[test]
fn grouped_map_contains_test() {
    let mut map = hs::GroupedMap::new();
    map.put("db", "host", "localhost");
    assert!(map.contains_key("db", "host"));
    assert!(map.contains_value("db", "localhost"));
    assert!(!map.contains_key("db", "missing"));
}

#[test]
fn grouped_map_size_test() {
    let mut map = hs::GroupedMap::new();
    map.put("g", "k1", "v1");
    map.put("g", "k2", "v2");
    assert_eq!(map.size(), 2);
}

#[test]
fn setting_loader_test() {
    let loader = hs::SettingsLoader::new();
    // 验证 builder 模式
    let _ = loader.required_file("/tmp/nonexistent.toml");
}
