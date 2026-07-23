#!/usr/bin/env python3
"""Mark Hutool HttpConnection APIs as planned (reqwest has no HttpURLConnection peer)."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpConnection"

NOTE = (
    "Java HttpURLConnection wrapper; reqwest owns headers/TLS/connect. "
    "Use HttpRequest/HttpClient header and config APIs instead of a parallel HttpConnection type."
)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-http" or not row["api_id"].startswith(PREFIX):
            continue
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "planned",
            "hitool_symbol": "",
            "test_evidence": "",
            "notes": NOTE,
        }
        selected += 1

    if selected != 36:
        raise SystemExit(f"expected 36 HttpConnection APIs, got {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} HttpConnection APIs as planned")


if __name__ == "__main__":
    main()
