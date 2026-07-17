#!/usr/bin/env python3
"""Record reviewed BooleanUtil, ByteUtil, classic HashUtil, and HexUtil APIs."""

from __future__ import annotations

import csv
from pathlib import Path


BOOLEAN_ROOT = "cn.hutool.core.util::BooleanUtil"
BYTE_ROOT = "cn.hutool.core.util::ByteUtil"
HASH_ROOT = "cn.hutool.core.util::HashUtil"
HEX_ROOT = "cn.hutool.core.util::HexUtil"
MODERN_HASH_METHODS = {"murmur32", "murmur64", "murmur128", "cityHash32", "cityHash64", "cityHash128", "metroHash64", "metroHash128"}
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    return parts[2] if len(parts) > 2 else ""


def boolean_evidence(name: str) -> str:
    if name in {"toBoolean", "toBooleanObject"}:
        return "parser_covers_hutool_multilingual_vocabulary_blank_and_unknown_values"
    if name.startswith("toString"):
        return "string_selection_covers_true_false_and_none"
    if name in {"and", "andOfWrap", "or", "orOfWrap", "xor", "xorOfWrap", "exactlyOneTrue"}:
        return "aggregations_match_hutool_empty_short_circuit_none_and_parity_rules"
    return "optional_negation_type_checks_and_all_conversions_are_explicit"


def byte_evidence(name: str) -> str:
    if name == "numberToBytes":
        return "number_to_bytes_trait_covers_every_java_wrapper_shape"
    if name == "bytesToNumber":
        return "bytes_to_number_trait_covers_primitives_atomics_adders_and_big_numbers"
    if "Float" in name or "Double" in name:
        return "floating_conversions_preserve_values_endianness_and_java_nan_canonicalization"
    return "primitive_integer_conversions_cover_defaults_endianness_offsets_and_bounds"


def hash_evidence(name: str) -> str:
    if name in {"additiveHash", "rotatingHash", "universal", "zobrist"}:
        return "modular_and_table_hashes_validate_inputs_and_cover_every_bit_branch"
    if name == "tianlHash":
        return "tianl_hash_covers_empty_short_tail_long_and_ascii_case_rules"
    return "classic_hashes_match_java_utf16_wrapping_and_signed_byte_rules"


def hex_evidence(name: str) -> str:
    if name in {"isHexNumber", "toBigInteger"}:
        return "recognition_preserves_hutool_prefix_sign_and_big_integer_rules"
    if name in {"encodeColor", "decodeColor"}:
        return "color_facade_matches_java_decode_and_padded_encoding"
    if name in {"appendHex", "format"}:
        return "append_and_pair_formatting_cover_empty_odd_and_prefix_forms"
    if name.startswith("toHex") or name.startswith("hexTo") or name == "toUnicodeHex":
        return "numeric_conversions_preserve_java_bit_patterns_and_prefix_fallthrough"
    return "byte_and_text_facades_delegate_to_base16_and_character_engines"


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        qualified_name = row["qualified_name"]
        name = method_name(qualified_name)
        if qualified_name.startswith(BOOLEAN_ROOT):
            symbol = "hitool_core::BooleanUtil"
            test = boolean_evidence(name)
            notes = "Rust bool, Option<bool>, TypeId, and iterator operations preserve Hutool conversion, vocabulary, selection, aggregation, and empty-input behavior."
        elif qualified_name.startswith(BYTE_ROOT):
            symbol = "hitool_core::ByteUtil"
            test = byte_evidence(name)
            notes = "Rust endian primitives implement checked offset conversion; generic traits replace Java Class dispatch while preserving primitive, atomic, adder, BigInteger, BigDecimal, and Java-canonical NaN behavior."
        elif qualified_name.startswith(HASH_ROOT) and name not in MODERN_HASH_METHODS:
            symbol = "hitool_core::HashUtil"
            test = hash_evidence(name)
            notes = "Explicit wrapping arithmetic and Java UTF-16 units preserve Hutool's classic hash formulas; identity hashing is lifetime-bound and table algorithms return validated errors. Murmur, City, and Metro methods remain uncounted pending a version-compatible engine."
        elif qualified_name.startswith(HEX_ROOT):
            symbol = "hitool_core::HexUtil"
            test = hex_evidence(name)
            notes = "The mature hex and num-bigint crates plus Rust UTF-16 and encoding primitives provide the engine; the facade preserves Hutool prefixes, Java numeric bit patterns, colors, pair formatting, and typed errors."
        else:
            continue

        selected += 1
        source = {
            "hitool_core::BooleanUtil": "boolean_util.rs",
            "hitool_core::ByteUtil": "byte_util.rs",
            "hitool_core::HashUtil": "hash_util.rs",
            "hitool_core::HexUtil": "hex_util.rs",
        }[symbol]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": f"crates/hitool-core/src/{source}::{test}",
            "notes": notes,
        }

    if selected != 120:
        raise SystemExit(f"expected 120 reviewed core util APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed BooleanUtil/ByteUtil/classic HashUtil/HexUtil APIs")


if __name__ == "__main__":
    main()
