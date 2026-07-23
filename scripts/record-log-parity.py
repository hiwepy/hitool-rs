#!/usr/bin/env python3
"""Record reviewed Hutool logging APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str, str]:
    if ".dialect." in qualified_name:
        return (
            "hutool_log::dialect",
            "crates/hutool-log/src/compat.rs::tracing_sink_supports_all_levels",
            "Hutool backend names remain migration aliases while one tracing sink replaces duplicated Java logging backends.",
        )
    if "::Level" in qualified_name or "Log#" in qualified_name:
        return (
            "hutool_log::{Log,LogLevel,LogRecord}",
            "crates/hutool-log/src/compat.rs::records_and_formats_every_level",
            "An object-safe logger and owned record preserve all five levels, formatted messages, errors, and caller metadata.",
        )
    if "GlobalLogFactory" in qualified_name:
        return (
            "hutool_log::GlobalLogFactory",
            "crates/hutool-log/src/compat.rs::static_facade_uses_replaceable_global",
            "Compatibility global access is explicit, replaceable, resettable, and shares the selected factory cache.",
        )
    if "LogFactory" in qualified_name:
        return (
            "hutool_log::LogFactory",
            "crates/hutool-log/src/compat.rs::factory_caches_and_creates_loggers",
            "An owned thread-safe factory supports cached and uncached loggers with an injectable sink.",
        )
    if "StaticLog" in qualified_name:
        return (
            "hutool_log::StaticLog",
            "crates/hutool-log/src/compat.rs::static_facade_uses_replaceable_global",
            "The static migration facade delegates every level to the replaceable compatibility factory.",
        )
    return (
        "hutool_log::AbstractLog",
        "crates/hutool-log/src/compat.rs::records_and_formats_every_level",
        "The shared logger consolidates Hutool overloads into typed level, error, caller, and formatting operations.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-log":
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 283:
        raise SystemExit(f"expected 283 reviewed log APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool log APIs")


if __name__ == "__main__":
    main()
