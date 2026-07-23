//! Collection utilities that preserve input order.
//!
//! The initial behavior was informed by yimi-rutool 0.2.5 (Apache-2.0),
//! then revised to reject invalid partition sizes and avoid partial ordering.
//!
//! Hutool 包路径对齐 facade 见 [`coll_util`] / [`list_util`]（委托到 crate 根
//! idiomatic [`crate::CollUtil`] / [`crate::ListUtil`]）。

pub mod coll_util;
pub mod list_util;

pub use coll_util::CollUtil;
pub use list_util::ListUtil;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::{CoreError, Result};

/// Returns stable, first-seen distinct elements.
#[must_use]
pub fn distinct<T>(items: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut seen = HashSet::with_capacity(items.len());
    items
        .iter()
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

/// Groups cloned elements by a derived key while preserving per-group order.
#[must_use]
pub fn group_by<T, K>(items: &[T], mut key: impl FnMut(&T) -> K) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
{
    let mut groups = HashMap::new();
    for item in items {
        groups
            .entry(key(item))
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    groups
}

/// Partitions a slice into owned chunks of `size`.
///
/// # Errors
///
/// Returns [`CoreError::InvalidArgument`] when `size` is zero.
pub fn partition<T: Clone>(items: &[T], size: usize) -> Result<Vec<Vec<T>>> {
    if size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "size",
            reason: "must be greater than zero",
        });
    }
    Ok(items.chunks(size).map(<[T]>::to_vec).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_is_stable() {
        assert_eq!(distinct(&[3, 1, 3, 2, 1]), [3, 1, 2]);
    }

    #[test]
    fn partition_rejects_zero_and_keeps_tail() {
        assert!(partition(&[1, 2], 0).is_err());
        assert_eq!(
            partition(&[1, 2, 3, 4, 5], 2).unwrap(),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
    }

    #[test]
    fn grouping_preserves_values() {
        let groups = group_by(&[1, 2, 3, 4], |value| value % 2);
        assert_eq!(groups.get(&0), Some(&vec![2, 4]));
        assert_eq!(groups.get(&1), Some(&vec![1, 3]));
    }
}

