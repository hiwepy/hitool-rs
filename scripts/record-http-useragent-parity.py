#!/usr/bin/env python3
"""Record reviewed Hutool HTTP User-Agent APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http.useragent::"


def mapping(qualified_name: str) -> tuple[str, str, str]:
    class_name = qualified_name.split("::")[1]
    if class_name == "UserAgentInfo":
        return (
            "hitool_http::useragent::UserAgentInfo",
            "crates/hitool-http/src/useragent.rs::information_rules_compare_hash_and_display_by_name",
            "Case-insensitive named rules preserve Hutool name-based equality, hashing, matching, unknown detection and display semantics.",
        )
    if class_name == "Browser":
        return (
            "hitool_http::useragent::Browser",
            "crates/hitool-http/src/useragent.rs::custom_rules_and_mutators_are_supported",
            "Built-in and custom browser rules expose version extraction and Hutool-compatible mobile-browser classification.",
        )
    if class_name == "Engine":
        return (
            "hitool_http::useragent::Engine",
            "crates/hitool-http/src/useragent.rs::parses_desktop_chrome_with_versions",
            "Rendering-engine detection and version extraction use owned Rust values with name-based identity.",
        )
    if class_name == "OS":
        return (
            "hitool_http::useragent::OperatingSystem",
            "crates/hitool-http/src/useragent.rs::parses_mobile_wechat_and_ios_helpers",
            "Operating-system rules preserve custom registration, version normalization, unknown handling and macOS detection.",
        )
    if class_name == "Platform":
        return (
            "hitool_http::useragent::Platform",
            "crates/hitool-http/src/useragent.rs::platform_family_helpers_cover_all_families",
            "Platform helpers cover mobile, iOS, Android, Google TV and HarmonyOS with explicit immutable classifications.",
        )
    if class_name == "UserAgent":
        return (
            "hitool_http::useragent::UserAgent",
            "crates/hitool-http/src/useragent.rs::custom_rules_and_mutators_are_supported",
            "The owned aggregate exposes every Hutool browser, engine, OS, platform, version and mobile getter/setter capability.",
        )
    return (
        "hitool_http::useragent::{UserAgentParser,UserAgentUtil}",
        "crates/hitool-http/src/useragent.rs::blank_unknown_and_woothee_fallbacks_are_safe",
        "The stateless facade rejects blank input and combines Hutool-specific rules with the mature Woothee fallback parser.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-http" or not row["qualified_name"].startswith(PREFIX):
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 53:
        raise SystemExit(f"expected 53 reviewed HTTP User-Agent APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool HTTP User-Agent APIs")


if __name__ == "__main__":
    main()
