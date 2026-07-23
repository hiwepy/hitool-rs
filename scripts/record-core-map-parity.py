#!/usr/bin/env python3
"""Merge-only recorder for Hutool `cn.hutool.core.map` APIs.

Never deletes existing decisions rows. Only upserts map-package api_ids.
Reflection / WeakReference / SoftReference JVM semantics are `planned`.
"""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.map"
MULTI_ROOT = "cn.hutool.core.map.multi"
REF_ROOT = "cn.hutool.core.map.reference"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

# Families with executable idiomatic Rust evidence.
IDIOMATIC: dict[str, tuple[str, str, str]] = {
    "MapUtil": (
        "hitool_core::MapUtil",
        "crates/hitool-core/tests/map_util_parity.rs::map_util_creation_filter_join_flatten_and_builders",
        "Owned HashMap/IndexMap/BTreeMap facades cover Hutool MapUtil creation, get*, filter/edit/map, join, flatten, partition, and builders without Java reflection.",
    ),
    "MapBuilder": (
        "hitool_core::MapBuilder",
        "crates/hitool-core/tests/map_parity.rs::map_builder_conditional_put_and_join",
        "Fluent builder put/putAll/clear/join paths match Hutool MapBuilder ownership semantics.",
    ),
    "MapWrapper": (
        "hitool_core::MapWrapper",
        "crates/hitool-core/tests/map_parity.rs::map_wrapper_compute_merge_and_replace",
        "Thin HashMap wrapper exposes Hutool compute/merge/replace/forEach style mutators.",
    ),
    "BiMap": (
        "hitool_core::BiMap",
        "crates/hitool-core/tests/map_parity.rs::bi_map_inverse_and_compute",
        "Bidirectional index maintains forward/inverse consistency for put/remove/compute paths.",
    ),
    "CaseInsensitiveMap": (
        "hitool_core::CaseInsensitiveMap",
        "crates/hitool-core/tests/map_parity.rs::case_insensitive_family",
        "Lowercased string keys provide case-insensitive HashMap behavior.",
    ),
    "CaseInsensitiveLinkedMap": (
        "hitool_core::CaseInsensitiveLinkedMap",
        "crates/hitool-core/tests/map_parity.rs::case_insensitive_family",
        "IndexMap-backed case-insensitive map preserves insertion order.",
    ),
    "CaseInsensitiveTreeMap": (
        "hitool_core::CaseInsensitiveTreeMap",
        "crates/hitool-core/tests/map_parity.rs::case_insensitive_family",
        "BTreeMap-backed case-insensitive map preserves key order.",
    ),
    "CamelCaseMap": (
        "hitool_core::CamelCaseMap",
        "crates/hitool-core/tests/map_parity.rs::camel_case_family",
        "Underscore keys are normalized to camelCase on write/read.",
    ),
    "CamelCaseLinkedMap": (
        "hitool_core::CamelCaseLinkedMap",
        "crates/hitool-core/tests/map_parity.rs::camel_case_family",
        "Ordered camelCase key map via IndexMap.",
    ),
    "TableMap": (
        "hitool_core::TableMap",
        "crates/hitool-core/tests/map_parity.rs::table_map_multi_value_lookup",
        "Parallel key/value vectors support duplicate keys and reverse lookups.",
    ),
    "TolerantMap": (
        "hitool_core::TolerantMap",
        "crates/hitool-core/tests/map_parity.rs::tolerant_map_default_get",
        "Missing keys return the configured default value.",
    ),
    "FixedLinkedHashMap": (
        "hitool_core::FixedLinkedHashMap",
        "crates/hitool-core/tests/map_parity.rs::fixed_linked_hash_map_lru",
        "Capacity-bounded IndexMap evicts least-recently-used entries.",
    ),
    "FuncMap": (
        "hitool_core::FuncMap",
        "crates/hitool-core/tests/map_parity.rs::func_and_custom_key_maps",
        "Missing keys are materialized via an injected factory and cached.",
    ),
    "FuncKeyMap": (
        "hitool_core::FuncKeyMap",
        "crates/hitool-core/tests/map_parity.rs::func_and_custom_key_maps",
        "Keys are transformed by closure before storage/lookup.",
    ),
    "CustomKeyMap": (
        "hitool_core::CustomKeyMap",
        "crates/hitool-core/tests/map_parity.rs::func_and_custom_key_maps",
        "CustomKeyMap is the FuncKeyMap type alias for Hutool custom key maps.",
    ),
    "TransMap": (
        "hitool_core::TransMap",
        "crates/hitool-core/tests/map_parity.rs::func_and_custom_key_maps",
        "Key and value transformers run on put before insertion.",
    ),
    "SafeConcurrentHashMap": (
        "hitool_core::SafeConcurrentHashMap",
        "crates/hitool-core/tests/map_parity.rs::safe_concurrent_hash_map_shared",
        "Arc<Mutex<HashMap>> provides Hutool SafeConcurrentHashMap computeIfAbsent semantics.",
    ),
    "AbsEntry": (
        "hitool_core::AbsEntry",
        "crates/hitool-core/tests/map_parity.rs::abs_entry_and_forest",
        "Owned mutable Map.Entry analogue with get/setValue.",
    ),
    "LinkedForestMap": (
        "hitool_core::LinkedForestMap",
        "crates/hitool-core/tests/map_parity.rs::abs_entry_and_forest",
        "Parent/child forest nodes support link/unlink, roots, and descendant queries.",
    ),
    "ForestMap": (
        "hitool_core::ForestMap",
        "crates/hitool-core/tests/map_parity.rs::abs_entry_and_forest",
        "ForestMap is the LinkedForestMap type alias matching Hutool's interface default.",
    ),
    "TreeEntry": (
        "hitool_core::TreeEntry",
        "crates/hitool-core/tests/map_parity.rs::abs_entry_and_forest",
        "Tree nodes expose key/value/parent/children for forest maps.",
    ),
    "AbsCollValueMap": (
        "hitool_core::AbsCollValueMap",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "Collection-valued map operations are shared via ListValueMap / CollValueMapOps.",
    ),
    "ListValueMap": (
        "hitool_core::ListValueMap",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "Keys map to Vec values with putValue/removeValue semantics.",
    ),
    "SetValueMap": (
        "hitool_core::SetValueMap",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "Keys map to de-duplicated value lists.",
    ),
    "CollectionValueMap": (
        "hitool_core::CollectionValueMap",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "CollectionValueMap aliases ListValueMap for Hutool naming parity.",
    ),
    "RowKeyTable": (
        "hitool_core::RowKeyTable",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "Row-primary 2D table supports put/get/column/row views and cell iteration.",
    ),
    "AbsTable": (
        "hitool_core::AbsTable",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "AbsTable aliases RowKeyTable as the concrete Table implementation.",
    ),
    "Table": (
        "hitool_core::Table",
        "crates/hitool-core/tests/map_parity.rs::multi_value_maps_and_table",
        "Table interface is satisfied by RowKeyTable in Rust.",
    ),
}

