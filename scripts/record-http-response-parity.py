#!/usr/bin/env python3
"""Record only runtime-verified Hutool HttpResponse capabilities."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpResponse"
SIMPLE_MEMBERS = {
    "getStatus",
    "isOk",
    "contentEncoding",
    "contentLength",
    "isGzip",
    "isDeflate",
    "isChunked",
    "getCookieStr",
    "bodyStream",
    "bodyBytes",
}


def is_verified(row: dict[str, str]) -> bool:
    member = row["qualified_name"].rsplit("::", 1)[-1]
    if member in SIMPLE_MEMBERS:
        return True
    if member == "body":
        return row["signature"] in {"HttpResponse (byte[] bodyBytes)", "String ()"}
    return member == "writeBody" and row["signature"].startswith("long (OutputStream out,")


def evidence(member: str) -> tuple[str, str]:
    if member in {"getStatus", "isOk", "getCookieStr"}:
        return (
            "crates/hitool-http/src/coverage_tests.rs::response_facade_preserves_non_success_status_and_bounded_body",
            "A real socket response proves non-success status, raw Set-Cookie and bounded response-body preservation.",
        )
    if member in {"bodyStream", "bodyBytes", "body", "writeBody"}:
        return (
            "crates/hitool-http/src/response.rs::body_decoding_stream_replacement_and_writes_are_real",
            "Executable tests prove charset decoding, repeatable byte/stream access, body replacement and successful and failing writer paths.",
        )
    return (
        "crates/hitool-http/src/response.rs::status_headers_encodings_and_lengths_match_hutool_semantics",
        "Header-driven length, transfer-encoding and content-encoding behavior is verified for valid, absent and invalid wire values.",
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
        if not is_verified(row):
            indexed.pop(row["api_id"], None)
            continue
        selected += 1
        member = qualified_name.rsplit("::", 1)[-1]
        test, notes = evidence(member)
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": "hitool_http::HttpResponse",
            "test_evidence": test,
            "notes": notes,
        }

    if selected != 13:
        raise SystemExit(f"expected 13 semantically verified HttpResponse APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} semantically verified Hutool HttpResponse APIs")


if __name__ == "__main__":
    main()
