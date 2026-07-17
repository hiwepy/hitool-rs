#!/usr/bin/env python3
"""Record reviewed Hutool core utility families with executable evidence."""

from __future__ import annotations

import csv
from pathlib import Path


BOOLEAN_ROOT = "cn.hutool.core.util::BooleanUtil"
BYTE_ROOT = "cn.hutool.core.util::ByteUtil"
CHAR_ROOT = "cn.hutool.core.util::CharUtil"
CHARSET_ROOT = "cn.hutool.core.util::CharsetUtil"
COORDINATE_ROOT = "cn.hutool.core.util::CoordinateUtil"
CREDIT_CODE_ROOT = "cn.hutool.core.util::CreditCodeUtil"
DESENSITIZED_ROOT = "cn.hutool.core.util::DesensitizedUtil"
HASH_ROOT = "cn.hutool.core.util::HashUtil"
HEX_ROOT = "cn.hutool.core.util::HexUtil"
IDCARD_ROOT = "cn.hutool.core.util::IdcardUtil"
PAGE_ROOT = "cn.hutool.core.util::PageUtil"
PHONE_ROOT = "cn.hutool.core.util::PhoneUtil"
RADIX_ROOT = "cn.hutool.core.util::RadixUtil"
VERSION_ROOT = "cn.hutool.core.util::VersionUtil"
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


def charset_evidence(name: str, signature: str) -> str:
    if name == "convert" and "File" in signature:
        return "files_and_candidate_detection_are_bounded_and_report_io_failures"
    if name == "defaultCharset" and "InputStream" in signature:
        return "files_and_candidate_detection_are_bounded_and_report_io_failures"
    if name == "convert":
        return "string_conversion_uses_exact_single_byte_utf16_and_encoding_rs_engines"
    return "resolution_parsing_names_and_special_java_charsets_are_explicit"


def char_evidence(name: str) -> str:
    if name.startswith("toClose"):
        return "enclosed_conversions_cover_supported_tables_and_errors"
    if name in {"isBlankChar", "isEmoji", "isFileSeparator", "equals", "getType", "digit16"}:
        return "unicode_blank_emoji_category_and_digit_paths_are_explicit"
    return "ascii_classification_and_dynamic_character_checks_match_hutool"


def credit_code_evidence(name: str) -> str:
    if name == "isCreditCodeSimple":
        return "simple_validation_enforces_every_structural_section"
    if name == "randomCreditCode":
        return "generated_codes_are_reproducible_with_an_injected_rng_and_always_valid"
    return "weighted_validation_matches_hutool_vectors_and_rejects_bad_check_digits"


def coordinate_evidence(qualified_name: str, name: str) -> str:
    if qualified_name.startswith(f"{COORDINATE_ROOT}::Coordinate") or name == "outOfChina":
        return "boundaries_mutation_equality_hash_and_display_are_real_value_semantics"
    return "conversions_match_hutool_reference_vectors_and_round_trip_mercator"


def hash_evidence(name: str) -> str:
    if name in {"additiveHash", "rotatingHash", "universal", "zobrist"}:
        return "modular_and_table_hashes_validate_inputs_and_cover_every_bit_branch"
    if name == "tianlHash":
        return "tianl_hash_covers_empty_short_tail_long_and_ascii_case_rules"
    return "classic_hashes_match_java_utf16_wrapping_and_signed_byte_rules"


def desensitized_evidence(name: str) -> str:
    if name in {"desensitized", ""}:
        return "dispatcher_covers_every_strategy_and_blank_short_circuit"
    if name in {"bankCard", "ipv4", "ipv6", "passport", "creditCode"}:
        return "bank_network_passport_and_credit_code_paths_are_complete"
    return "individual_masking_functions_cover_invalid_short_and_unicode_inputs"


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


def idcard_evidence(qualified_name: str, name: str) -> str:
    if qualified_name.startswith(f"{IDCARD_ROOT}::Idcard::") or qualified_name == f"{IDCARD_ROOT}::Idcard":
        return "owned_idcard_info_exposes_every_value_and_display"
    if name in {"isValidCard", "isValidCard18", "isValidCard15", "convert15To18", "convert18To15"}:
        return "mainland_conversion_validation_and_checksums_match_hutool"
    if name in {"isValidCard10", "isValidTWCard", "isValidHKCard"}:
        return "regional_card_rules_cover_taiwan_macao_and_hong_kong"
    if name in {"getBirthByIdCard", "getBirth", "getBirthDate", "getAgeByIdCard"}:
        return "birthday_and_age_are_checked"
    return "components_codes_and_masking_are_checked"


def page_evidence(name: str) -> str:
    if name == "rainbow":
        return "rainbow_covers_short_leading_centered_trailing_even_and_odd_windows"
    return "numbering_offsets_segments_and_totals_match_hutool_semantics"


def phone_evidence(name: str) -> str:
    if name.startswith("hide") or name.startswith("sub"):
        return "masking_slicing_and_landline_capture_are_unicode_safe"
    return "regional_and_service_patterns_match_hutool_exactly"


