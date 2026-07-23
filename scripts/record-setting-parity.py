#!/usr/bin/env python3
"""Record reviewed Hutool setting APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("::", 1)[0]


def mapping(qualified_name: str) -> tuple[str, str, str]:
    name = family(qualified_name)
    symbol = f"hutool_setting::{name}"
    if name in {"GroupedMap", "GroupedSet"}:
        test = "crates/hutool-setting/src/grouped.rs::grouped_map_covers_ordered_map_semantics"
        note = "IndexMap-backed ordered groups and sets retain Hutool group, mutation, iteration, parsing, and reload behavior."
    elif name in {"Props", "PropsUtil"}:
        test = "crates/hutool-setting/src/props.rs::properties_parse_convert_mutate_and_round_trip"
        note = "Java-properties syntax, escapes, typed getters, bean conversion, deterministic storage, lookup, and system snapshots are covered by the Props facade."
    elif name in {"Profile", "GlobalProfile"}:
        test = "crates/hutool-setting/src/profile.rs::owned_profile_resolves_extensions_and_cache"
        note = "Owned profiles provide explicit roots and caches; the opt-in compatibility global is resettable and isolated from the primary API."
    elif name == "YamlUtil":
        test = "crates/hutool-setting/src/yaml.rs::yaml_loads_and_dumps_dynamic_and_typed_values"
        note = "Serde YAML provides typed and dynamic path, reader, string, writer, and dump operations with typed errors."
    else:
        test = "crates/hutool-setting/src/setting.rs::setting_loads_variables_groups_mutations_and_storage"
        note = "The Setting facade and loader preserve grouped syntax, variables, typed conversion, storage, profile lookup, and explicitly owned injectable file watching."
    return symbol, test, note


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-setting":
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
    if selected != 225:
        raise SystemExit(f"expected 225 reviewed setting APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool setting APIs")


if __name__ == "__main__":
    main()
