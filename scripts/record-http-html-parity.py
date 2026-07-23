#!/usr/bin/env python3
"""Record reviewed Hutool HtmlUtil / HTMLFilter APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

HTML_UTIL = {
    "HtmlUtil": (
        "hitool_http::html::HtmlUtil",
        "crates/hitool-http/tests/html_util_parity.rs::clean_html_tag_test",
        "Stateless HTML helpers ported from Hutool HtmlUtil.",
    ),
    "escape": (
        "hitool_http::html::HtmlUtil::escape",
        "crates/hitool-http/tests/html_util_parity.rs::clean_html_tag_test",
        "Escapes HTML special characters including NBSP.",
    ),
    "unescape": (
        "hitool_http::html::HtmlUtil::unescape",
        "crates/hitool-http/tests/html_util_parity.rs::clean_html_tag_test",
        "Restores common HTML4 entities.",
    ),
    "cleanHtmlTag": (
        "hitool_http::html::HtmlUtil::clean_html_tag",
        "crates/hitool-http/tests/html_util_parity.rs::clean_html_tag_test",
        "Strips tags while keeping inner text.",
    ),
    "cleanEmptyTag": (
        "hitool_http::html::HtmlUtil::clean_empty_tag",
        "crates/hitool-http/tests/html_util_parity.rs::clean_empty_tag_test",
        "Removes empty paired tags.",
    ),
    "removeHtmlTag": (
        "hitool_http::html::HtmlUtil::remove_html_tag",
        "crates/hitool-http/tests/html_util_parity.rs::remove_html_tag_test",
        "Named-tag removal with optional content retention overload.",
    ),
    "unwrapHtmlTag": (
        "hitool_http::html::HtmlUtil::unwrap_html_tag",
        "crates/hitool-http/tests/html_util_parity.rs::unwrap_html_tag_test",
        "Removes tags while keeping inner content.",
    ),
    "removeHtmlAttr": (
        "hitool_http::html::HtmlUtil::remove_html_attr",
        "crates/hitool-http/tests/html_util_parity.rs::remove_html_attr_test",
        "Attribute removal including issue I8YV0K trailing-space cleanup.",
    ),
    "removeAllHtmlAttr": (
        "hitool_http::html::HtmlUtil::remove_all_html_attr",
        "crates/hitool-http/tests/html_util_parity.rs::remove_all_html_attr_test",
        "Clears all attributes from named tags.",
    ),
    "filter": (
        "hitool_http::html::HtmlUtil::filter",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Delegates to HTMLFilter default whitelist.",
    ),
}

HTML_FILTER = {
    "HTMLFilter": (
        "hitool_http::html::HtmlFilter",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "XSS HTML whitelist filter ported from Hutool HTMLFilter.",
    ),
    "chr": (
        "hitool_http::html::HtmlFilter::chr",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Decimal code point to character.",
    ),
    "htmlSpecialChars": (
        "hitool_http::html::HtmlFilter::html_special_chars",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Escapes &, quotes, and angle brackets.",
    ),
    "filter": (
        "hitool_http::html::HtmlFilter::filter",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Whitelist filter pipeline.",
    ),
    "isAlwaysMakeTags": (
        "hitool_http::html::HtmlFilter::is_always_make_tags",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Reports alwaysMakeTags configuration.",
    ),
    "isStripComments": (
        "hitool_http::html::HtmlFilter::is_strip_comments",
        "crates/hitool-http/tests/html_util_parity.rs::html_filter_issue3433_test",
        "Reports stripComment configuration.",
    ),
}


def member_of(api_id: str, type_name: str) -> str:
    rest = api_id.split("::", 2)[-1]
    if rest.endswith("#") and "#" not in rest[:-1]:
        return type_name
    name = rest.split("#", 1)[0]
    if name.startswith(f"{type_name}::"):
        name = name.split("::", 1)[1]
    if name == type_name:
        # constructor overloads HTMLFilter#() etc.
        return type_name
    return name


def record_type(indexed: dict, inventory: list, prefix: str, type_name: str, table: dict) -> int:
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-http" or not row["api_id"].startswith(prefix):
            continue
        member = member_of(row["api_id"], type_name)
        # Constructors share the type name in Hutool inventory.
        if "HTMLFilter#" in row["api_id"] and row["api_id"].count("::") == 1:
            decision = table.get("HTMLFilter")
        elif member == type_name and "HTMLFilter#" in row["api_id"]:
            decision = table.get("HTMLFilter")
        else:
            decision = table.get(member)
            if decision is None and member == type_name:
                decision = table.get(type_name)
        if decision is None and row["signature"].startswith("("):
            # Constructor overloads
            decision = table.get(type_name)
        if decision is None:
            continue
        symbol, evidence, notes = decision
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
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

    util_count = record_type(
        indexed, inventory, "cn.hutool.http::HtmlUtil", "HtmlUtil", HTML_UTIL
    )
    filter_count = record_type(
        indexed, inventory, "cn.hutool.http::HTMLFilter", "HTMLFilter", HTML_FILTER
    )
    if util_count != 11:
        raise SystemExit(f"expected 11 HtmlUtil APIs, got {util_count}")
    if filter_count != 9:
        raise SystemExit(f"expected 9 HTMLFilter APIs, got {filter_count}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {util_count} HtmlUtil + {filter_count} HTMLFilter APIs")


if __name__ == "__main__":
    main()
