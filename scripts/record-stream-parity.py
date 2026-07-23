#!/usr/bin/env python3
"""Record reviewed Hutool stream APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.stream::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "CollectorUtil": (
        "hutool_core::CollectorUtil",
        "collector_util_joins_groups_maps_and_reduces",
    ),
    "SimpleCollector": (
        "hutool_core::SimpleCollector",
        "simple_collector_exposes_and_executes_every_stage",
    ),
    "StreamUtil": (
        "hutool_core::StreamUtil",
        "stream_util_builds_reads_and_joins_iterators",
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
        stream_family = family(row["qualified_name"])
        if stream_family is None:
            continue
        selected += 1
        symbol, test = FAMILIES[stream_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/stream.rs::{test}",
            "notes": (
                "Java Stream and Collector behavior maps to lazy Rust iterators, "
                "explicit collector stages, stable grouping, joining, and bounded text loading."
            ),
        }

    if selected != 36:
        raise SystemExit(f"expected 36 reviewed stream APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool stream APIs")


if __name__ == "__main__":
    main()
