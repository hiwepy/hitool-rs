#!/usr/bin/env python3
"""Record Hutool MultipartBody and HttpException APIs with Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

MULTIPART = {
    "MultipartBody": (
        "hutool_http::MultipartBody",
        "crates/hutool-http/tests/http_util_offline_parity.rs::multipart_body_test_build_test",
        "RFC2388 multipart/form-data builder for string form fields.",
    ),
    "create": (
        "hutool_http::MultipartBody::create",
        "crates/hutool-http/tests/http_util_offline_parity.rs::multipart_body_test_build_test",
        "Factory matching Hutool MultipartBody.create.",
    ),
    "getContentType": (
        "hutool_http::MultipartBody::content_type",
        "crates/hutool-http/tests/http_util_offline_parity.rs::multipart_body_test_build_test",
        "Returns multipart Content-Type with boundary.",
    ),
    "write": (
        "hutool_http::MultipartBody::write",
        "crates/hutool-http/tests/http_util_offline_parity.rs::multipart_body_test_build_test",
        "Writes serialized multipart bytes to a Write sink.",
    ),
    "toString": (
        "hutool_http::MultipartBody",
        "crates/hutool-http/tests/http_util_offline_parity.rs::multipart_body_test_build_test",
        "Display summary of charset and field count.",
    ),
}

EXCEPTION = {
    "HttpException": (
        "hutool_http::HttpException",
        "crates/hutool-http/src/exception.rs::constructors_match_hutool_message_shapes",
        "Owned message exception mirroring Hutool constructors; transport uses HttpError.",
    ),
}


def member_of(api_id: str, type_name: str) -> str:
    rest = api_id.split("::", 2)[-1]
    if rest.endswith("#") and "#" not in rest[:-1]:
        return type_name
    name = rest.split("#", 1)[0]
    if f"{type_name}::" in name:
        name = name.split("::")[-1]
    return name if name else type_name


def record_prefix(indexed: dict, inventory: list, prefix: str, type_name: str, table: dict) -> int:
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-http" or not row["api_id"].startswith(prefix):
            continue
        member = member_of(row["api_id"], type_name)
        decision = table.get(member)
        if decision is None and (
            member == type_name or row["signature"].startswith("(") or not row["signature"]
        ):
            decision = table.get(type_name)
        if decision is None:
            continue
        symbol, evidence, notes = decision
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }
        selected += 1
    return selected


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    mp = record_prefix(
        indexed,
        inventory,
        "cn.hutool.http.body::MultipartBody",
        "MultipartBody",
        MULTIPART,
    )
    ex = record_prefix(
        indexed,
        inventory,
        "cn.hutool.http::HttpException",
        "HttpException",
        EXCEPTION,
    )
    if mp != 6:
        raise SystemExit(f"expected 6 MultipartBody APIs, got {mp}")
    if ex != 7:
        raise SystemExit(f"expected 7 HttpException APIs, got {ex}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {mp} MultipartBody + {ex} HttpException APIs")


if __name__ == "__main__":
    main()
