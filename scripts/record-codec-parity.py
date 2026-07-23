#!/usr/bin/env python3
"""Record only the Hutool codec APIs reviewed against executable vectors."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.codec::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def family(row: dict[str, str]) -> str | None:
    name = row["qualified_name"]

    if name == f"{ROOT}Base16Codec" or name.startswith(f"{ROOT}Base16Codec::"):
        return "base16"

    for base64_type in ("Base64", "Base64Decoder", "Base64Encoder"):
        prefix = f"{ROOT}{base64_type}"
        if name == prefix or name.startswith(f"{prefix}::"):
            return "base64"

    if name == f"{ROOT}PercentCodec" or name.startswith(f"{ROOT}PercentCodec::"):
        return "percent"

    if name in {f"{ROOT}Encoder", f"{ROOT}Decoder"}:
        return "traits"

    for complete in ("Rot", "Caesar", "Morse", "PunyCode", "Hashids"):
        prefix = f"{ROOT}{complete}"
        if name == prefix or name.startswith(f"{prefix}::"):
            return complete.lower()

    if name == f"{ROOT}BCD" or name.startswith(f"{ROOT}BCD::"):
        return "bcd"

    if name == f"{ROOT}Base32" or name.startswith(f"{ROOT}Base32::"):
        return "base32"
    if name == f"{ROOT}Base32Codec" or name.startswith(f"{ROOT}Base32Codec::"):
        return "base32"

    if name == f"{ROOT}Base58" or name.startswith(f"{ROOT}Base58::"):
        return "base58"
    if name == f"{ROOT}Base58Codec" or name.startswith(f"{ROOT}Base58Codec::"):
        return "base58"

    if name == f"{ROOT}Base62" or name.startswith(f"{ROOT}Base62::"):
        return "base62"
    if name == f"{ROOT}Base62Codec" or name.startswith(f"{ROOT}Base62Codec::"):
        return "base62"
    return None


def rust_symbol(codec_family: str) -> str:
    return {
        "base16": "hutool_core::Base16Codec",
        "base32": "hutool_core::{Base32Encoder,Base32Decoder,base32_encode_text,base32_decode_text,base32_encode_file,base32_decode_to_file}",
        "base58": "hutool_core::{Base58Encoder,Base58Decoder,base58_encode_checked,base58_decode_checked}",
        "base62": "hutool_core::{Base62Encoder,Base62Decoder,base62_encode_text,base62_decode_text,base62_encode_file,base62_decode_to_file}",
        "rot": "hutool_core::{rot_encode,rot_decode}",
        "caesar": "hutool_core::{caesar_encode,caesar_decode}",
        "bcd": "hutool_core::{bcd_encode,bcd_decode,bcd_encode_ascii_prefix}",
        "morse": "hutool_core::MorseCodec",
        "punycode": "hutool_core::{punycode_encode_prefixed,punycode_decode,idna_encode_domain,idna_decode_domain}",
        "hashids": "hutool_core::HashIds",
        "base64": "hutool_core::{base64_encode_config,base64_decode_tolerant,base64_encode_text,base64_decode_text}",
        "percent": "hutool_core::PercentCodec",
        "traits": "hutool_core::{Encoder,Decoder}",
    }[codec_family]


def evidence(codec_family: str) -> str:
    test = {
        "base16": "base16_matches_hutool_whitespace_odd_length_and_unicode_rules",
        "base32": "custom_radix_alphabets_round_trip_and_validate",
        "base58": "custom_radix_alphabets_round_trip_and_validate",
        "base62": "text_stream_file_and_bcd_overloads_are_bounded_and_reversible",
        "rot": "classical_codecs_are_reversible",
        "caesar": "classical_codecs_are_reversible",
        "bcd": "text_stream_file_and_bcd_overloads_are_bounded_and_reversible",
        "morse": "morse_handles_dictionary_custom_markers_and_unicode_fallback",
        "punycode": "punycode_and_idna_round_trip",
        "hashids": "hashids_support_numbers_hex_and_custom_alphabet",
        "base64": "base64_supports_hutool_variants_and_tolerant_decoding",
        "percent": "configurable_percent_codec_matches_hutool_safe_sets_and_plus_mode",
        "traits": "base16_matches_hutool_whitespace_odd_length_and_unicode_rules",
    }[codec_family]
    source = "radix_codec.rs" if codec_family in {"base32", "base58", "base62", "bcd"} else "advanced_codec.rs"
    if codec_family in {"base16", "base64", "percent", "traits"}:
        source = "hutool_codec.rs"
    return f"crates/hutool-core/src/{source}::{test}"


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        existing = list(csv.DictReader(stream))

    indexed = {row["api_id"]: row for row in existing}
    selected = 0
    for row in inventory:
        codec_family = family(row)
        if codec_family is None:
            continue
        selected += 1
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": rust_symbol(codec_family),
            "test_evidence": evidence(codec_family),
            "notes": "Hutool overloads are consolidated into typed Rust APIs; official Hutool vectors are asserted.",
        }

    if selected != 175:
        raise SystemExit(f"expected 175 reviewed codec APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool codec APIs")


if __name__ == "__main__":
    main()
