#!/usr/bin/env python3
"""Record reviewed Hutool clone APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.clone::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "Cloneable": (
        "native",
        "hutool_core::Cloneable",
        "clone_contract_supports_owned_and_shared_rust_values",
    ),
    "DefaultCloneable": (
        "idiomatic",
        "hutool_core::DefaultCloneable",
        "clone_contract_supports_owned_and_shared_rust_values",
    ),
    "CloneSupport": (
        "idiomatic",
        "hutool_core::CloneSupport",
        "clone_contract_supports_owned_and_shared_rust_values",
    ),
    "CloneRuntimeException": (
        "idiomatic",
        "hutool_core::CloneRuntimeException",
        "clone_runtime_exception_preserves_messages_templates_and_sources",
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
        clone_family = family(row["qualified_name"])
        if clone_family is None:
            continue
        selected += 1
        status, symbol, test = FAMILIES[clone_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": status,
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/clone_support.rs::{test}",
            "notes": (
                "Rust's Clone supplies the ownership-safe clone contract; the "
                "wrapper, extension trait, and sourced error preserve Hutool intent."
            ),
        }

    if selected != 10:
        raise SystemExit(f"expected 10 reviewed clone APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool clone APIs")


if __name__ == "__main__":
    main()
