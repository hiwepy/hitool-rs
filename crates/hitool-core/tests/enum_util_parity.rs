//! enum_util parity tests
//! 对齐: hutool-core EnumUtilTest

use hitool_core::EnumUtil;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

// ── 枚举名称 ──

#[test]
fn name_of_color() {
    assert_eq!(EnumUtil::name_of(&Color::Red), "Red");
    assert_eq!(EnumUtil::name_of(&Color::Green), "Green");
    assert_eq!(EnumUtil::name_of(&Color::Blue), "Blue");
}

#[test]
fn name_of_size() {
    assert_eq!(EnumUtil::name_of(&Size::Small), "Small");
    assert_eq!(EnumUtil::name_of(&Size::Large), "Large");
}

// ── 枚举查找 ──

#[test]
fn from_names_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::from_names(&variants, "Red"), Some(Color::Red));
    assert_eq!(EnumUtil::from_names(&variants, "Green"), Some(Color::Green));
}

#[test]
fn from_names_not_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::from_names(&variants, "Yellow"), None);
}

// ── 枚举列表 ──

#[test]
fn names_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    let names = EnumUtil::names(&variants);
    assert_eq!(names, vec!["Red", "Green", "Blue"]);
}

#[test]
fn names_size() {
    let variants = vec![Size::Small, Size::Medium, Size::Large];
    let names = EnumUtil::names(&variants);
    assert_eq!(names, vec!["Small", "Medium", "Large"]);
}

#[test]
fn count_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::count(&variants), 3);
}

// ── 枚举映射 ──

#[test]
fn name_map_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    let map = EnumUtil::name_map(&variants);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("Red"), Some(&Color::Red));
    assert_eq!(map.get("Green"), Some(&Color::Green));
    assert_eq!(map.get("Blue"), Some(&Color::Blue));
}

// ── 枚举验证 ──

#[test]
fn contains_name_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert!(EnumUtil::contains_name(&variants, "Red"));
    assert!(EnumUtil::contains_name(&variants, "Green"));
    assert!(EnumUtil::contains_name(&variants, "Blue"));
}

#[test]
fn contains_name_not_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert!(!EnumUtil::contains_name(&variants, "Yellow"));
    assert!(!EnumUtil::contains_name(&variants, ""));
}