# JVM GC / Proxy / reflection — register as planned only.
PLANNED: dict[str, str] = {
    "MapProxy": "Java dynamic Proxy / Bean-style Map getters have no safe 1:1 Rust mapping; MapProxy is a thin wrap with PendingEngine property access.",
    "ReferenceConcurrentMap": "JVM SoftReference/WeakReference concurrent map semantics are not portable; Arc<Mutex<HashMap>> placeholder only.",
    "WeakConcurrentMap": "WeakHashMap GC eviction semantics require a JVM collector; planned pending an explicit weak-key design.",
    "SoftConcurrentMap": "SoftReference memory-pressure eviction is JVM-specific; planned.",
    "WeakKeyConcurrentMap": "Weak key concurrent maps are JVM GC semantics; planned.",
    "WeakKeyValueConcurrentMap": "Weak key/value concurrent maps are JVM GC semantics; planned.",
}

# Per-API planned overrides inside otherwise idiomatic families.
PLANNED_API_SUBSTR = (
    "TypeReference",
    "reverse::<Entry$anon",
    "MapUtil::createProxy",
    "MapUtil::createMap",  # Class<?> reflection — covered by CreateMapKind but Class path stays planned
)


def class_name(qualified_name: str) -> str | None:
    for prefix in (MULTI_ROOT + "::", REF_ROOT + "::", ROOT + "::"):
        if qualified_name.startswith(prefix):
            rest = qualified_name[len(prefix) :]
            return rest.split("::", 1)[0].split("#", 1)[0]
    return None


def is_map_package(api_id: str) -> bool:
    return (
        api_id.startswith(ROOT + "::")
        or api_id.startswith(MULTI_ROOT + "::")
        or api_id.startswith(REF_ROOT + "::")
        or api_id.startswith(ROOT + ".")
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        # Preserve insertion order of existing rows; merge-only upserts.
        ordered: list[dict[str, str]] = list(csv.DictReader(stream))
        indexed = {row["api_id"]: row for row in ordered}
        order = [row["api_id"] for row in ordered]

    selected = 0
    idiomatic_n = 0
    planned_n = 0
    for row in inventory:
        api_id = row["api_id"]
        if not is_map_package(api_id):
            continue
        qn = row["qualified_name"]
        cls = class_name(qn)
        if cls is None:
            # Fallback: parse from api_id (map.multi::Foo / map::Foo)
            for prefix in (MULTI_ROOT + "::", REF_ROOT + "::", ROOT + "::"):
                if api_id.startswith(prefix):
                    cls = api_id[len(prefix) :].split("::", 1)[0].split("#", 1)[0]
                    break
        if cls is None:
            continue
        selected += 1

        planned_note = PLANNED.get(cls)
        if planned_note is None and any(s in qn or s in api_id for s in PLANNED_API_SUBSTR):
            planned_note = (
                "Java reflection / anonymous Entry / Class<?> Map factory paths are planned; "
                "idiomatic Rust uses closures, CreateMapKind, and owned entries instead."
            )

        if planned_note is not None:
            decision = {
                "api_id": api_id,
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": planned_note,
            }
            planned_n += 1
        elif cls in IDIOMATIC:
            symbol, evidence, notes = IDIOMATIC[cls]
            decision = {
                "api_id": api_id,
                "status": "idiomatic",
                "hitool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
            idiomatic_n += 1
        else:
            decision = {
                "api_id": api_id,
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": f"cn.hutool.core.map::{cls} awaits an idiomatic Rust mapping.",
            }
            planned_n += 1

        if api_id in indexed:
            indexed[api_id] = decision
        else:
            indexed[api_id] = decision
            order.append(api_id)

    if selected != 490:
        raise SystemExit(f"expected 490 map APIs in inventory, selected {selected}")
    if idiomatic_n < 150:
        raise SystemExit(f"expected >=150 idiomatic map APIs, got {idiomatic_n}")

    rows = [indexed[api_id] for api_id in order if api_id in indexed]
    # Keep any indexed keys somehow missing from order (should not happen).
    seen = set(order)
    for api_id, row in indexed.items():
        if api_id not in seen:
            rows.append(row)

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(rows)

    print(
        f"recorded map package: idiomatic={idiomatic_n}, planned={planned_n}, "
        f"selected={selected} (merge-only into {DECISIONS})"
    )


if __name__ == "__main__":
    main()
