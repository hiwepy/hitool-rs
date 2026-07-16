#!/usr/bin/env python3
"""Record reviewed Hutool DFA APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "FoundWord": (
        "hitool_dfa::FoundWord",
        "crates/hitool-dfa/src/word_tree.rs::found_word_exposes_effective_and_source_forms",
    ),
    "SensitiveProcessor": (
        "hitool_dfa::{SensitiveProcessor,DefaultSensitiveProcessor}",
        "crates/hitool-dfa/src/sensitive.rs::filtering_supports_default_custom_and_serialized_processors",
    ),
    "SensitiveUtil": (
        "hitool_dfa::SensitiveUtil",
        "crates/hitool-dfa/src/sensitive.rs::initialization_search_and_async_replacement_are_thread_safe",
    ),
    "StopChar": (
        "hitool_dfa::StopChar",
        "crates/hitool-dfa/src/stop_char.rs::classifies_whitespace_hutool_symbols_and_normal_text",
    ),
    "WordTree": (
        "hitool_dfa::{WordTree,MatchOptions}",
        "crates/hitool-dfa/src/word_tree.rs::density_and_greed_modes_match_hutool_ordering",
    ),
}


def family(qualified_name: str) -> str | None:
    if not qualified_name.startswith("cn.hutool.dfa::"):
        return None
    candidate = qualified_name.removeprefix("cn.hutool.dfa::").split("::", 1)[0]
    return candidate if candidate in FAMILIES else None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-dfa":
            continue
        dfa_family = family(row["qualified_name"])
        if dfa_family is None:
            raise SystemExit(f"unmapped Hutool DFA family: {row['qualified_name']}")
        selected += 1
        symbol, test = FAMILIES[dfa_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": (
                "Aho-Corasick remains the immutable high-throughput engine; the mutable Rust "
                "trie preserves Hutool stop-character, density, greed, span, and filtering semantics."
            ),
        }

    if selected != 43:
        raise SystemExit(f"expected 43 reviewed DFA APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool DFA APIs")


if __name__ == "__main__":
    main()
