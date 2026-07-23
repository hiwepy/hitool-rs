#!/usr/bin/env python3
"""Generate Hutool @Test / @ParameterizedTest inventory from a local checkout.

Output CSV columns:
  test_id,module,class_name,method_name,file_path,start_line,kind

test_id format: <fqcn>::<method_name>#<occurrence>
"""

from __future__ import annotations

import argparse
import csv
import re
from pathlib import Path


ANNOTATION = re.compile(r"@(?:Test|ParameterizedTest)\b")
# Capture method name after annotation blocks / modifiers.
# JUnit 5 allows package-private methods (`void fooTest()` without public).
METHOD = re.compile(
    r"(?:(?:public|protected|private)\s+)?(?:static\s+)?(?:final\s+)?"
    r"(?:void|[\w.<>,\[\]\s]+)\s+(\w+)\s*\("
)
DISABLED = re.compile(r"@Disabled\b")
PACKAGE = re.compile(r"^\s*package\s+([\w.]+)\s*;", re.M)
CLASS = re.compile(
    r"^\s*(?:public\s+)?(?:abstract\s+)?(?:final\s+)?class\s+(\w+)", re.M
)


def module_of(path: Path, hutool_root: Path) -> str:
    rel = path.relative_to(hutool_root)
    return rel.parts[0] if rel.parts else ""


def fqcn_of(path: Path, text: str) -> str:
    pkg = PACKAGE.search(text)
    cls = CLASS.search(text)
    if not pkg or not cls:
        return path.stem
    return f"{pkg.group(1)}.{cls.group(1)}"


def extract_tests(path: Path, hutool_root: Path) -> list[dict[str, str]]:
    text = path.read_text(encoding="utf-8", errors="ignore")
    lines = text.splitlines()
    fqcn = fqcn_of(path, text)
    module = module_of(path, hutool_root)
    rel = str(path.relative_to(hutool_root)).replace("\\", "/")
    rows: list[dict[str, str]] = []
    seen: dict[str, int] = {}

    i = 0
    while i < len(lines):
        line = lines[i]
        if not ANNOTATION.search(line):
            i += 1
            continue
        kind = "ParameterizedTest" if "ParameterizedTest" in line else "Test"
        # Scan forward a few lines for the method signature.
        method_name = None
        start_line = i + 1
        window_end = min(i + 20, len(lines))
        method_line = None
        for j in range(i, window_end):
            # Skip pure annotation / attribute lines.
            stripped = lines[j].strip()
            if stripped.startswith("@") or stripped.startswith("//"):
                continue
            m = METHOD.search(lines[j])
            if m:
                method_name = m.group(1)
                start_line = j + 1
                method_line = j
                break
        if method_name is None or method_line is None:
            i += 1
            continue
        # @Disabled only counts when attached to THIS test (between @Test and method).
        attached = "\n".join(lines[i : method_line + 1])
        if DISABLED.search(attached):
            kind = f"{kind}+Disabled"
        occ = seen.get(method_name, 0) + 1
        seen[method_name] = occ
        suffix = "" if occ == 1 else f"@{occ}"
        test_id = f"{fqcn}::{method_name}{suffix}"
        rows.append(
            {
                "test_id": test_id,
                "module": module,
                "class_name": fqcn,
                "method_name": method_name,
                "file_path": rel,
                "start_line": str(start_line),
                "kind": kind,
            }
        )
        i += 1
    return rows


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "hutool_root",
        type=Path,
        nargs="?",
        default=Path(__file__).resolve().parents[1].parent / "hutool",
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=Path(__file__).resolve().parents[1]
        / "parity"
        / "hutool-v5.8.46-tests.csv",
    )
    args = parser.parse_args()
    hutool_root = args.hutool_root.resolve()
    if not hutool_root.is_dir():
        print(f"Hutool checkout not found: {hutool_root}", flush=True)
        return 2

    files = sorted(hutool_root.glob("**/src/test/java/**/*Test.java"))
    rows: list[dict[str, str]] = []
    for path in files:
        rows.extend(extract_tests(path, hutool_root))

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=[
                "test_id",
                "module",
                "class_name",
                "method_name",
                "file_path",
                "start_line",
                "kind",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)

    print(
        f"Generated {len(rows)} Hutool test records "
        f"from {len(files)} *Test.java files at {args.output}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
