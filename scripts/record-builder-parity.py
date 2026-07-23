#!/usr/bin/env python3
"""Record reviewed Hutool core.builder APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.builder::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "Builder": (
        "hutool_core::Builder",
        "generic_builder_supports_every_supplier_and_modifier_shape",
        "Rust's typed Builder contract returns the current state without Java serialization machinery.",
    ),
    "GenericBuilder": (
        "hutool_core::GenericBuilder",
        "generic_builder_supports_every_supplier_and_modifier_shape",
        "Boxed Rust suppliers and modifiers cover zero through five constructor arguments, one-shot mutation, and reuse.",
    ),
    "CompareToBuilder": (
        "hutool_core::CompareToBuilder",
        "compare_builder_short_circuits_options_slices_custom_and_float_edges",
        "Ord, explicit comparators, slices, options, Java floating-point bit rules, and Serde structural traversal provide real lexicographic comparison.",
    ),
    "EqualsBuilder": (
        "hutool_core::EqualsBuilder",
        "equals_builder_matches_java_float_bits_short_circuit_and_reset",
        "PartialEq, Java floating-point bit rules, reset, short-circuiting, and Serde structural traversal provide real equality behavior.",
    ),
    "HashCodeBuilder": (
        "hutool_core::HashCodeBuilder",
        "hash_builder_matches_hutool_primitive_array_and_structural_rules",
        "Wrapping i32 arithmetic preserves Hutool's 17/37 hash formulas across primitives, UTF-16 strings, slices, superclass values, and Serde structures.",
    ),
    "IDKey": (
        "hutool_core::IdKey",
        "identity_key_uses_reference_identity_and_standard_hashing",
        "A lifetime-bound reference address supplies identity equality and hashing without unsafe code or a global registry.",
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
        builder_family = family(row["qualified_name"])
        if builder_family is None:
            continue
        selected += 1
        symbol, test, notes = FAMILIES[builder_family]
        if "reflection" in row["qualified_name"]:
            test = "structural_compare_equals_and_exclusions_are_real_serde_walks"
            notes += " Java reflection overloads map to explicit, fallible Serde structure inspection with field exclusions."
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/builder.rs::{test}",
            "notes": notes,
        }

    if selected != 96:
        raise SystemExit(f"expected 96 reviewed builder APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool builder APIs")


if __name__ == "__main__":
    main()
