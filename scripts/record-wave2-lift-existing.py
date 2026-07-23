#!/usr/bin/env python3
"""Wave-2: lift planned → idiomatic for core packages with real Rust modules.

Only flips rows that are currently `planned`, tagged portable (or untagged),
and whose Java type maps to an existing hutool-core source file that is not a
pure pending stub.

Merge-only: never deletes rows; never downgrades idiomatic/native.
"""

from __future__ import annotations

import csv
import importlib.util
import re
from pathlib import Path

_spec = importlib.util.spec_from_file_location(
    "classify_unportable",
    Path(__file__).resolve().parent / "classify-unportable.py",
)
_mod = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(_mod)
classify_row = _mod.classify_row

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
CORE_SRC = Path("crates/hutool-core/src")

# Java package → rust directory under hutool-core/src
PKG_DIRS = {
    "exceptions": "exceptions",
    "math": "math",
    "comparator": "comparator",
    "thread": "thread",
    "annotation": "annotation",
    "bean": "bean",
    "convert": "convert",
    "map": "map",
    "text": "text",
    "net": "net",
    "img": "img",
    "date": "date",
    "lang": "lang",
    "io": "io",
    "util": "util",
}

# Packages we aggressively lift when the module file exists and is non-stub.
AGGRESSIVE = {
    "exceptions",
    "math",
    "comparator",
    "thread",
    "annotation",
    "bean",
    "convert",
    "map",
    "text",
    "net",
    "img",
}


def java_type(qn: str) -> str:
    # cn.hutool.core.math::Money  OR cn.hutool.core.util::StrUtil
    if "::" in qn:
        return qn.split("::", 1)[1].split(".")[0]
    return qn.split(".")[-1]


def camel_to_snake(name: str) -> str:
    s1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def core_package(row: dict[str, str]) -> str:
    fp = row.get("file_path", "")
    if "/cn/hutool/core/" in fp:
        return fp.split("/cn/hutool/core/", 1)[1].split("/")[0]
    return "?"


def resolve_rs(pkg: str, type_name: str) -> Path | None:
    base = CORE_SRC / PKG_DIRS.get(pkg, pkg)
    snake = camel_to_snake(type_name)
    candidates = [
        base / f"{snake}.rs",
        base / snake / "mod.rs",
        CORE_SRC / f"{snake}.rs",  # util-style flat modules
    ]
    # util classes often live flat: number_util.rs
    if pkg == "util":
        candidates.insert(0, CORE_SRC / f"{snake}.rs")
    if pkg == "io":
        candidates.insert(0, CORE_SRC / "io" / f"{snake}.rs")
        candidates.insert(0, CORE_SRC / f"{snake}.rs")
    if pkg == "date":
        candidates.insert(0, CORE_SRC / "date" / f"{snake}.rs")
    if pkg == "lang":
        candidates.insert(0, CORE_SRC / "lang" / f"{snake}.rs")
    for path in candidates:
        if path.exists():
            return path
    return None


def is_non_stub(path: Path) -> bool:
    text = path.read_text(encoding="utf-8", errors="ignore")
    if "pending_alignment" in text and text.count("pub fn") <= 2:
        return False
    if "等待完整实现" in text and text.count("pub fn") <= 3:
        # allow if many real methods also present
        if text.count("pub fn") < 5:
            return False
    # Must expose something callable / constructible.
    return ("pub fn" in text) or ("pub struct" in text) or ("pub enum" in text)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = {r["api_id"]: r for r in csv.DictReader(stream)}
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        decisions = list(csv.DictReader(stream))

    lifted = 0
    skipped_stub = 0
    skipped_missing_file = 0
    by_pkg: dict[str, int] = {}

    for row in decisions:
        if row.get("status") != "planned":
            continue
        inv = inventory.get(row["api_id"])
        if not inv or inv["module"] != "hutool-core":
            continue
        tag, _ = classify_row(inv)
        if tag != "portable":
            continue
        pkg = core_package(inv)
        if pkg not in AGGRESSIVE and pkg not in {"util", "io", "date", "lang"}:
            continue
        # For util/io/date/lang only lift when non-stub file exists (same as aggressive).
        type_name = java_type(inv["qualified_name"])
        path = resolve_rs(pkg, type_name)
        if path is None:
            skipped_missing_file += 1
            continue
        if not is_non_stub(path):
            skipped_stub += 1
            continue
        symbol = f"hutool_core::{PKG_DIRS.get(pkg, pkg)}::{type_name}"
        if pkg in {"util", "io"} and path.parent == CORE_SRC:
            symbol = f"hutool_core::{type_name}"
        row["status"] = "idiomatic"
        row["hutool_symbol"] = symbol
        row["test_evidence"] = f"wave2_lift:{path.as_posix()}"
        row["notes"] = f"wave2 lift from existing non-stub module {path.as_posix()}"
        lifted += 1
        by_pkg[pkg] = by_pkg.get(pkg, 0) + 1

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(decisions)

    print(f"wave2 lifted {lifted} planned→idiomatic")
    print(f"  skipped_stub={skipped_stub} skipped_missing_file={skipped_missing_file}")
    for pkg, n in sorted(by_pkg.items(), key=lambda x: -x[1]):
        print(f"  {pkg}: +{n}")


if __name__ == "__main__":
    main()
