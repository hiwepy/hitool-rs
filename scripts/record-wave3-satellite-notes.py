#!/usr/bin/env python3
"""Wave-3: refresh planned notes for satellite modules (merge-only, no status flip to idiomatic).

Ensures hutool-extra / hutool-db / hutool-http / hutool-crypto planned rows carry
honest unportable-matrix tags. Does not claim covered for Java-glue surfaces.
"""

from __future__ import annotations

import csv
import importlib.util
from pathlib import Path

_spec = importlib.util.spec_from_file_location(
    "classify_unportable",
    Path(__file__).resolve().parent / "classify-unportable.py",
)
_mod = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(_mod)
classify_row = _mod.classify_row

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
MODULES = {"hutool-extra", "hutool-db", "hutool-http", "hutool-crypto"}


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inv = {r["api_id"]: r for r in csv.DictReader(stream)}
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        decisions = list(csv.DictReader(stream))

    updated = 0
    for row in decisions:
        inv_row = inv.get(row["api_id"])
        if not inv_row or inv_row["module"] not in MODULES:
            continue
        if row["status"] not in {"planned", "unsafe-to-copy"}:
            continue
        tag, reason = classify_row(inv_row)
        prefix = f"[{tag}] {reason}"
        notes = (row.get("notes") or "").strip()
        if notes.startswith(f"[{tag}]"):
            continue
        # Keep prior detail after refreshed tag.
        row["notes"] = f"{prefix}; {notes}" if notes else prefix
        updated += 1

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(decisions)
    print(f"wave3 refreshed planned notes on {updated} satellite rows")


if __name__ == "__main__":
    main()
