#!/usr/bin/env python3
"""Scan hitool-rs for Hutool test alignment and build test-decisions.csv.

Coverage evidence (any one is enough for status=covered):
1. Explicit comment: 对齐 Java: `StrUtilTest.isBlankTest()`
2. Class-level header 对齐: `cn.hutool...StrUtilTest` plus a #[test] fn whose
   snake_case name maps to a Java camelCase method on that class
3. Filename heuristic: `str_util_parity.rs` → StrUtilTest + name mapping

Statuses: covered | ignored | planned | unresolved

Behavioral gate: `covered` means a runnable test with real asserts.
`#[ignore]` stubs AND soft stubs (`assert!(true)`, empty smoke) count as `ignored`.
"""

from __future__ import annotations

import argparse
import csv
import re
from collections import defaultdict
from pathlib import Path


METHOD_REF = re.compile(
    r"对齐\s*Java\s*[:：]\s*`?((?:[\w.]+\.)?(\w+Test)\.(\w+)\s*(?:\(\s*\))?)`?"
)
# parity_get!/parity_download!/soap_test! carry the Hutool id in a string literal.
MACRO_PARITY = re.compile(
    r"(?:parity_get|parity_download|soap_test)!\s*\(\s*(\w+)\s*,\s*"
    r'"((?:[\w.]+\.)?(\w+Test)\.(\w+)\s*(?:\(\s*\))?)'
)
# `cn.hutool.json.JSONUtilTest` or bare `JSONUtilTest` or `hutool-json JSONUtilTest`
CLASS_REF = re.compile(
    r"对齐\s*[:：]\s*`?((?:[\w.]+\.)+(\w+Test)|(\w+Test))`?"
)
CLASS_REF_LOOSE = re.compile(r"对齐[^`\n]*\b(\w+Test)\b")
TEST_ATTR = re.compile(r"#\[(?:tokio::)?test\]")
IGNORE_ATTR = re.compile(r"#\[ignore(?:\s*[=\(]|])")
# Attribute-only: do not treat `#[ignore]` inside // / //! / /// comments as real ignores.
FN_DEF = re.compile(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)")
PENDING = re.compile(r"待实现|unimplemented!|PendingEngine")
# Soft "covered" stubs that do not prove Hutool I/O equivalence.
SOFT_ASSERT = re.compile(r"assert!\(\s*true\s*\)")
# #[ignore] stubs accepted as deferred (not runnable parity) → status=planned.
PLANNED_IGNORE = re.compile(
    r"planned:|@Disabled|搁置|deferred|无对等运行时反射|"
    r"等待\s*(?:ReflectUtil|ClassUtil|ModifierUtil|ReferenceUtil|"
    r"ClassLoaderUtil|JndiUtil|JaxbUtil|TypeUtil|XmlUtil)"
)
FILE_HINT = re.compile(r"^([a-z0-9_]+)_parity\.rs$")


def is_ignore_attr_line(line: str) -> bool:
    """Return True only when the line is a Rust `#[ignore]` attribute (not a comment)."""
    stripped = line.strip()
    if not stripped or stripped.startswith("//") or stripped.startswith("/*"):
        return False
    return bool(IGNORE_ATTR.match(stripped))

# Filename stem → preferred Hutool *Test simple class name(s)
FILE_CLASS_MAP: dict[str, list[str]] = {
    "json_parity": ["JSONUtilTest"],
    "json_util_parity": ["JSONUtilTest"],
    "json_array_parity": ["JSONArrayTest"],
    "json_object_parity": ["JSONObjectTest"],
    "json_extended_parity": ["JSONUtilTest", "JSONObjectTest", "JSONArrayTest"],
    "json_issue_parity": ["JSONUtilTest"],
    "json_expansion": ["JSONUtilTest", "JSONObjectTest", "JSONArrayTest"],
    "json_expansion_2": ["JSONUtilTest", "JSONObjectTest", "JSONArrayTest"],
    "cache_parity": [
        "CacheTest",
        "LRUCacheTest",
        "WeakCacheTest",
        "FileCacheTest",
        "Issue3618Test",
        "IssueI8MEIXTest",
        "CacheConcurrentTest",
    ],
}


def snake_to_camel(name: str) -> str:
    parts = name.split("_")
    if not parts:
        return name
    return parts[0] + "".join(p[:1].upper() + p[1:] for p in parts[1:] if p)


