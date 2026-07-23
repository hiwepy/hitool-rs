#!/usr/bin/env python3
"""Record reviewed Hutool system APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
COMPAT = "crates/hutool-system/src/compat.rs::"
OSHI = "crates/hutool-system/src/oshi.rs::"


def mapping(qualified_name: str) -> tuple[str, str, str]:
    tail = qualified_name.split("::", 1)[1]
    family = tail.split("::", 1)[0]

    if qualified_name.startswith("cn.hutool.system.oshi::"):
        if family in {"CpuInfo", "CpuTicks"}:
            test = "ticks_and_cpu_models_are_normalized_and_mutable_as_rust_data"
        else:
            test = "live_collectors_return_consistent_process_memory_and_hardware_views"
        return (
            f"hutool_system::{family}",
            OSHI + test,
            "The mature sysinfo engine provides real CPU, process, memory, disk, network, "
            "and sensor snapshots; Rust fields replace JavaBean getter/setter boilerplate.",
        )

    if family == "OsInfo":
        test = "os_predicates_cover_every_hutool_family_and_version"
        note = "Deterministic OS family/version predicates preserve Hutool behavior while native separators come from the Rust target."
    elif family in {"JavaInfo", "JavaRuntimeInfo"}:
        test = "java_versions_and_runtime_paths_are_deterministic"
        note = (
            "Java/JVM properties are returned only when explicitly configured; HiTool never "
            "spawns a hidden JVM. Version and path behavior remains available as typed Rust data."
        )
    elif family in {"JavaSpecInfo", "JvmInfo", "JvmSpecInfo"}:
        test = "live_property_runtime_and_management_facades_are_consistent"
        note = (
            "JVM-specific properties are explicit optional snapshots; absent JVM state is not "
            "fabricated in a native Rust process."
        )
    elif family == "SystemUtil":
        test = "live_property_runtime_and_management_facades_are_consistent"
        note = (
            "JVM MXBeans map to native process, memory, OS, compilation, and thread-capacity "
            "snapshots; managed heap pools, managers, and collectors are explicitly empty in native Rust."
        )
    else:
        test = "live_property_runtime_and_management_facades_are_consistent"
        note = "Portable host, runtime, user, locale, path, and system-property capabilities use owned Rust snapshots and typed I/O."
    return f"hutool_system::{family}", COMPAT + test, note


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-system":
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

    if selected != 189:
        raise SystemExit(f"expected 189 reviewed system APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool system APIs")


if __name__ == "__main__":
    main()
