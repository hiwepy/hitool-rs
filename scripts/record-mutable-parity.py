#!/usr/bin/env python3
"""Record reviewed Hutool core.lang.mutable APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.lang.mutable::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "Mutable": (
        "hutool_core::Mutable",
        "boolean_and_object_mutables_are_owned_standard_value_wrappers",
        "An ownership-aware Rust trait provides shared borrow, mutable borrow, and replacement semantics.",
    ),
    "MutableBool": (
        "hutool_core::MutableBool",
        "boolean_and_object_mutables_are_owned_standard_value_wrappers",
        "A typed bool wrapper preserves Hutool parsing, comparison, display, mutation, and Java hash behavior.",
    ),
    "MutableByte": (
        "hutool_core::MutableByte",
        "integer_mutables_cover_wrapping_parsing_conversion_and_hashing",
        "An i8-backed wrapper uses standard parsing and explicit Java-compatible wrapping arithmetic, conversions, ordering, display, and hashing.",
    ),
    "MutableShort": (
        "hutool_core::MutableShort",
        "integer_mutables_cover_wrapping_parsing_conversion_and_hashing",
        "An i16-backed wrapper uses standard parsing and explicit Java-compatible wrapping arithmetic, conversions, ordering, display, and hashing.",
    ),
    "MutableInt": (
        "hutool_core::MutableInt",
        "integer_mutables_cover_wrapping_parsing_conversion_and_hashing",
        "An i32-backed wrapper uses standard parsing and explicit Java-compatible wrapping arithmetic, conversions, ordering, display, and hashing.",
    ),
    "MutableLong": (
        "hutool_core::MutableLong",
        "integer_mutables_cover_wrapping_parsing_conversion_and_hashing",
        "An i64-backed wrapper uses standard parsing and explicit Java-compatible wrapping arithmetic, conversions, ordering, display, and folded Java hashing.",
    ),
    "MutableFloat": (
        "hutool_core::MutableFloat",
        "floating_mutables_use_java_nan_signed_zero_comparison_and_hashing",
        "An f32-backed wrapper uses standard parsing and canonical Java NaN, signed-zero, comparison, conversion, display, and hash semantics.",
    ),
    "MutableDouble": (
        "hutool_core::MutableDouble",
        "floating_mutables_use_java_nan_signed_zero_comparison_and_hashing",
        "An f64-backed wrapper uses standard parsing and canonical Java NaN, signed-zero, comparison, conversion, display, and hash semantics.",
    ),
    "MutableObj": (
        "hutool_core::MutableObj",
        "boolean_and_object_mutables_are_owned_standard_value_wrappers",
        "A generic owned Rust value wrapper supplies factory, borrow, replacement, equality, ordering, hashing, display, and extraction behavior.",
    ),
    "MutablePair": (
        "hutool_core::MutablePair",
        "mutable_pair_supports_individual_and_atomic_replacement",
        "An owned tuple wrapper supports key, value, and atomic pair replacement while preserving standard Rust equality, ordering, and hashing.",
    ),
}


def family(qualified_name: str) -> str | None:
    if not qualified_name.startswith(ROOT):
        return None
    candidate = qualified_name[len(ROOT) :].split("::", 1)[0].split("#", 1)[0]
    return candidate if candidate in FAMILIES else None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        mutable_family = family(row["qualified_name"])
        if mutable_family is None:
            continue
        selected += 1
        symbol, test, notes = FAMILIES[mutable_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/mutable.rs::{test}",
            "notes": notes,
        }

    if selected != 161:
        raise SystemExit(f"expected 161 reviewed mutable APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool mutable APIs")


if __name__ == "__main__":
    main()