def rust_fn_to_java_methods(fn_name: str) -> list[str]:
    """Generate candidate Java test method names from a Rust #[test] fn."""
    candidates = [snake_to_camel(fn_name)]
    # is_blank_test → isBlankTest (already), also try without trailing Test duplication
    if fn_name.endswith("_test"):
        base = fn_name[: -len("_test")]
        candidates.append(snake_to_camel(base) + "Test")
        candidates.append(snake_to_camel(base))
    # numbered: to_json_str_test_2 → toJsonStrTest2 ; split_test_2 → splitTest2
    m = re.match(r"^(.+)_test_(\d+)$", fn_name)
    if m:
        candidates.append(snake_to_camel(m.group(1)) + "Test" + m.group(2))
        candidates.append(snake_to_camel(m.group(1) + "_" + m.group(2)) + "Test")
        candidates.append(snake_to_camel(m.group(1)) + m.group(2) + "Test")
    # issue_3540_test → issue3540Test
    m = re.match(r"^issue_(\w+)_test$", fn_name)
    if m:
        candidates.append("issue" + m.group(1).replace("_", "") + "Test")
        candidates.append("Issue" + m.group(1).replace("_", "") + "Test")
    # dedupe preserve order
    out: list[str] = []
    for c in candidates:
        if c and c not in out:
            out.append(c)
    return out


def load_inventory(path: Path) -> dict:
    with path.open(encoding="utf-8", newline="") as stream:
        rows = list(csv.DictReader(stream))
    by_simple: dict[str, list[dict[str, str]]] = defaultdict(list)
    by_class_method: dict[str, dict[str, str]] = {}
    classes: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        simple_class = row["class_name"].rsplit(".", 1)[-1]
        by_simple[f"{simple_class}.{row['method_name']}"].append(row)
        by_simple[f"{row['class_name']}.{row['method_name']}"].append(row)
        by_class_method[f"{simple_class}::{row['method_name']}"] = row
        by_class_method[f"{row['class_name']}::{row['method_name']}"] = row
        classes[simple_class].append(row)
        classes[row["class_name"]].append(row)
    return {
        "rows": rows,
        "by_simple": by_simple,
        "by_class_method": by_class_method,
        "classes": classes,
    }


def resolve_ids(class_key: str, method: str, inv: dict) -> list[str]:
    found: list[str] = []
    for key in (
        f"{class_key}.{method}",
        f"{class_key.rsplit('.', 1)[-1]}.{method}",
    ):
        for row in inv["by_simple"].get(key, []):
            if row["test_id"] not in found:
                found.append(row["test_id"])
    return found


def file_hint_classes(path: Path) -> list[str]:
    stem = path.stem
    if stem in FILE_CLASS_MAP:
        return list(FILE_CLASS_MAP[stem])
    m = FILE_HINT.match(path.name)
    if not m:
        return []
    # str_util → StrUtilTest
    camel = snake_to_camel(m.group(1))
    camel = camel[:1].upper() + camel[1:]
    return [camel + "Test"]


