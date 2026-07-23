#!/usr/bin/env python3
"""Verify that every pinned Hutool API has an evidenced HiTool decision."""

from __future__ import annotations

import argparse
import csv
from collections import defaultdict
from pathlib import Path
import sys


# Count toward API parity percentage.
COUNTED_STATUSES = {"implemented", "native", "idiomatic"}
# Valid ledger rows that do not raise covered % (deferrals / policy).
REGISTERED_ONLY_STATUSES = {"compatible", "unsafe-to-copy", "planned"}
VALID_STATUSES = COUNTED_STATUSES | REGISTERED_ONLY_STATUSES


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
    parser.add_argument(
        "--by-module",
        action="store_true",
        help="Print per-module covered/registered totals",
    )
    parser.add_argument(
        "--by-package",
        action="store_true",
        help="Print hutool-core package covered/registered/missing totals",
    )
    parser.add_argument(
        "--feasible",
        action="store_true",
        help="Print feasible_covered%% excluding unportable planned/unsafe rows",
    )
    parser.add_argument(
        "--tags",
        type=Path,
        default=Path("parity/unportable-tags.csv"),
        help="Optional classify-unportable.csv for --feasible",
    )
    args = parser.parse_args()

    inventory = read_rows(args.inventory)
    decisions = read_rows(args.decisions)
    inventory_by_id = {row["api_id"]: row for row in inventory}
    if len(inventory_by_id) != len(inventory):
        print("inventory contains duplicate api_id values", file=sys.stderr)
        return 2

    counted: dict[str, dict[str, str]] = {}
    registered: dict[str, dict[str, str]] = {}
    invalid: list[str] = []
    for row in decisions:
        api_id = row.get("api_id", "")
        status = row.get("status", "")
        if not api_id or api_id in registered or api_id not in inventory_by_id:
            invalid.append(api_id or "<empty>")
            continue
        if status not in VALID_STATUSES:
            invalid.append(api_id)
            continue
        if status in COUNTED_STATUSES:
            if not row.get("hutool_symbol") or not row.get("test_evidence"):
                invalid.append(api_id)
                continue
            counted[api_id] = row
        else:
            # planned / compatible / unsafe-to-copy: notes required; symbol optional.
            if not (row.get("notes") or "").strip():
                invalid.append(api_id)
                continue
        registered[api_id] = row

    total = len(inventory_by_id)
    covered = len(counted)
    registered_n = len(registered)
    missing = total - registered_n
    percentage = 100.0 if total == 0 else covered * 100.0 / total
    registered_pct = 100.0 if total == 0 else registered_n * 100.0 / total
    print(
        f"Hutool API parity: {covered}/{total} ({percentage:.2f}%), "
        f"registered={registered_n}/{total} ({registered_pct:.2f}%), "
        f"missing={missing}, invalid_decisions={len(invalid)}"
    )
    if args.by_module:
        inv_mods: dict[str, set[str]] = defaultdict(set)
        for row in inventory:
            inv_mods[row["module"]].add(row["api_id"])
        print("module                         covered/total  registered/total")
        for module in sorted(inv_mods):
            ids = inv_mods[module]
            c = sum(1 for i in ids if i in counted)
            r = sum(1 for i in ids if i in registered)
            t = len(ids)
            print(f"{module:<30} {c:>4}/{t:<5}      {r:>4}/{t}")

    def core_package(row: dict[str, str]) -> str:
        fp = row.get("file_path", "")
        marker = "/cn/hutool/core/"
        if marker in fp:
            return fp.split(marker, 1)[1].split("/")[0]
        return "?"

    if args.by_package:
        pkgs: dict[str, set[str]] = defaultdict(set)
        for row in inventory:
            if row["module"] != "hutool-core":
                continue
            pkgs[core_package(row)].add(row["api_id"])
        print("core_package                   covered/total  registered/total  missing")
        for package in sorted(pkgs, key=lambda p: -len(pkgs[p])):
            ids = pkgs[package]
            c = sum(1 for i in ids if i in counted)
            r = sum(1 for i in ids if i in registered)
            t = len(ids)
            print(f"{package:<30} {c:>4}/{t:<5}      {r:>4}/{t:<5}      {t - r}")

    if args.feasible:
        unportable_tags = {
            "jvm_only",
            "reflection",
            "awt_swing",
            "javax_servlet",
            "javax_sql_spi",
            "jndi",
            "bouncycastle_only",
            "soap_server",
        }
        tags: dict[str, str] = {}
        if args.tags.exists():
            with args.tags.open(encoding="utf-8", newline="") as stream:
                for row in csv.DictReader(stream):
                    tags[row["api_id"]] = row.get("tag", "portable")
        # Fallback: treat planned/unsafe with known note keywords as unportable.
        unportable_ids: set[str] = set()
        for api_id, row in registered.items():
            tag = tags.get(api_id)
            if tag in unportable_tags:
                unportable_ids.add(api_id)
                continue
            if row.get("status") in REGISTERED_ONLY_STATUSES:
                notes = (row.get("notes") or "").lower()
                if any(
                    k in notes
                    for k in (
                        "jvm",
                        "reflection",
                        "swing",
                        "servlet",
                        "jndi",
                        "jdbc",
                        "bouncy",
                        "soap",
                        "planned until easy",
                        "poi deferred",
                        "unsafe-to-copy",
                    )
                ):
                    unportable_ids.add(api_id)
        feasible_denom = total - len(unportable_ids)
        feasible_cov = sum(1 for i in counted if i not in unportable_ids)
        feasible_pct = 100.0 if feasible_denom == 0 else feasible_cov * 100.0 / feasible_denom
        print(
            f"feasible_covered: {feasible_cov}/{feasible_denom} ({feasible_pct:.2f}%), "
            f"unportable_excluded={len(unportable_ids)}"
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
