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
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": f"crates/hitool-core/src/collection_types.rs::{test}",
            "notes": "Java collection inheritance is consolidated into an owned Rust API with matching behavioral invariants.",
        }

    if selected != 41:
        raise SystemExit(f"expected 41 reviewed collection APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool collection APIs")


if __name__ == "__main__":
    main()
