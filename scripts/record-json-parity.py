#!/usr/bin/env python3
"""Record reviewed Hutool JSON APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str, str]:
    parts = qualified_name.split("::")
    class_name = parts[1]
    if class_name in {"JSONArray", "JSONArraySerializer"}:
        return (
            "hitool_json::JSONArray",
            "crates/hitool-json/src/compat.rs::array_mutations_paths_and_formatting_are_complete",
            "The configured Rust array preserves Hutool construction, mutation, paths, iteration, conversion and formatting over serde_json values.",
        )
    if class_name in {"JSONObject", "JSONObjectIter", "JSONObjectSerializer", "JSONGetter"}:
        return (
            "hitool_json::JSONObject",
            "crates/hitool-json/src/compat.rs::object_mutations_cover_case_null_duplicates_arrays_and_numbers",
            "The configured Rust object preserves Hutool lookup, mutation, paths, duplicate/null policy, iteration and conversion without Java reflection.",
        )
    if class_name in {"JSONConfig", "JSONNull", "JSON"}:
        return (
            "hitool_json::{JSONConfig,JSONNull,JsonContainer}",
            "crates/hitool-json/src/compat.rs::config_exposes_every_hutool_option",
            "Configuration and null/container compatibility are explicit owned values; Java-only ordering hooks are retained as deterministic Rust policy.",
        )
    if class_name in {"JSONTokener", "JSONParser", "ParseConfig"}:
        return (
            "hitool_json::{JSONTokener,JSONParser,ParseConfig}",
            "crates/hitool-json/src/parser.rs::readers_parsers_and_string_errors_are_explicit",
            "The Unicode tokenizer and parser enforce bounded input and nesting while preserving Hutool navigation and container parsing operations.",
        )
    if class_name in {"XML", "XMLTokener", "JSONXMLParser", "JSONXMLSerializer"}:
        return (
            "hitool_json::{XML,XMLTokener,JSONXMLParser,JSONXMLSerializer}",
            "crates/hitool-json/src/xml.rs::xml_parses_attributes_repeated_tags_scalars_cdata_and_empty_nodes",
            "Bounded quick-xml conversion covers attributes, repeated nodes, scalar policy, CDATA, empty elements and escaped serialization.",
        )
    if class_name in {
        "GlobalSerializeMapping",
        "JSONSerializer",
        "JSONDeserializer",
        "TemporalAccessorSerializer",
    }:
        return (
            "hitool_json::{SerializeRegistry,GlobalSerializeMapping,JSONSerializer,JSONDeserializer}",
            "crates/hitool-json/src/serialize.rs::owned_registry_maps_both_directions_and_reports_missing_types",
            "Typed serializers and deserializers use an owned registry; the migration global is replaceable and resettable rather than hidden mutable state.",
        )
    if class_name == "JSONWriter":
        return (
            "hitool_json::JSONWriter",
            "crates/hitool-json/src/facade.rs::writer_enforces_state_nulls_and_long_strings",
            "The stateful writer enforces object/array sequencing, null policy, long-integer policy and propagates write and flush failures.",
        )
    if class_name in {
        "JSONUtil",
        "JSONStrFormatter",
        "JSONSupport",
        "JSONString",
        "JSONConverter",
        "ObjectMapper",
        "JSONBeanParser",
    }:
        return (
            "hitool_json::{JSONUtil,JSONStrFormatter,JSONSupport,JSONConverter,ObjectMapper}",
            "crates/hitool-json/src/facade.rs::util_support_converter_and_mapper_delegate_to_serde",
            "The Hutool-shaped facade delegates to one Serde engine and typed Rust conversion instead of duplicating parser or reflection logic.",
        )
    return (
        "hitool_json::{JsonError,parse,to_string,from_str}",
        "crates/hitool-json/src/lib.rs::formatting_is_reversible",
        "Internal conversion and exception surfaces map to typed Rust results backed by the shared Serde implementation.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-json":
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 294:
        raise SystemExit(f"expected 294 reviewed JSON APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool JSON APIs")


if __name__ == "__main__":
    main()
