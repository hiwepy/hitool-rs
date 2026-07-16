#!/usr/bin/env python3
"""Record reviewed Hutool collection type APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.collection::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "CollStreamUtil": (
        "hitool_core::CollStreamUtil",
        "coll_stream_util_matches_hutool_grouping_mapping_and_merge",
    ),
    "ComputeIter": (
        "hitool_core::ComputeIter",
        "compute_iter_caches_finishes_and_resets_state",
    ),
    "LineIter": (
        "hitool_core::LineIter",
        "line_iter_filters_closes_and_propagates_utf8_errors",
    ),
    "NodeListIter": (
        "hitool_core::NodeListIter",
        "node_transforming_and_spliterator_views_preserve_source_semantics",
    ),
    "TransCollection": (
        "hitool_core::TransCollection",
        "node_transforming_and_spliterator_views_preserve_source_semantics",
    ),
    "TransSpliterator": (
        "hitool_core::TransSpliterator",
        "node_transforming_and_spliterator_views_preserve_source_semantics",
    ),
    "SpliteratorUtil": (
        "hitool_core::SpliteratorUtil",
        "node_transforming_and_spliterator_views_preserve_source_semantics",
    ),
    "CollectionUtil": (
        "hitool_core::CollectionUtil",
        "node_transforming_and_spliterator_views_preserve_source_semantics",
    ),
    "IterUtil": (
        "hitool_core::IterUtil",
        "iter_util_matches_hutool_consumption_mapping_and_formatting",
    ),
    "ListUtil": (
        "hitool_core::ListUtil",
        "list_util_matches_hutool_creation_paging_mutation_and_search",
    ),
    "ArrayIter": (
        "hitool_core::ArrayIter",
        "array_iter_normalizes_bounds_and_resets",
    ),
    "CopiedIter": (
        "hitool_core::CopiedIter",
        "copied_iter_is_an_owned_read_only_snapshot",
    ),
    "FilterIter": (
        "hitool_core::FilterIter",
        "filter_iter_supports_lookahead_and_optional_filtering",
    ),
    "TransIter": (
        "hitool_core::TransIter",
        "trans_iter_maps_lazily_with_lookahead",
    ),
    "IterChain": (
        "hitool_core::IterChain",
        "iter_chain_skips_empty_sources_and_accepts_late_additions",
    ),
    "IterableIter": (
        "hitool_core::IterableIter",
        "rust_iterators_cover_iterable_and_enumeration_adapters",
    ),
    "ResettableIter": (
        "hitool_core::ResettableIter",
        "array_iter_normalizes_bounds_and_resets",
    ),
    "EnumerationIter": (
        "hitool_core::EnumerationIter",
        "rust_iterators_cover_iterable_and_enumeration_adapters",
    ),
    "IteratorEnumeration": (
        "hitool_core::IteratorEnumeration",
        "rust_iterators_cover_iterable_and_enumeration_adapters",
    ),
    "Partition": (
        "hitool_core::Partition",
        "partition_exposes_borrowed_random_access_chunks",
    ),
    "RandomAccessPartition": (
        "hitool_core::RandomAccessPartition",
        "partition_exposes_borrowed_random_access_chunks",
    ),
    "AvgPartition": (
        "hitool_core::AvgPartition",
        "average_partition_distributes_remainder_and_empty_tails",
    ),
    "RandomAccessAvgPartition": (
        "hitool_core::RandomAccessAvgPartition",
        "average_partition_distributes_remainder_and_empty_tails",
    ),
    "PartitionIter": (
        "hitool_core::PartitionIter",
        "partition_iter_preserves_order_tail_and_lookahead",
    ),
    "RingIndexUtil": (
        "hitool_core::{ring_next_index,ring_next_u64,ring_next_for_len}",
        "ring_indices_match_hutool_progression_and_validate_bounds",
    ),
    "BoundedPriorityQueue": (
        "hitool_core::BoundedPriorityQueue",
        "bounded_priority_queue_keeps_best_values_in_sorted_order",
    ),
    "ConcurrentHashSet": (
        "hitool_core::ConcurrentHashSet",
        "concurrent_hash_set_supports_shared_atomic_updates",
    ),
    "UniqueKeySet": (
        "hitool_core::UniqueKeySet",
        "unique_key_set_replaces_or_preserves_duplicates_as_requested",
    ),
}

SOURCE_FILES = {
    "CollStreamUtil": "coll_stream_util.rs",
    "IterUtil": "iter_util.rs",
    "ListUtil": "list_util.rs",
    "ArrayIter": "collection_iter.rs",
    "CopiedIter": "collection_iter.rs",
    "FilterIter": "collection_iter.rs",
    "TransIter": "collection_iter.rs",
    "IterChain": "collection_iter.rs",
    "IterableIter": "collection_iter.rs",
    "ResettableIter": "collection_iter.rs",
    "EnumerationIter": "collection_iter.rs",
    "IteratorEnumeration": "collection_iter.rs",
    "Partition": "collection_partition.rs",
    "RandomAccessPartition": "collection_partition.rs",
    "AvgPartition": "collection_partition.rs",
    "RandomAccessAvgPartition": "collection_partition.rs",
    "PartitionIter": "collection_partition.rs",
    "ComputeIter": "collection_adapters.rs",
    "LineIter": "collection_adapters.rs",
    "NodeListIter": "collection_adapters.rs",
    "TransCollection": "collection_adapters.rs",
    "TransSpliterator": "collection_adapters.rs",
    "SpliteratorUtil": "collection_adapters.rs",
    "CollectionUtil": "collection_adapters.rs",
}


def family(qualified_name: str) -> str | None:
    if not qualified_name.startswith(ROOT):
        return None
    candidate = qualified_name[len(ROOT) :].split("::", 1)[0]
    return candidate if candidate in FAMILIES else None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        collection_family = family(row["qualified_name"])
        if collection_family is None:
            continue
        selected += 1
        symbol, test = FAMILIES[collection_family]
        source_file = SOURCE_FILES.get(collection_family, "collection_types.rs")
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": f"crates/hitool-core/src/{source_file}::{test}",
            "notes": "Java collection inheritance is consolidated into an owned Rust API with matching behavioral invariants.",
        }

    if selected != 257:
        raise SystemExit(f"expected 257 reviewed collection APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool collection APIs")


if __name__ == "__main__":
    main()
