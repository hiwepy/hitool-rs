#!/usr/bin/env python3
"""Record reviewed Hutool HttpBase APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpBase"


def evidence(qualified_name: str) -> tuple[str, str]:
    member = qualified_name.rsplit("::", 1)[-1]
    if member in {"bodyBytes", "charset", "httpVersion", "toString"}:
        return (
            "crates/hutool-http/src/base.rs::version_body_charset_and_display_match_hutool_metadata_semantics",
            "Encoding Standard character sets, owned body bytes, protocol labels and Hutool-style display output are explicit and fallible where Rust requires it.",
        )
    return (
        "crates/hutool-http/src/base.rs::headers_are_case_insensitive_mutable_and_wire_aggregation_is_explicit",
        "Rust methods consolidate Java overloads while preserving case-insensitive lookup, append/overwrite/remove behavior and optional comma aggregation.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        qualified_name = row["qualified_name"]
        if row["module"] != "hutool-http" or not qualified_name.startswith(PREFIX):
            continue
        selected += 1
        test, notes = evidence(qualified_name)
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": "hutool_http::HttpBase",
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 25:
        raise SystemExit(f"expected 25 reviewed HttpBase APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool HttpBase APIs")


if __name__ == "__main__":
    main()
