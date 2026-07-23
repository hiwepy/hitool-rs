#!/usr/bin/env python3
"""Record reviewed Hutool script APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str] | None:
    prefix = "cn.hutool.script::"
    if not qualified_name.startswith(prefix):
        return None
    family = qualified_name.removeprefix(prefix).split("::", 1)[0]
    if family == "FullSupportScriptEngine":
        invocation = any(token in qualified_name for token in ("invoke", "getInterface"))
        test = (
            "crates/hutool-script/src/compat.rs::invocable_functions_methods_and_interfaces_share_the_active_ast"
            if invocation
            else "crates/hutool-script/src/compat.rs::full_engine_compiles_evaluates_readers_bindings_and_contexts"
        )
        return "hutool_script::FullSupportScriptEngine", test
    if family == "JavaScriptEngine":
        return (
            "hutool_script::JavaScriptEngine",
            "crates/hutool-script/src/compat.rs::javascript_wrapper_and_util_cover_supported_and_optional_languages",
        )
    if family == "ScriptRuntimeException":
        return (
            "hutool_script::ScriptRuntimeException",
            "crates/hutool-script/src/compat.rs::runtime_exception_constructors_and_locations_match_hutool",
        )
    if family == "ScriptUtil":
        return (
            "hutool_script::ScriptUtil",
            "crates/hutool-script/src/compat.rs::javascript_wrapper_and_util_cover_supported_and_optional_languages",
        )
    return None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-script":
            continue
        target = mapping(row["qualified_name"])
        if target is None:
            raise SystemExit(f"unmapped Hutool script family: {row['qualified_name']}")
        selected += 1
        symbol, test = target
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": test,
            "notes": (
                "Sandboxed Rhai replaces JSR-223 while preserving compile, evaluate, bindings, "
                "context, reader, invocation, factory, and location-rich error capabilities; "
                "optional external languages return an explicit unsupported-engine error."
            ),
        }

    if selected != 79:
        raise SystemExit(f"expected 79 reviewed script APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool script APIs")


if __name__ == "__main__":
    main()
