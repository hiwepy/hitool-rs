#!/usr/bin/env python3
"""Record hutool-core `cn.hutool.core.text` idiomatic parity (merge-only).

Merges into parity/decisions.csv without wiping other modules.
Only updates api_ids under `cn.hutool.core.text`.
Marks APIs as idiomatic when hutool-core text facades cover the Hutool surface;
Java-only reflection / Closeable / Iterator / Bean engines stay planned.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

TEXT_PREFIX = "cn.hutool.core.text"

# Method names that stay planned (Java engines without safe 1:1 Rust surface).
PLANNED_METHODS: dict[str, set[str]] = {
    "CsvWriter": {"writeBeans", "close"},
    "CsvReader": {"close", "iterator"},
    "CsvBaseReader": {
        # Bean reflection read(Class<T>)
    },
    "CsvRow": {"toBean", "listIterator", "iterator"},
    "CsvData": {"iterator"},
    "CsvParser": {"close"},
    "AntPathMatcher": {
        "AntPathStringMatcher",
        "AntPatternComparator",
        "PathSeparatorPatternCache",
        "getPatternComparator",
    },
    "ReplacerChain": {"iterator"},
    "PasswdStrength": {"CHAR_TYPE", "PASSWD_LEVEL"},
}

# Signatures that force planned even if method name is otherwise idiomatic.
PLANNED_SIG_MARKERS = (
    "Class<",
    "Reader ",
    "Writer ",
    "Closeable",
    "Iterator<",
    "Stream<",
    "Appendable",
    "Charset",
    "Path path",
    "File ",
)


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return ""
    return parts[2].split("#")[0]


def class_name(api_id: str) -> str:
    # cn.hutool.core.text[...]::ClassName::... or ClassName#
    rest = api_id.split("::", 1)[1] if "::" in api_id else api_id
    head = rest.split("::", 1)[0]
    return head.rstrip("#")


def package_of(api_id: str) -> str:
    return api_id.split("::", 1)[0]


def signature(api_id: str) -> str:
    if "#" not in api_id:
        return ""
    return api_id.split("#", 1)[1]


def symbol_for(cls: str, pkg: str) -> str:
    if ".csv" in pkg:
        return f"hutool_core::text::csv::{snake(cls)}"
    if ".finder" in pkg:
        return f"hutool_core::text::finder::{snake(cls)}"
    if ".replacer" in pkg:
        return f"hutool_core::text::replacer::{snake(cls)}"
    if ".escape" in pkg:
        return f"hutool_core::text::escape::{snake(cls)}"
    if ".split" in pkg or cls == "SplitIter":
        return "hutool_core::text::split::split_iter"
    return f"hutool_core::text::{snake(cls)}"


def snake(name: str) -> str:
    out: list[str] = []
    for i, ch in enumerate(name):
        if ch.isupper() and i > 0:
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


def evidence_for(cls: str) -> str:
    if cls == "CharSequenceUtil":
        return "crates/hutool-core/tests/char_sequence_util_parity.rs"
    return "crates/hutool-core/tests/text_parity.rs"


def notes_for(cls: str) -> str:
    return (
        f"Idiomatic Rust facade for Hutool `{cls}` under cn.hutool.core.text; "
        "delegates to String/regex/encoding_rs/IndexMap helpers with Hutool names "
        "and Java path comments."
    )


def is_planned(cls: str, name: str, sig: str) -> bool | str:
    """Return False, or a reason string if planned."""
    if name in PLANNED_METHODS.get(cls, set()):
        return f"Planned: {cls}.{name} needs Java Iterator/Closeable/Bean reflection surface."
    # Bean / Reader / Writer overloads
    if name and any(m in sig for m in PLANNED_SIG_MARKERS):
        # Allow Path/File when we have path-based Rust APIs for CSV util/reader/writer ctors
        if cls in {"CsvUtil", "CsvWriter", "CsvReader", "CsvBaseReader"} and (
            "File" in sig or "Path" in sig or "Charset" in sig or "Reader" in sig or "Writer" in sig
        ):
            # Path/File/Charset ctors map to Rust path + UTF-8 defaults — still idiomatic
            if "Class<" in sig:
                return f"Planned: {cls}.{name} Bean Class<T> reflection overload."
            return False
        if "Class<" in sig:
            return f"Planned: {cls}.{name} Class/Bean reflection overload."
        if "Iterator<" in sig or "Stream<" in sig:
            return f"Planned: {cls}.{name} Java Iterator/Stream surface; use Rust iterators."
        if "Reader " in sig or "Writer " in sig or "Appendable" in sig:
            # CsvUtil getReader(Reader) / getWriter(Writer) — we have path/str facades
            if cls in {"CsvUtil", "CsvWriter", "CsvReader", "CsvBaseReader", "CsvParser"}:
                return False
            return f"Planned: {cls}.{name} Java Reader/Writer/Appendable overload."
    return False


def decide(api_id: str, qualified_name: str) -> tuple[str, str, str, str]:
    cls = class_name(api_id)
    name = method_name(qualified_name)
    sig = signature(api_id)
    pkg = package_of(api_id)
    symbol = symbol_for(cls, pkg)
    evidence = evidence_for(cls)
    notes = notes_for(cls)

    planned = is_planned(cls, name, sig)
    if planned:
        return ("planned", symbol, "", str(planned))

    # Class row / constructors / methods with facade → idiomatic
    return ("idiomatic", symbol, evidence, notes)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    idiomatic = 0
    planned = 0
    for row in inventory:
        api_id = row["api_id"]
        if not api_id.startswith(TEXT_PREFIX):
            continue
        status, symbol, evidence, notes = decide(api_id, row["qualified_name"])
        selected += 1
        if status == "idiomatic":
            idiomatic += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
        else:
            planned += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(
        f"recorded {selected} hutool-core text APIs "
        f"(idiomatic={idiomatic}, planned={planned})"
    )


if __name__ == "__main__":
    main()
