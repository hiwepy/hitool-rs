#!/usr/bin/env python3
"""Record reviewed hutool-all APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-all":
            continue
        selected += 1
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": "hutool::Hutool",
            "test_evidence": "crates/hutool/src/lib.rs::hutool_registry_is_feature_aware_stable_and_writable",
            "notes": "Cargo features replace Java classpath reflection with a deterministic, "
            "lexically ordered capability registry and injectable writer output.",
        }

    if selected != 3:
        raise SystemExit(f"expected 3 reviewed hutool-all APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed hutool-all APIs")


if __name__ == "__main__":
    main()
