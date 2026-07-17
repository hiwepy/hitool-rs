#!/usr/bin/env python3
"""Record reviewed Hutool cache APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
COMPAT = "crates/hitool-cache/src/compat.rs::"
FILE = "crates/hitool-cache/src/file_cache.rs::"
NATIVE = "crates/hitool-cache/src/lib.rs::"


def mapping(qualified_name: str) -> tuple[str, str, str]:
    if qualified_name.startswith("cn.hutool.cache.file::"):
        tail = qualified_name.removeprefix("cn.hutool.cache.file::")
        family = tail.split("::", 1)[0]
        if family in {"LFUFileCache", "LRUFileCache"}:
            test = "lru_and_lfu_evict_by_byte_capacity"
        else:
            test = "file_cache_reuses_bytes_reports_limits_and_clears"
        return (
            f"hitool_cache::{family}",
            FILE + test,
            "Path-based Rust file caching preserves Hutool byte limits, LRU/LFU eviction, "
            "expiration, cache statistics, clear, and I/O failure behavior with Arc byte slices.",
        )

    if qualified_name.startswith("cn.hutool.cache.impl::"):
        tail = qualified_name.removeprefix("cn.hutool.cache.impl::")
        family = tail.split("::", 1)[0]
        if family in {"FIFOCache", "LFUCache", "LRUCache"}:
            test = "fifo_lru_and_lfu_apply_deterministic_eviction"
        elif family == "TimedCache":
            test = "timed_cache_schedules_replaces_and_cancels_workers"
        elif family == "WeakCache":
            test = "weak_cache_observes_arc_lifetime_timeout_listener_and_prune"
        elif family == "NoCache":
            test = "no_cache_and_all_cache_util_constructors_are_usable"
        else:
            test = "abstract_cache_covers_expiration_refresh_counters_and_views"
        return (
            f"hitool_cache::{family}",
            COMPAT + test,
            "The thread-safe Rust compatibility engine preserves Hutool cache lifecycle, "
            "statistics, iteration snapshots, listeners, expiration, and deterministic policy behavior.",
        )

    if not qualified_name.startswith("cn.hutool.cache::"):
        raise ValueError(f"unexpected Hutool cache API: {qualified_name}")
    tail = qualified_name.removeprefix("cn.hutool.cache::")
    family = tail.split("::", 1)[0]
    if family == "CacheUtil":
        test = "no_cache_and_all_cache_util_constructors_are_usable"
        prefix = COMPAT
    elif family == "GlobalPruneTimer":
        test = "explicit_prune_handle_runs_and_stops"
        prefix = COMPAT
    elif family == "Cache":
        test = "mature_cache_covers_clone_clear_count_ttl_tti_and_capacity"
        prefix = NATIVE
    else:
        test = "factories_listeners_replacement_clear_and_unlimited_capacity_work"
        prefix = COMPAT
    return (
        f"hitool_cache::{family}",
        prefix + test,
        "Hutool's cache contract maps to explicit Rust ownership, Arc-backed values, typed "
        "durations, per-cache workers, and mature Moka concurrency without hidden global state.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-cache":
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }

    if selected != 124:
        raise SystemExit(f"expected 124 reviewed cache APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool cache APIs")


if __name__ == "__main__":
    main()
