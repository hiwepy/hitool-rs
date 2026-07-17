#!/usr/bin/env python3
"""Record semantically verified Hutool HttpConfig APIs with runtime evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpConfig"
VERIFIED_MEMBERS = {
    "HttpConfig",
    "create",
    "timeout",
    "setConnectionTimeout",
    "setReadTimeout",
    "disableCache",
    "setMaxRedirectCount",
    "setHostnameVerifier",
    "setHttpProxy",
    "setProxy",
    "setSSLSocketFactory",
    "setSSLProtocol",
    "addRequestInterceptor",
    "addResponseInterceptor",
}


def evidence(member: str) -> tuple[str, str]:
    if member == "setProxy":
        return (
            "crates/hitool-http/src/coverage_tests.rs::configured_proxy_is_the_real_transport_destination",
            "The Reqwest proxy engine receives the configuration and a real request reaches the configured proxy endpoint.",
        )
    if member in {"disableCache", "addRequestInterceptor", "addResponseInterceptor"}:
        return (
            "crates/hitool-http/src/coverage_tests.rs::runtime_config_changes_real_requests_responses_and_errors",
            "End-to-end transport tests observe cache headers and request/response interceptor mutations and failures.",
        )
    return (
        "crates/hitool-http/src/config.rs::configuration_setters_validate_every_hutool_shape_and_redact_secrets",
        "Hutool configuration is translated into validated Reqwest/Rustls timeouts, redirect, hostname, proxy and TLS settings for async and blocking clients.",
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
        member = qualified_name.rsplit("::", 1)[-1]
        if member not in VERIFIED_MEMBERS:
            indexed.pop(row["api_id"], None)
            continue
        selected += 1
        test, notes = evidence(member)
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": "hitool_http::HttpConfig",
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 14:
        raise SystemExit(f"expected 14 semantically verified HttpConfig APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} semantically verified Hutool HttpConfig APIs")


if __name__ == "__main__":
    main()
