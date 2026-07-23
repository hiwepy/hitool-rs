#!/usr/bin/env python3
"""Record reviewed Hutool compression APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.compress::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "Deflate": ("hutool_core::Deflate", "deflate_and_gzip_round_trip_both_wrapper_modes"),
    "Gzip": ("hutool_core::Gzip", "deflate_and_gzip_round_trip_both_wrapper_modes"),
    "ZipCopyVisitor": ("hutool_core::ZipCopyVisitor", "zip_copy_visitor_copies_relative_tree"),
    "ZipReader": ("hutool_core::ZipReader", "zip_reader_gets_visits_filters_extracts_and_limits"),
    "ZipWriter": ("hutool_core::ZipWriter", "zip_writer_adds_streams_directories_paths_and_comments"),
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
        compress_family = family(row["qualified_name"])
        if compress_family is None:
            continue
        selected += 1
        symbol, test = FAMILIES[compress_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/compress.rs::{test}",
            "notes": (
                "Hutool compression streams and ZIP visitors map to Rust Read/Write APIs "
                "with deterministic archives and mandatory ZIP Slip, symlink, size, and ratio guards."
            ),
        }

    if selected != 45:
        raise SystemExit(f"expected 45 reviewed compression APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool compression APIs")


if __name__ == "__main__":
    main()