def scan_file(path: Path, repo: Path, inv: dict) -> list[dict[str, str]]:
    text = path.read_text(encoding="utf-8", errors="ignore")
    lines = text.splitlines()
    rel = str(path.relative_to(repo)).replace("\\", "/")
    decisions: list[dict[str, str]] = []

    # Collect class targets from headers + filename
    class_targets: list[str] = []
    for line in lines[:80]:
        m = CLASS_REF.search(line)
        if m:
            fq = m.group(1)
            simple = m.group(2) or m.group(3)
            if fq and "." in fq:
                class_targets.append(fq)
            if simple:
                class_targets.append(simple)
        else:
            m2 = CLASS_REF_LOOSE.search(line)
            if m2:
                class_targets.append(m2.group(1))
    class_targets.extend(file_hint_classes(path))
    # unique
    seen_c: list[str] = []
    for c in class_targets:
        if c not in seen_c:
            seen_c.append(c)
    class_targets = seen_c

    # Map #[test] fns; track whether #[ignore] applies (Hutool parity must be runnable).
    # (fn_line, fn_name, ignored)
    tests: list[tuple[int, str, bool]] = []
    pending: int | None = None
    pending_ignored = False
    for idx, line in enumerate(lines):
        if is_ignore_attr_line(line):
            # Attribute may sit immediately above #[test]
            pending_ignored = True
        if TEST_ATTR.search(line):
            ignored = pending_ignored or is_ignore_attr_line(line)
            # Also look a few lines above for #[ignore] attributes only
            if not ignored:
                for back in range(1, 6):
                    if idx - back < 0:
                        break
                    prev = lines[idx - back]
                    if is_ignore_attr_line(prev):
                        ignored = True
                        break
                    if prev.strip() and not prev.strip().startswith("#"):
                        # hit a non-attribute code line
                        if not prev.strip().startswith("///") and not prev.strip().startswith(
                            "//"
                        ):
                            break
            m_same = FN_DEF.search(line)
            if m_same:
                tests.append((idx, m_same.group(1), ignored))
                pending = None
                pending_ignored = False
            else:
                pending = idx
                pending_ignored = ignored
            continue
        if pending is not None:
            m = FN_DEF.match(line)
            if m:
                tests.append((idx, m.group(1), pending_ignored))
                pending = None
                pending_ignored = False

    def status_for_window(window: str, ignored: bool) -> str:
        if ignored:
            # Deferred / JVM-only / Java @Disabled → planned (registered, not behavioral)
            if PLANNED_IGNORE.search(window):
                return "planned"
            return "ignored"
        if PENDING.search(window):
            return "planned"
        # assert!(true) is registration theater — not result-equivalent to Hutool.
        if SOFT_ASSERT.search(window):
            return "ignored"
        return "covered"

    def record_alignment(
        full: str,
        class_simple: str,
        method: str,
        rust_fn: str,
        rust_ignored: bool,
        window: str,
        source: str,
    ) -> None:
        class_part = class_simple
        if "." in full:
            class_part = full.rsplit(".", 1)[0]
        status = status_for_window(window, rust_ignored)
        evidence = f"{rel}::{rust_fn}" if rust_fn else rel
        ids = resolve_ids(class_part, method, inv)
        if not ids:
            decisions.append(
                {
                    "test_id": f"UNRESOLVED::{class_simple}::{method}",
                    "status": "unresolved",
                    "hitool_test": evidence,
                    "notes": f"aligned from {source}: {full}",
                }
            )
            return
        for tid in ids:
            decisions.append(
                {
                    "test_id": tid,
                    "status": status,
                    "hitool_test": evidence,
                    "notes": f"aligned from {source}: {full}",
                }
            )

    test_by_name = {name: (line, ign) for line, name, ign in tests}

    # Explicit method comments
    for idx, line in enumerate(lines):
        m = METHOD_REF.search(line)
        if not m:
            continue
        full, class_simple, method = m.group(1), m.group(2), m.group(3)
        rust_fn = ""
        rust_ignored = False
        for look in range(idx, min(idx + 30, len(lines))):
            for tline, tname, tign in tests:
                if tline == look:
                    rust_fn = tname
                    rust_ignored = tign
                    break
            if rust_fn:
                break
        if not rust_fn:
            for tline, tname, tign in tests:
                if tline >= idx:
                    rust_fn = tname
                    rust_ignored = tign
                    break
        window = "\n".join(lines[max(0, idx - 2) : min(len(lines), idx + 25)])
        record_alignment(full, class_simple, method, rust_fn, rust_ignored, window, "comment")

    # Macro-generated parity tests (http_network_parity.rs)
    for idx, line in enumerate(lines):
        m = MACRO_PARITY.search(line)
        if not m:
            continue
        rust_fn = m.group(1)
        full, class_simple, method = m.group(2), m.group(3), m.group(4)
        rust_ignored = test_by_name.get(rust_fn, (idx, False))[1]
        window = "\n".join(lines[max(0, idx - 2) : min(len(lines), idx + 25)])
        record_alignment(full, class_simple, method, rust_fn, rust_ignored, window, "macro")

    # Name-based mapping for class-targeted files
    if class_targets and ("/tests/" in rel or path.name.endswith("_parity.rs")):
        for tline, fn_name, tign in tests:
            methods = rust_fn_to_java_methods(fn_name)
            matched = False
            window = "\n".join(lines[max(0, tline - 5) : min(len(lines), tline + 15)])
            status = status_for_window(window, tign)
            for class_key in class_targets:
                for method in methods:
                    ids = resolve_ids(class_key, method, inv)
                    if not ids:
                        continue
                    matched = True
                    for tid in ids:
                        decisions.append(
                            {
                                "test_id": tid,
                                "status": status,
                                "hitool_test": f"{rel}::{fn_name}",
                                "notes": (
                                    f"name-mapped {fn_name} → {class_key}.{method}"
                                ),
                            }
                        )
                    break
                if matched:
                    break

    return decisions


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--inventory", type=Path, default=None)
    parser.add_argument("--output", type=Path, default=None)
    args = parser.parse_args()
    repo = args.repo.resolve()
    inventory_path = args.inventory or repo / "parity" / "hutool-v5.8.46-tests.csv"
    output = args.output or repo / "parity" / "test-decisions.csv"

    inv = load_inventory(inventory_path)
    decisions: list[dict[str, str]] = []
    for path in sorted((repo / "crates").rglob("*.rs")):
        decisions.extend(scan_file(path, repo, inv))

    # Prefer a runnable covered test over an ignored stub for the same id.
    ranked = {"covered": 4, "ignored": 3, "planned": 2, "unresolved": 1}
    best: dict[str, dict[str, str]] = {}
    for row in decisions:
        tid = row["test_id"]
        prev = best.get(tid)
        if prev is None or ranked.get(row["status"], 0) > ranked.get(prev["status"], 0):
            best[tid] = row

    rows = [best[k] for k in sorted(best)]
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(
            stream, fieldnames=["test_id", "status", "hitool_test", "notes"]
        )
        writer.writeheader()
        writer.writerows(rows)

    covered = sum(1 for r in rows if r["status"] == "covered")
    ignored = sum(1 for r in rows if r["status"] == "ignored")
    planned = sum(1 for r in rows if r["status"] == "planned")
    unresolved = sum(1 for r in rows if r["status"] == "unresolved")
    print(
        f"Wrote {len(rows)} test decisions "
        f"(covered={covered}, ignored={ignored}, planned={planned}, "
        f"unresolved={unresolved}) → {output}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