def version_evidence(name: str) -> str:
    if name in {"matchEl", "anyMatch"}:
        return "exact_comparison_range_and_multi_expression_matching_are_complete"
    return "comparisons_support_hutool_loose_versions_and_null_expression"


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
        elif qualified_name.startswith(CHAR_ROOT):
            symbol = "hitool_core::CharUtil"
            test = char_evidence(name)
            notes = "Rust char and Any provide the thin facade; unicode-general-category supplies Unicode 16 categories while explicit Java constants, blank characters, digit values, emoji heuristics, and checked enclosed conversions preserve Hutool behavior."
        elif qualified_name.startswith(CHARSET_ROOT):
            symbol = "hitool_core::CharsetUtil"
            test = charset_evidence(name, row["signature"])
            notes = "encoding_rs supplies mature WHATWG codecs while the thin facade preserves exact ISO-8859-1, ASCII, BOM-aware UTF-16, string repair, bounded file conversion, and candidate detection semantics."
        elif qualified_name.startswith(COORDINATE_ROOT):
            symbol = "hitool_core::CoordinateUtil"
            test = coordinate_evidence(qualified_name, name)
            notes = "Rust f64 math implements Hutool's WGS84, GCJ-02, BD-09, and Web Mercator formulas; the owned Coordinate value preserves mutation, Java-style equality, hashing, and display semantics."
        elif qualified_name.startswith(CREDIT_CODE_ROOT):
            symbol = "hitool_core::CreditCodeUtil"
            test = credit_code_evidence(name)
            notes = "A direct GB 32100-2015 weighted checksum implementation validates every section and rand-backed generation always appends a verified parity character."
        elif qualified_name.startswith(DESENSITIZED_ROOT):
            symbol = "hitool_core::DesensitizedUtil"
            test = desensitized_evidence(name)
            notes = "A Unicode-safe owned-string facade implements every Hutool masking policy while Option preserves the observable null versus empty distinction for dispatcher, bank-card, passport, and credit-code paths."
        elif qualified_name.startswith(HASH_ROOT) and name not in MODERN_HASH_METHODS:
            symbol = "hitool_core::HashUtil"
            test = hash_evidence(name)
            notes = "Explicit wrapping arithmetic and Java UTF-16 units preserve Hutool's classic hash formulas; identity hashing is lifetime-bound and table algorithms return validated errors. Murmur, City, and Metro methods remain uncounted pending a version-compatible engine."
        elif qualified_name.startswith(HEX_ROOT):
            symbol = "hitool_core::HexUtil"
            test = hex_evidence(name)
            notes = "The mature hex and num-bigint crates plus Rust UTF-16 and encoding primitives provide the engine; the facade preserves Hutool prefixes, Java numeric bit patterns, colors, pair formatting, and typed errors."
        elif qualified_name.startswith(IDCARD_ROOT):
            symbol = "hitool_core::{IdcardUtil,Idcard,Card10Info}"
            test = idcard_evidence(qualified_name, name)
            notes = "Chrono supplies strict Gregorian dates while checked Rust checksum, region, age, conversion, masking, and owned-value logic preserves Hutool mainland, Taiwan, Macao, Hong Kong, and foreign-resident card semantics with structured errors."
        elif qualified_name.startswith(PAGE_ROOT):
            symbol = "hitool_core::PageUtil"
            test = page_evidence(name)
            notes = "Explicit owned page-number configuration, Rust Range segments, Java-compatible wrapping offsets, checked total-page narrowing, and the complete rainbow algorithm implement Hutool behavior without hidden global state."
        elif qualified_name.startswith(PHONE_ROOT):
            symbol = "hitool_core::PhoneUtil"
            test = phone_evidence(name)
            notes = "The mature regex engine compiles Hutool's mainland, Hong Kong, Taiwan, Macao, landline, 400, and 800 rules once; the thin facade provides Unicode-safe masking, slicing, and typed optional captures."
        elif qualified_name.startswith(RADIX_ROOT):
            symbol = "hitool_core::RadixUtil"
            test = "integer_overloads_round_trip_zero_positive_negative_and_unicode_alphabets"
            notes = "A bounded unique Unicode alphabet, checked arithmetic, and exact unsigned-i32 and narrowing semantics provide reversible custom-radix conversion with structured errors."
        elif qualified_name.startswith(VERSION_ROOT):
            symbol = "hitool_core::VersionUtil"
            test = version_evidence(name)
            notes = "A direct loose-version token engine preserves Hutool's non-SemVer ordering, null comparison, exact, operator, inclusive-range, multi-expression, and delimiter-validation behavior."
        else:
            continue

        selected += 1
        source = {
            "hitool_core::BooleanUtil": "boolean_util.rs",
            "hitool_core::ByteUtil": "byte_util.rs",
            "hitool_core::CharUtil": "char_util.rs",
            "hitool_core::CharsetUtil": "charset_util.rs",
            "hitool_core::CoordinateUtil": "coordinate_util.rs",
            "hitool_core::CreditCodeUtil": "credit_code_util.rs",
            "hitool_core::DesensitizedUtil": "desensitized_util.rs",
            "hitool_core::HashUtil": "hash_util.rs",
            "hitool_core::HexUtil": "hex_util.rs",
            "hitool_core::{IdcardUtil,Idcard,Card10Info}": "idcard_util.rs",
            "hitool_core::PageUtil": "page_util.rs",
            "hitool_core::PhoneUtil": "phone_util.rs",
            "hitool_core::RadixUtil": "radix_util.rs",
            "hitool_core::VersionUtil": "version_util.rs",
        }[symbol]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": f"crates/hitool-core/src/{source}::{test}",
            "notes": notes,
        }

    if selected != 275:
        raise SystemExit(f"expected 275 reviewed core util APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed core util APIs")


if __name__ == "__main__":
    main()
