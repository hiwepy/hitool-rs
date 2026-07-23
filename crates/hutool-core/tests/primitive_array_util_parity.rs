//! PrimitiveArrayUtil parity tests
//! 对齐: `cn.hutool.core.util.PrimitiveArrayUtil`

use hutool_core::PrimitiveArrayUtil;

#[test]
fn is_empty_and_not_empty() {
    assert!(PrimitiveArrayUtil::is_empty::<i32>(&[]));
    assert!(PrimitiveArrayUtil::is_not_empty(&[1, 2]));
}

#[test]
fn reverse_swap_shuffle_min_max() {
    let mut a = [1, 2, 3, 4];
    PrimitiveArrayUtil::reverse(&mut a);
    assert_eq!(a, [4, 3, 2, 1]);
    PrimitiveArrayUtil::swap(&mut a, 0, 3);
    assert_eq!(a, [1, 3, 2, 4]);
    assert_eq!(PrimitiveArrayUtil::min(&[3, 1, 2]).unwrap(), 1);
    assert_eq!(PrimitiveArrayUtil::max(&[3, 1, 2]).unwrap(), 3);
    let mut b = [1, 2, 3, 4, 5];
    PrimitiveArrayUtil::shuffle(&mut b);
    assert_eq!(b.len(), 5);
}

#[test]
fn range_add_all_sorted() {
    assert_eq!(PrimitiveArrayUtil::range(3), vec![0, 1, 2]);
    assert_eq!(PrimitiveArrayUtil::range_step(0, 6, 2), vec![0, 2, 4]);
    let merged = PrimitiveArrayUtil::add_all(&[&[1, 2][..], &[3][..]]);
    assert_eq!(merged, vec![1, 2, 3]);
    assert!(PrimitiveArrayUtil::is_sorted_asc(&[1, 2, 3]));
    assert!(PrimitiveArrayUtil::is_sorted_desc(&[3, 2, 1]));
}

#[test]
fn reverse_range_and_contains() {
    let mut a = [1, 2, 3, 4, 5];
    PrimitiveArrayUtil::reverse_range(&mut a, 1, 4);
    assert_eq!(a, [1, 4, 3, 2, 5]);
    assert!(PrimitiveArrayUtil::contains(&[1, 2, 3], &2));
}
