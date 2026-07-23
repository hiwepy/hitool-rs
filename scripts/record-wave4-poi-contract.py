#!/usr/bin/env python3
"""Wave-4: confirm hutool-poi contract — all APIs planned stubs until easy* engines.

Re-asserts every hutool-poi decision as planned with engine-deferral notes.
Does not mark any poi API idiomatic.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
NOTE = (
    "[jvm_only] poi deferred until easyexcel-rs / easydoc-rs / easyofd-rs / "
    "easypdf-rs; hutool-poi keeps Hutool-aligned signatures with empty PendingEngine stubs"
)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        poi_ids = [r["api_id"] for r in csv.DictReader(stream) if r["module"] == "hutool-poi"]
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        decisions = {r["api_id"]: r for r in csv.DictReader(stream)}

    fixed = 0
    for api_id in poi_ids:
        row = decisions.get(api_id)
        if row is None:
            decisions[api_id] = {
                "api_id": api_id,
                "status": "planned",
                "hutool_symbol": "hutool_poi::PendingEngine",
                "test_evidence": "",
                "notes": NOTE,
            }
            fixed += 1
            continue
        if row["status"] != "planned" or "easyexcel-rs" not in (row.get("notes") or ""):
            row["status"] = "planned"
            row["hutool_symbol"] = row.get("hutool_symbol") or "hutool_poi::PendingEngine"
            row["test_evidence"] = ""
            row["notes"] = NOTE
            fixed += 1

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(decisions.values())
    print(f"wave4 poi contract: {len(poi_ids)} APIs planned; updated={fixed}")


if __name__ == "__main__":
    main()
