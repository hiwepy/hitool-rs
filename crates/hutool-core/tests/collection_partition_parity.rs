//! collection_partition module parity tests
//! 对齐: hutool-core PartitionIter/Partition tests

use hutool_core::{AvgPartition, Partition, PartitionIter};

// ── Partition ──

#[test]
fn partition_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let p = Partition::new(&items, 2).unwrap();
    assert_eq!(p.len(), 3);
    assert!(!p.is_empty());
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
    assert_eq!(p.get(2), Some(&[5][..]));
}

#[test]
fn partition_exact_division() {
    let items = vec![1, 2, 3, 4];
    let p = Partition::new(&items, 2).unwrap();
    assert_eq!(p.len(), 2);
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
}

#[test]
fn partition_empty() {
    let items: Vec<i32> = vec![];
    let p = Partition::new(&items, 3).unwrap();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);
}

#[test]
fn partition_iter() {
    let items = vec![1, 2, 3, 4, 5];
    let p = Partition::new(&items, 2).unwrap();
    let collected: Vec<&[i32]> = p.iter().collect();
    assert_eq!(collected, vec![&[1, 2][..], &[3, 4][..], &[5][..]]);
}

// ── AvgPartition ──

#[test]
fn avg_partition_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let p = AvgPartition::new(&items, 3).unwrap();
    // 5 items into 3 partitions: sizes 2, 2, 1
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
    assert_eq!(p.get(2), Some(&[5][..]));
}

#[test]
fn avg_partition_iter() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let p = AvgPartition::new(&items, 3).unwrap();
    let collected: Vec<&[i32]> = p.iter().collect();
    assert_eq!(collected.len(), 3);
}

// ── PartitionIter ──

#[test]
fn partition_iter_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let mut iter = PartitionIter::new(items.into_iter(), 2).unwrap();
    assert!(iter.has_next());
    assert_eq!(iter.next(), Some(vec![1, 2]));
    assert!(iter.has_next());
    assert_eq!(iter.next(), Some(vec![3, 4]));
    assert!(iter.has_next());
    assert_eq!(iter.next(), Some(vec![5]));
    assert!(!iter.has_next());
}

#[test]
fn partition_iter_empty() {
    let items: Vec<i32> = vec![];
    let mut iter = PartitionIter::new(items.into_iter(), 3).unwrap();
    assert!(!iter.has_next());
}
