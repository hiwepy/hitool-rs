//! array_util parity tests
//! 对齐: hutool-core ArrayUtilTest

use hitool_core::ArrayUtil;

// ── 空值判断 ──

#[test]
fn is_empty_empty() {
    assert!(ArrayUtil::is_empty(&[] as &[i32]));
}

#[test]
fn is_empty_non_empty() {
    assert!(!ArrayUtil::is_empty(&[1, 2, 3]));
}

#[test]
fn is_not_empty_non_empty() {
    assert!(ArrayUtil::is_not_empty(&[1, 2, 3]));
}

#[test]
fn is_not_empty_empty() {
    assert!(!ArrayUtil::is_not_empty(&[] as &[i32]));
}

// ── 默认值 ──

#[test]
fn default_if_empty_non_empty() {
    let result = ArrayUtil::default_if_empty(&[1, 2, 3], &[4, 5]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn default_if_empty_empty() {
    let result = ArrayUtil::default_if_empty(&[] as &[i32], &[4, 5]);
    assert_eq!(result, vec![4, 5]);
}

// ── 查找操作 ──

#[test]
fn contains_found() {
    assert!(ArrayUtil::contains(&[1, 2, 3], &2));
}

#[test]
fn contains_not_found() {
    assert!(!ArrayUtil::contains(&[1, 2, 3], &5));
}

#[test]
fn index_of_found() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3, 2], &2), Some(1));
}

#[test]
fn index_of_not_found() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3], &5), None);
}

#[test]
fn last_index_of_found() {
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3, 2], &2), Some(3));
}

#[test]
fn last_index_of_not_found() {
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3], &5), None);
}

// ── 转换操作 ──

#[test]
fn to_vec_basic() {
    let result = ArrayUtil::to_vec(&[1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn new_array_basic() {
    let result: Vec<i32> = ArrayUtil::new_array(3);
    assert_eq!(result, vec![0, 0, 0]);
}

// ── 操作 ──

#[test]
fn append_basic() {
    let result = ArrayUtil::append(&[1, 2], &[3, 4]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn insert_basic() {
    let result = ArrayUtil::insert(&[1, 2, 3], 1, &[10, 20]);
    assert_eq!(result, vec![1, 10, 20, 2, 3]);
}

#[test]
fn remove_basic() {
    let result = ArrayUtil::remove(&[1, 2, 3, 4], 2);
    assert_eq!(result, vec![1, 2, 4]);
}

#[test]
fn remove_element_basic() {
    let result = ArrayUtil::remove_element(&[1, 2, 3, 2, 1], &2);
    assert_eq!(result, vec![1, 3, 1]);
}

// ── 排序操作 ──

#[test]
fn sort_basic() {
    let mut arr = vec![3, 1, 4, 1, 5];
    ArrayUtil::sort(&mut arr);
    assert_eq!(arr, vec![1, 1, 3, 4, 5]);
}

#[test]
fn sort_by_basic() {
    let mut arr = vec![3, 1, 4, 1, 5];
    ArrayUtil::sort_by(&mut arr, |a, b| b.cmp(a));
    assert_eq!(arr, vec![5, 4, 3, 1, 1]);
}

#[test]
fn reverse_basic() {
    let mut arr = vec![1, 2, 3, 4, 5];
    ArrayUtil::reverse(&mut arr);
    assert_eq!(arr, vec![5, 4, 3, 2, 1]);
}

// ── 合并操作 ──

#[test]
fn add_all_basic() {
    let result = ArrayUtil::add_all(&[&[1, 2][..], &[3, 4][..], &[5][..]]);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

// ── 截取操作 ──

#[test]
fn sub_basic() {
    let result = ArrayUtil::sub(&[1, 2, 3, 4, 5], 1, 3);
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn sub_out_of_bounds() {
    let result = ArrayUtil::sub(&[1, 2, 3], 0, 10);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn split_basic() {
    let result = ArrayUtil::split(&[1, 2, 3, 4, 5], 2);
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn split_empty() {
    let result: Vec<Vec<i32>> = ArrayUtil::split(&[], 3);
    assert_eq!(result, vec![Vec::<i32>::new()]);
}

// ── 填充操作 ──

#[test]
fn fill_basic() {
    let mut arr = vec![0; 5];
    ArrayUtil::fill(&mut arr, &42);
    assert_eq!(arr, vec![42, 42, 42, 42, 42]);
}

#[test]
fn fill_range_basic() {
    let mut arr = vec![0; 5];
    ArrayUtil::fill_range(&mut arr, 1, 3, &42);
    assert_eq!(arr, vec![0, 42, 42, 0, 0]);
}

// ── 转换操作 ──

#[test]
fn to_string_basic() {
    assert_eq!(ArrayUtil::to_string(&[1, 2, 3]), "[1, 2, 3]");
}

#[test]
fn join_basic() {
    assert_eq!(ArrayUtil::join(&[1, 2, 3], ", "), "1, 2, 3");
}

// ── 比较操作 ──

#[test]
fn equals_equal() {
    assert!(ArrayUtil::equals(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn equals_not_equal() {
    assert!(!ArrayUtil::equals(&[1, 2, 3], &[1, 2, 4]));
}

#[test]
fn is_equals_equal() {
    assert!(ArrayUtil::is_equals(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn is_equals_not_equal() {
    assert!(!ArrayUtil::is_equals(&[1, 2, 3], &[1, 2, 4]));
}
