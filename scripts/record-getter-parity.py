#!/usr/bin/env python3
"""Record reviewed Hutool getter APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.getter::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "ArrayTypeGetter": "hitool_core::ArrayTypeGetter",
    "BasicTypeGetter": "hitool_core::BasicTypeGetter",
    "GroupedTypeGetter": "hitool_core::GroupedTypeGetter",
    "ListTypeGetter": "hitool_core::ListTypeGetter",
    "OptArrayTypeGetter": "hitool_core::OptArrayTypeGetter",
    "OptBasicTypeGetter": "hitool_core::OptBasicTypeGetter",
    "OptNullBasicTypeFromObjectGetter": "hitool_core::OptNullBasicTypeFromObjectGetter",
    "OptNullBasicTypeFromStringGetter": "hitool_core::OptNullBasicTypeFromStringGetter",
    "OptNullBasicTypeGetter": "hitool_core::OptNullBasicTypeGetter",
}
EVIDENCE = (
    "crates/hitool-core/src/getter.rs::"
    "getter_traits_cover_scalars_defaults_arrays_lists_groups_and_objects"
)


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
        getter_family = family(row["qualified_name"])
        if getter_family is None:
            continue
        selected += 1
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": FAMILIES[getter_family],
            "test_evidence": EVIDENCE,
            "notes": (
                "Java typed getter overloads are consolidated into FromStr/Any "
                "Rust traits with Option and caller-provided default semantics."
            ),
        }

    if selected != 9:
        raise SystemExit(f"expected 9 reviewed getter APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool getter APIs")


if __name__ == "__main__":
    main()
