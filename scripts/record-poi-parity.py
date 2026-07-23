#!/usr/bin/env python3
"""Record Hutool poi APIs as planned signature-aligned stubs (no POI engine)."""

from __future__ import annotations

import csv
import re
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
MODULE = "hutool-poi"
EXPECTED = 555
ROOT = Path("crates/hitool-poi/src")

ENGINE_NOTE = (
    "Signature-aligned empty implementation; waiting for easyexcel-rs / "
    "easydoc-rs / easyofd-rs / easypdf-rs. Do not treat as a POI engine."
)


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("#", 1)[0].split("::", 1)[0]


def path_score(path: str) -> int:
    score = 0
    if "/excel_in/" in path:
        score -= 5
    if "/excel/" in path:
        score += 3
    if path.endswith("csv_util.rs"):
        score -= 4
    if "/word_impl/" in path:
        score += 1
    return score


def build_class_paths() -> dict[str, str]:
    """Map Hutool simple class name → best stub source file under hitool-poi."""
    found: dict[str, str] = {}
    for path in ROOT.rglob("*.rs"):
        text = path.read_text(encoding="utf-8", errors="ignore")
        rel = str(path).replace("\\", "/")
        names: set[str] = set()
        for match in re.finditer(
            r"对齐(?: Java)?: `cn\.hutool\.poi[\w.]*\.(\w+)`", text
        ):
            names.add(match.group(1))
        for match in re.finditer(r"pub (?:struct|enum|type|trait)\s+(\w+)", text):
            names.add(match.group(1))
        for name in names:
            prev = found.get(name)
            if prev is None or path_score(rel) > path_score(prev):
                found[name] = rel
    # Trait aliases / inventory-only names.
    found.setdefault("SheetReader", found.get("AbstractSheetReader", str(ROOT / "excel" / "reader.rs")))
    found.setdefault("CellEditor", found.get("CellEditorDispatch", str(ROOT / "excel" / "cell" / "mod.rs")))
    found.setdefault("CellHandler", found.get("CellHandlerDispatch", str(ROOT / "excel" / "cell" / "mod.rs")))
    found.setdefault("CellSetter", str(ROOT / "excel" / "cell" / "setters.rs"))
    found.setdefault("CellValue", str(ROOT / "excel" / "cell" / "mod.rs"))
    found.setdefault("ExcelSaxReader", found.get("Excel03SaxReader", str(ROOT / "excel" / "sax")))
    found.setdefault("POIException", str(ROOT / "error.rs"))
    return found


def to_symbol(class_name: str, file_path: str) -> str:
    rel = file_path.split("crates/hitool-poi/src/")[-1]
    if rel.endswith(".rs"):
        rel = rel[:-3]
    modules = "::".join(part for part in rel.split("/") if part)
    return f"hitool_poi::{modules}::{class_name}"


def engine_for(class_name: str, qualified_name: str) -> str:
    pkg = qualified_name.split("::", 1)[0]
    if ".ofd" in pkg or class_name.startswith("Ofd"):
        return "easyofd-rs"
    if ".word" in pkg or class_name.startswith("Word") or class_name == "DocUtil":
        return "easydoc-rs"
    if "pdf" in class_name.lower():
        return "easypdf-rs"
    return "easyexcel-rs"


def main() -> None:
    class_paths = build_class_paths()
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != MODULE:
            continue
        selected += 1
        class_name = family(row["qualified_name"])
        file_path = class_paths.get(
            class_name, "crates/hitool-poi/src/excel/excel_util.rs"
        )
        engine = engine_for(class_name, row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "planned",
            "hitool_symbol": to_symbol(class_name, file_path),
            "test_evidence": "crates/hitool-poi/tests/poi_parity.rs::read_test",
            "notes": (
                f"{ENGINE_NOTE} Stub path for {class_name}; engine={engine}."
            ),
        }

    if selected != EXPECTED:
        raise SystemExit(f"expected {EXPECTED} {MODULE} APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} {MODULE} APIs (all planned signature stubs)")


if __name__ == "__main__":
    main()
