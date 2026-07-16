#!/usr/bin/env python3
"""Verify that every pinned Hutool API has an evidenced HiTool decision."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path
import sys


COUNTED_STATUSES = {"implemented", "native", "idiomatic"}


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--inventory",
        type=Path,
        default=Path("parity/hutool-v5.8.46-api.csv"),
    )
    parser.add_argument(
        "--decisions",
        type=Path,
        default=Path("parity/decisions.csv"),
    )
    parser.add_argument("--require-complete", action="store_true")
    args = parser.parse_args()

    inventory = read_rows(args.inventory)
    decisions = read_rows(args.decisions)
    inventory_ids = {row["api_id"] for row in inventory}
    if len(inventory_ids) != len(inventory):
        print("inventory contains duplicate api_id values", file=sys.stderr)
        return 2

    indexed: dict[str, dict[str, str]] = {}
    invalid: list[str] = []
    for row in decisions:
        api_id = row.get("api_id", "")
        if not api_id or api_id in indexed or api_id not in inventory_ids:
            invalid.append(api_id or "<empty>")
            continue
        if row.get("status") not in COUNTED_STATUSES:
            invalid.append(api_id)
            continue
        if not row.get("hitool_symbol") or not row.get("test_evidence"):
            invalid.append(api_id)
            continue
        indexed[api_id] = row

    total = len(inventory)
    covered = len(indexed)
    missing = total - covered
    percentage = 100.0 if total == 0 else covered * 100.0 / total
    print(
        f"Hutool API parity: {covered}/{total} ({percentage:.2f}%), "
        f"missing={missing}, invalid_decisions={len(invalid)}"
    )
    if invalid:
        print("Invalid decisions:", file=sys.stderr)
        for api_id in invalid[:20]:
            print(f"  {api_id}", file=sys.stderr)
    if args.require_complete and (missing or invalid):
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

