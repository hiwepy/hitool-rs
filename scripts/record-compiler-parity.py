#!/usr/bin/env python3
"""Record reviewed Hutool compiler APIs with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.compiler::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "CompilerException": (
        "hutool_core::CompilerException",
        "source_loading_and_exception_constructors_are_bounded",
    ),
    "CompilerUtil": (
        "hutool_core::RustSourceCompiler",
        "rust_source_compiler_builds_and_reports_diagnostics",
    ),
    "DiagnosticUtil": (
        "hutool_core::diagnostic_messages",
        "file_objects_enforce_names_limits_and_manage_artifacts",
    ),
    "JavaClassFileManager": (
        "hutool_core::ClassFileManager",
        "file_objects_enforce_names_limits_and_manage_artifacts",
    ),
    "JavaClassFileObject": (
        "hutool_core::ClassFileObject",
        "file_objects_enforce_names_limits_and_manage_artifacts",
    ),
    "JavaFileObjectUtil": (
        "hutool_core::SourceFileObjectUtil",
        "source_loading_and_exception_constructors_are_bounded",
    ),
    "JavaSourceCompiler": (
        "hutool_core::RustSourceCompiler",
        "rust_source_compiler_builds_and_reports_diagnostics",
    ),
    "JavaSourceFileObject": (
        "hutool_core::SourceFileObject",
        "source_loading_and_exception_constructors_are_bounded",
    ),
}


def family(qualified_name: str) -> str | None:
    if not qualified_name.startswith(ROOT):
        return None
    candidate = qualified_name[len(ROOT) :].split("::", 1)[0]
    return candidate if candidate in FAMILIES else None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        compiler_family = family(row["qualified_name"])
        if compiler_family is None:
            continue
        selected += 1
        symbol, test = FAMILIES[compiler_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": f"crates/hutool-core/src/compiler.rs::{test}",
            "notes": (
                "The JVM compiler and class-loader workflow maps to bounded Rust "
                "source units, rustc execution, artifacts, and sourced diagnostics."
            ),
        }

    if selected != 33:
        raise SystemExit(f"expected 33 reviewed compiler APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool compiler APIs")


if __name__ == "__main__":
    main()
