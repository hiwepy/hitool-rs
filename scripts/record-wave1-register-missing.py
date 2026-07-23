#!/usr/bin/env python3
"""Wave-1: register every missing Hutool API (merge-only).

Unportable APIs → planned / unsafe-to-copy with classifier notes.
Portable APIs without an implementation yet → planned (Wave-2 lifts them).

Does not delete or overwrite existing decisions.
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

# Tags that should never be claimed idiomatic in Wave-1 bulk register.
UNPORTABLE = {
    "jvm_only",
    "reflection",
    "awt_swing",
    "javax_servlet",
    "javax_sql_spi",
    "jndi",
    "bouncycastle_only",
    "soap_server",
}


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    existing: dict[str, dict[str, str]] = {}
    if DECISIONS.exists():
        with DECISIONS.open(encoding="utf-8", newline="") as stream:
            for row in csv.DictReader(stream):
                existing[row["api_id"]] = row

    added = 0
    by_tag: dict[str, int] = {}
    for row in inventory:
        api_id = row["api_id"]
        if api_id in existing:
            continue
        tag, reason = classify_row(row)
        by_tag[tag] = by_tag.get(tag, 0) + 1
        if tag == "reflection":
            status = "unsafe-to-copy"
            notes = f"[{tag}] {reason}; use Serde/From/TryFrom or explicit typed APIs"
        else:
            status = "planned"
            notes = f"[{tag}] {reason}; wave1 register — portable items lifted in wave2+"
        existing[api_id] = {
            "api_id": api_id,
            "status": status,
            "hitool_symbol": "",
            "test_evidence": "",
            "notes": notes,
        }
        added += 1

    # Preserve stable-ish order: previous rows first, then new by api_id.
    ordered = list(existing.values())
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(ordered)

    print(f"wave1 registered {added} missing APIs (total decisions={len(existing)})")
    for tag, n in sorted(by_tag.items(), key=lambda x: -x[1]):
        print(f"  {tag}: {n}")


if __name__ == "__main__":
    main()
