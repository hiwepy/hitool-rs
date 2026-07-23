#!/usr/bin/env python3
"""Verify Hutool → hitool-rs test-method parity ledger.

Two bars (do not conflate them):

1. **Inventory / registration** (`--require-complete`):
   Every Hutool `@Test` has a decision row (covered | ignored | planned).
   `#[ignore]` stubs still count here — they only prove the id is registered.

2. **Behavioral** (`--require-behavioral`):
   Every Hutool `@Test` is either status=`covered` (runnable, real asserts) or
   status=`planned` (accepted deferral: JVM reflection, Java `@Disabled`, no Rust
   equivalent). Any remaining `ignored` soft stubs fail the gate.
"""

from __future__ import annotations

import argparse
import csv
from collections import Counter, defaultdict
from pathlib import Path
import sys


# Runnable behavioral evidence only.
BEHAVIORAL = {"covered"}
# Registered against the Hutool inventory (may still be ignore stubs).
REGISTERED = {"covered", "ignored", "planned"}


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--inventory",
        type=Path,
        default=Path("parity/hutool-v5.8.46-tests.csv"),
    )
    parser.add_argument(
        "--decisions",
        type=Path,
        default=Path("parity/test-decisions.csv"),
    )
    parser.add_argument(
        "--require-complete",
        action="store_true",
        help="Fail unless every inventory id is registered (covered|ignored|planned)",
    )
    parser.add_argument(
        "--require-behavioral",
        action="store_true",
        help="Fail unless every inventory id is runnable covered (no #[ignore] stubs)",
    )
    parser.add_argument(
        "--by-module",
        action="store_true",
        help="Print per-module coverage table",
    )
    parser.add_argument(
        "--missing-limit",
        type=int,
        default=30,
        help="How many missing / ignored test_ids to print",
    )
    args = parser.parse_args()

    inventory = read_rows(args.inventory)
    decisions = read_rows(args.decisions)
    inventory_ids = {row["test_id"]: row for row in inventory}

    by_id: dict[str, dict[str, str]] = {}
    invalid: list[str] = []
    for row in decisions:
        tid = row.get("test_id", "")
        if not tid or tid.startswith("UNRESOLVED::"):
            invalid.append(tid or "<empty>")
            continue
        if tid not in inventory_ids:
            invalid.append(tid)
            continue
        by_id[tid] = row

    total = len(inventory_ids)
    status_counts: Counter[str] = Counter()
    behavioral = 0
    registered = 0
    ignored = 0
    planned = 0
    for tid in inventory_ids:
        row = by_id.get(tid)
        if not row:
            status_counts["missing"] += 1
            continue
        st = row.get("status", "")
        status_counts[st] += 1
        if st in BEHAVIORAL and row.get("hitool_test"):
            behavioral += 1
            registered += 1
        elif st == "ignored" and row.get("hitool_test"):
            ignored += 1
            registered += 1
        elif st == "planned" and row.get("hitool_test"):
            planned += 1
            registered += 1

    missing = total - registered
    pct_reg = 100.0 if total == 0 else registered * 100.0 / total
    pct_beh = 100.0 if total == 0 else behavioral * 100.0 / total

    print(
        f"Hutool TEST registration: {registered}/{total} ({pct_reg:.2f}%), "
        f"missing={missing}, invalid_or_orphan={len(invalid)}"
    )
    print(
        f"Hutool TEST behavioral:   {behavioral}/{total} ({pct_beh:.2f}%), "
        f"ignored_stubs={ignored}, planned={planned}"
    )
    print(
        "  (behavioral = runnable covered asserts; "
        "planned = accepted deferrals; ignored = unfinished soft stubs)"
    )

    if args.by_module:
        mod_total: Counter[str] = Counter()
        mod_beh: Counter[str] = Counter()
        mod_ign: Counter[str] = Counter()
        mod_reg: Counter[str] = Counter()
        for tid, row in inventory_ids.items():
            mod = row["module"]
            mod_total[mod] += 1
            d = by_id.get(tid)
            if not d:
                continue
            st = d.get("status", "")
            if st in BEHAVIORAL and d.get("hitool_test"):
                mod_beh[mod] += 1
                mod_reg[mod] += 1
            elif st == "ignored" and d.get("hitool_test"):
                mod_ign[mod] += 1
                mod_reg[mod] += 1
            elif st in REGISTERED and d.get("hitool_test"):
                mod_reg[mod] += 1
        print("\nPer-module (behavioral / ignored / registered / total):")
        print(
            f"{'module':<22} {'behav':>7} {'ignore':>7} {'reg':>7} {'total':>7} {'beh%':>7}"
        )
        for mod in sorted(mod_total, key=lambda m: -mod_total[m]):
            t = mod_total[mod]
            b = mod_beh[mod]
            i = mod_ign[mod]
            r = mod_reg[mod]
            p = 100.0 * b / t if t else 0.0
            print(f"{mod:<22} {b:7d} {i:7d} {r:7d} {t:7d} {p:6.1f}%")

    if missing and args.missing_limit > 0:
        print(f"\nMissing registration sample (first {args.missing_limit}):")
        shown = 0
        for tid in sorted(inventory_ids):
            d = by_id.get(tid)
            if d and d.get("status") in REGISTERED and d.get("hitool_test"):
                continue
            row = inventory_ids[tid]
            print(f"  {tid}  ({row['file_path']}:{row['start_line']})")
            shown += 1
            if shown >= args.missing_limit:
                break

    if ignored and args.missing_limit > 0 and (
        args.require_behavioral or args.by_module
    ):
        print(f"\nIgnored stub sample (first {args.missing_limit}):")
        shown = 0
        for tid in sorted(inventory_ids):
            d = by_id.get(tid)
            if not d or d.get("status") != "ignored":
                continue
            row = inventory_ids[tid]
            print(
                f"  {tid}  → {d.get('hitool_test')}  "
                f"({row['file_path']}:{row['start_line']})"
            )
            shown += 1
            if shown >= args.missing_limit:
                break

    if invalid:
        print("\nInvalid/orphan decisions (sample):", file=sys.stderr)
        for tid in invalid[:20]:
            print(f"  {tid}", file=sys.stderr)

    if args.require_complete and (missing or invalid):
        return 1
    # Behavioral bar: every inventory id is either runnable `covered` or accepted
    # deferred `planned` (JVM reflection / Java @Disabled / no Rust equivalent).
    # Fail on any remaining `ignored` stubs (soft asserts / unfinished ignores).
    if args.require_behavioral and (
        ignored or missing or invalid or (behavioral + planned) != total
    ):
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
