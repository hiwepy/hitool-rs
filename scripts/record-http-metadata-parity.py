#!/usr/bin/env python3
"""Record reviewed Hutool HTTP metadata APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
CLASSES = {
    "ContentType",
    "GlobalHeaders",
    "Header",
    "HttpStatus",
    "Method",
    "Status",
}


def mapping(class_name: str) -> tuple[str, str, str]:
    if class_name == "ContentType":
        return (
            "hutool_http::ContentType",
            "crates/hutool-http/src/metadata.rs::content_types_format_detect_and_classify_every_shape",
            "A typed Rust enum preserves Hutool media values, charset formatting, default checks and JSON/XML body detection.",
        )
    if class_name == "GlobalHeaders":
        return (
            "hutool_http::GlobalHeaders",
            "crates/hutool-http/src/metadata.rs::owned_global_headers_cover_defaults_overloads_mutation_and_reset",
            "Hutool header overloads are consolidated into an explicitly owned collection with the same defaults, append, overwrite, remove and reset behavior.",
        )
    if class_name == "Header":
        return (
            "hutool_http::Header",
            "crates/hutool-http/src/metadata.rs::methods_headers_and_statuses_match_hutool_catalogs",
            "The typed enum exposes every Hutool header wire value with value and display accessors.",
        )
    if class_name == "HttpStatus":
        return (
            "hutool_http::HttpStatus",
            "crates/hutool-http/src/metadata.rs::methods_headers_and_statuses_match_hutool_catalogs",
            "All Hutool status constants and its exact five-code redirect predicate are available without allocation.",
        )
    if class_name == "Status":
        return (
            "hutool_http::Status",
            "crates/hutool-http/src/metadata.rs::methods_headers_and_statuses_match_hutool_catalogs",
            "The legacy Hutool status namespace is retained as associated Rust constants.",
        )
    return (
        "hutool_http::Method",
        "crates/hutool-http/src/metadata.rs::methods_headers_and_statuses_match_hutool_catalogs",
        "The mature reqwest Method type natively provides all nine Hutool HTTP methods.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        parts = row["qualified_name"].split("::")
        if row["module"] != "hutool-http" or len(parts) < 2 or parts[1] not in CLASSES:
            continue
        selected += 1
        symbol, test, notes = mapping(parts[1])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 30:
        raise SystemExit(f"expected 30 reviewed HTTP metadata APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool HTTP metadata APIs")


if __name__ == "__main__":
    main()
