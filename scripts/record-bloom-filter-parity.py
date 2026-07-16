#!/usr/bin/env python3
"""Record reviewed Hutool bloom-filter APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str] | None:
    bitmap_test = "crates/hitool-bloom-filter/src/tests.rs::checked_sparse_bitmaps_cover_default_32_and_64_bit_layouts"
    named_test = "crates/hitool-bloom-filter/src/tests.rs::function_and_named_filters_share_hutool_add_semantics"
    families = {
        "BitMapBloomFilter": (
            "hitool_bloom_filter::BitMapBloomFilter",
            "crates/hitool-bloom-filter/src/tests.rs::bitmap_composition_uses_or_for_add_and_and_for_contains",
        ),
        "BitSetBloomFilter": (
            "hitool_bloom_filter::BitSetBloomFilter",
            "crates/hitool-bloom-filter/src/tests.rs::bitset_filter_validates_hashes_files_encoding_and_probability",
        ),
        "BloomFilter": (
            "hitool_bloom_filter::{BloomFilter,StringBloomFilter}",
            "crates/hitool-bloom-filter/src/tests.rs::mature_engine_validates_and_tracks_generic_values",
        ),
        "BloomFilterUtil": (
            "hitool_bloom_filter::BloomFilterUtil",
            "crates/hitool-bloom-filter/src/tests.rs::utility_constructors_delegate_to_concrete_filters",
        ),
        "BitMap": ("hitool_bloom_filter::BitMap", bitmap_test),
        "IntMap": ("hitool_bloom_filter::IntMap", bitmap_test),
        "LongMap": ("hitool_bloom_filter::LongMap", bitmap_test),
        "AbstractFilter": ("hitool_bloom_filter::FuncFilter", named_test),
        "FuncFilter": ("hitool_bloom_filter::FuncFilter", named_test),
        "DefaultFilter": ("hitool_bloom_filter::DefaultFilter", named_test),
        "ELFFilter": ("hitool_bloom_filter::ELFFilter", named_test),
        "FNVFilter": ("hitool_bloom_filter::FNVFilter", named_test),
        "HfFilter": ("hitool_bloom_filter::HfFilter", named_test),
        "HfIpFilter": ("hitool_bloom_filter::HfIpFilter", named_test),
        "JSFilter": ("hitool_bloom_filter::JSFilter", named_test),
        "PJWFilter": ("hitool_bloom_filter::PJWFilter", named_test),
        "RSFilter": ("hitool_bloom_filter::RSFilter", named_test),
        "SDBMFilter": ("hitool_bloom_filter::SDBMFilter", named_test),
        "TianlFilter": ("hitool_bloom_filter::TianlFilter", named_test),
    }
    if not qualified_name.startswith("cn.hutool.bloomfilter"):
        return None
    family = qualified_name.split("::", 1)[1].split("::", 1)[0]
    return families.get(family)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-bloomFilter":
            continue
        target = mapping(row["qualified_name"])
        if target is None:
            raise SystemExit(f"unmapped Hutool bloom-filter family: {row['qualified_name']}")
        selected += 1
        symbol, test = target
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": (
                "The mature generic engine is retained; checked sparse bitmaps and Java UTF-16 "
                "hash strategies preserve Hutool add, contains, composition, and file-init semantics."
            ),
        }

    if selected != 72:
        raise SystemExit(f"expected 72 reviewed bloom-filter APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool bloom-filter APIs")


if __name__ == "__main__":
    main()
