//! random_util parity tests
//! 对齐: hutool-core RandomUtilTest

use hitool_core::RandomUtil;

#[test]
fn random_int_basic() {
    let val = RandomUtil::random_int();
    assert!(val >= i32::MIN && val <= i32::MAX);
}

#[test]
fn random_int_max() {
    for _ in 0..100 {
        let val = RandomUtil::random_int_max(10);
        assert!(val >= 0 && val < 10);
    }
}

#[test]
fn random_int_range() {
    for _ in 0..100 {
        let val = RandomUtil::random_int_range(5, 10);
        assert!(val >= 5 && val < 10);
    }
}

#[test]
fn random_long_basic() {
    let _val = RandomUtil::random_long();
}

#[test]
fn random_double_basic() {
    let val = RandomUtil::random_double();
    assert!(val >= 0.0 && val < 1.0);
}

#[test]
fn random_double_range() {
    for _ in 0..100 {
        let val = RandomUtil::random_double_range(1.0, 5.0);
        assert!(val >= 1.0 && val < 5.0);
    }
}

#[test]
fn random_boolean_basic() {
    let _val = RandomUtil::random_boolean();
}

#[test]
fn random_string_length() {
    let s = RandomUtil::random_string(10);
    assert_eq!(s.len(), 10);
    assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn random_string_empty() {
    let s = RandomUtil::random_string(0);
    assert!(s.is_empty());
}

#[test]
fn random_string_from_chars() {
    let s = RandomUtil::random_string_from("abc", 5);
    assert_eq!(s.len(), 5);
    assert!(s.chars().all(|c| "abc".contains(c)));
}

#[test]
fn random_string_from_empty() {
    let s = RandomUtil::random_string_from("", 5);
    assert!(s.is_empty());
}

#[test]
fn random_numbers_length() {
    let s = RandomUtil::random_numbers(8);
    assert_eq!(s.len(), 8);
    assert!(s.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn random_element_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let val = RandomUtil::random_element(&items);
    assert!(val.is_some());
    assert!(items.contains(&val.unwrap()));
}

#[test]
fn random_element_empty() {
    let items: Vec<i32> = vec![];
    assert!(RandomUtil::random_element(&items).is_none());
}

#[test]
fn random_elements_basic() {
    let items = vec![1, 2, 3];
    let result = RandomUtil::random_elements(&items, 5);
    assert_eq!(result.len(), 5);
    assert!(result.iter().all(|x| items.contains(x)));
}

#[test]
fn weighted_random_basic() {
    let items = vec![("a", 1), ("b", 99)];
    // With 99% weight on "b", we should get "b" most of the time
    let mut b_count = 0;
    for _ in 0..1000 {
        if let Some(val) = RandomUtil::weighted_random(&items) {
            if val == "b" {
                b_count += 1;
            }
        }
    }
    assert!(b_count > 900, "Expected mostly 'b', got {}", b_count);
}

#[test]
fn weighted_random_empty() {
    let items: Vec<(&str, u32)> = vec![];
    assert!(RandomUtil::weighted_random(&items).is_none());
}
