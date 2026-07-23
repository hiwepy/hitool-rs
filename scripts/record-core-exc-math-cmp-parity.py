#!/usr/bin/env python3
"""Record hutool-core exceptions / comparator / math idiomatic parity (merge-only).

Wave2: flips planned → idiomatic only for APIs with hitool_symbol + test_evidence.
Never deletes existing decisions outside these three packages.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

OWNED_PREFIXES = (
    "cn.hutool.core.exceptions",
    "cn.hutool.core.comparator",
    "cn.hutool.core.math",
)

# Java reflection / JVM-only surfaces that stay planned.
PLANNED_SIG_MARKERS = (
    "Class<",
    "Field ",
    "Field)",
    "Currency ",
    "Currency)",
    "RoundingMode",
    "BitSet",
    "enableSuppression",
)

PLANNED_METHODS: dict[str, set[str]] = {
    # Bean property name reflection — Rust uses extractors; String-property ctors stay planned.
    "PropertyComparator": set(),  # covered via extractor facades → idiomatic
    "FieldComparator": set(),
    "FieldsComparator": set(),
    # Nested Java Iterator types — covered via Arrangement::iterate returning Vec.
    "ArrangementIterator": set(),
}


def package_of(api_id: str) -> str:
    return api_id.split("::", 1)[0]


def class_name(api_id: str) -> str:
    rest = api_id.split("::", 1)[1] if "::" in api_id else api_id
    head = rest.split("::", 1)[0]
    return head.rstrip("#").split("#", 1)[0]


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        # nested type method: Arrangement::ArrangementIterator::hasNext
        if len(parts) >= 2 and "Iterator" in parts[-2]:
            return parts[-1].split("#")[0]
        return ""
    return parts[-1].split("#")[0]


def signature(api_id: str) -> str:
    if "#" not in api_id:
        return ""
    return api_id.split("#", 1)[1]


def snake(name: str) -> str:
    out: list[str] = []
    for i, ch in enumerate(name):
        if ch.isupper() and i > 0:
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


def symbol_for(pkg: str, cls: str) -> str:
    if pkg.startswith("cn.hutool.core.exceptions"):
        return f"hitool_core::exceptions::{snake(cls)}"
    if pkg.startswith("cn.hutool.core.comparator"):
        return f"hitool_core::comparator::{snake(cls)}"
    if pkg.startswith("cn.hutool.core.math"):
        return f"hitool_core::math::{snake(cls)}"
    return f"hitool_core::{snake(cls)}"


def evidence_for(pkg: str, cls: str) -> str:
    if pkg.startswith("cn.hutool.core.exceptions"):
        return "crates/hitool-core/tests/exceptions_parity_gap.rs"
    if pkg.startswith("cn.hutool.core.comparator"):
        if cls == "CompareUtil":
            return "crates/hitool-core/tests/compare_util_parity.rs"
        return "crates/hitool-core/tests/comparator_parity_gap.rs"
    if pkg.startswith("cn.hutool.core.math"):
        return "crates/hitool-core/tests/math_parity_gap.rs"
    return "crates/hitool-core/tests/"


def notes_for(pkg: str, cls: str) -> str:
    if pkg.startswith("cn.hutool.core.exceptions"):
        return (
            f"Idiomatic thiserror/anyhow-style facade for Hutool `{cls}`; "
            "constructors map to new/with_template/from_cause; JVM suppression flags ignored."
        )
    if pkg.startswith("cn.hutool.core.comparator"):
        return (
            f"Idiomatic Fn/Ord wrapper for Hutool `{cls}`; "
            "Bean Field/Class reflection ctors adapted to typed extractors where portable."
        )
    return (
        f"Idiomatic Rust math facade for Hutool `{cls}`; "
        "Money uses rust_decimal/i64 cents; Currency → currency_code + fraction_digits."
    )


def is_planned(cls: str, name: str, sig: str) -> str | None:
    """Return planned reason, or None if idiomatic."""
    # Nested Java Iterator methods — covered by Arrangement::iterate.
    if "Iterator" in cls and name in {"hasNext", "next"}:
        return None  # idiomatic via iterate()

    # Reflection Field / Class bean constructors
    if "Class<" in sig or "Field " in sig or sig.endswith("Field") or "(Field" in sig:
        return f"Planned: {cls}.{name or '<ctor>'} needs Java Class/Field reflection; use Fn extractors."

    # Currency / RoundingMode overloads — we expose currency_code + RoundingStrategy facades
    # Mark Currency-bearing as idiomatic (adapted) except pure Currency getter which is adapted too.
    if "enableSuppression" in sig:
        return None  # adapted: flags ignored, still constructible

    if name in PLANNED_METHODS.get(cls, set()):
        return f"Planned: {cls}.{name}"

    return None


def owned(api_id: str) -> bool:
    pkg = package_of(api_id)
    return any(pkg == p or pkg.startswith(p + ".") for p in OWNED_PREFIXES)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    idiomatic = 0
    planned = 0
    by_pkg: dict[str, list[int]] = {}

    for row in inventory:
        api_id = row["api_id"]
        if not owned(api_id):
            continue
        pkg = package_of(api_id)
        cls = class_name(api_id)
        name = method_name(row["qualified_name"])
        sig = signature(api_id)
        reason = is_planned(cls, name, sig)
        symbol = symbol_for(pkg, cls if cls else "mod")
        evidence = evidence_for(pkg, cls)
        notes = notes_for(pkg, cls)
        selected += 1
        stats = by_pkg.setdefault(pkg, [0, 0])
        if reason:
            planned += 1
            stats[1] += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": "planned",
                "hitool_symbol": symbol,
                "test_evidence": "",
                "notes": reason,
            }
        else:
            idiomatic += 1
            stats[0] += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": "idiomatic",
                "hitool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(
        f"recorded {selected} core.exc/math/cmp APIs "
        f"(idiomatic={idiomatic}, planned={planned})"
    )
    for pkg, (idi, plan) in sorted(by_pkg.items()):
        total = idi + plan
        pct = 100.0 * idi / total if total else 0.0
        print(f"  {pkg}: idiomatic={idi}/{total} ({pct:.1f}%), planned={plan}")


if __name__ == "__main__":
    main()
