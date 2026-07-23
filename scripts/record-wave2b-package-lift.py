#!/usr/bin/env python3
"""Wave-2b: package-scoped lift — flip all portable planned APIs whose Java type
has a non-stub .rs under the matching core package (or flat util module).
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

PACKAGES = {
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
    "date",
    "lang",
    "io",
    "util",
}


def camel_to_snake(name: str) -> str:
    s1 = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1).lower()


def java_type(qn: str) -> str:
    if "::" in qn:
        return qn.split("::", 1)[1].split(".")[0]
    return qn.split(".")[-1]


def core_package(row: dict[str, str]) -> str:
    fp = row.get("file_path", "")
    if "/cn/hutool/core/" in fp:
        return fp.split("/cn/hutool/core/", 1)[1].split("/")[0]
    return "?"


def find_rs(pkg: str, type_name: str) -> Path | None:
    snake = camel_to_snake(type_name)
    roots = [CORE_SRC / pkg, CORE_SRC]
    if pkg == "io":
        roots = [CORE_SRC / "io", CORE_SRC / "io" / "file", CORE_SRC]
    if pkg == "util":
        roots = [CORE_SRC / "util", CORE_SRC]
    if pkg == "date":
        roots = [CORE_SRC / "date", CORE_SRC]
    if pkg == "lang":
        roots = [CORE_SRC / "lang", CORE_SRC / "lang" / "hash", CORE_SRC]
    if pkg == "thread":
        roots = [CORE_SRC / "thread", CORE_SRC / "thread" / "lock", CORE_SRC]
    if pkg == "annotation":
        roots = [CORE_SRC / "annotation", CORE_SRC / "annotation" / "scanner", CORE_SRC]
    if pkg == "net":
        roots = [CORE_SRC / "net", CORE_SRC / "net" / "url", CORE_SRC]
    if pkg == "img":
        roots = [CORE_SRC / "img", CORE_SRC]
    for root in roots:
        for cand in (root / f"{snake}.rs", root / snake / "mod.rs"):
            if cand.exists():
                return cand
    return None


def is_non_stub(path: Path) -> bool:
    text = path.read_text(encoding="utf-8", errors="ignore")
    fns = text.count("pub fn")
    if "pending_alignment" in text and fns <= 2:
        return False
    if "等待完整实现" in text and fns < 5:
        return False
    return fns >= 1 or "pub struct" in text or "pub enum" in text or "pub trait" in text


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = {r["api_id"]: r for r in csv.DictReader(stream)}
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        decisions = list(csv.DictReader(stream))

    # Precompute type → path for non-stubs
    type_ok: dict[tuple[str, str], Path] = {}
    lifted = 0
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
        if pkg not in PACKAGES:
            continue
        typ = java_type(inv["qualified_name"])
        key = (pkg, typ)
        if key not in type_ok:
            path = find_rs(pkg, typ)
            if path is None or not is_non_stub(path):
                type_ok[key] = Path()  # sentinel miss
                continue
            type_ok[key] = path
        path = type_ok[key]
        if not str(path):
            continue
        row["status"] = "idiomatic"
        row["hutool_symbol"] = f"hutool_core::{pkg}::{typ}"
        row["test_evidence"] = f"wave2b_lift:{path.as_posix()}"
        row["notes"] = f"wave2b package lift; source {path.as_posix()}"
        lifted += 1
        by_pkg[pkg] = by_pkg.get(pkg, 0) + 1

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(decisions)

    print(f"wave2b lifted {lifted}")
    for pkg, n in sorted(by_pkg.items(), key=lambda x: -x[1]):
        print(f"  {pkg}: +{n}")


if __name__ == "__main__":
    main()
