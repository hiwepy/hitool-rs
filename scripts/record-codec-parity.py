#!/usr/bin/env python3
"""Record only the Hutool codec APIs reviewed against executable vectors."""

from __future__ import annotations

import csv
from pathlib import Path


ROOT = "cn.hutool.core.codec::"
INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def family(row: dict[str, str]) -> str | None:
    name = row["qualified_name"]
    signature = row["signature"]

    for complete in ("Rot", "Caesar", "Morse", "PunyCode", "Hashids"):
        prefix = f"{ROOT}{complete}"
        if name == prefix or name.startswith(f"{prefix}::"):
            return complete.lower()

    if name == f"{ROOT}BCD" or (
        name.startswith(f"{ROOT}BCD::") and "ascLength" not in signature
    ):
        return "bcd"

    if name == f"{ROOT}Base32" or (
        name.startswith(f"{ROOT}Base32::") and "Charset" not in signature
    ):
        return "base32"
    if name == f"{ROOT}Base32Codec" or name in {
        f"{ROOT}Base32Codec::encode",
        f"{ROOT}Base32Codec::decode",
    }:
        return "base32"

    if name == f"{ROOT}Base58" or name.startswith(f"{ROOT}Base58::"):
        return "base58"
    if name == f"{ROOT}Base58Codec" or name in {
        f"{ROOT}Base58Codec::encode",
        f"{ROOT}Base58Codec::decode",
    }:
        return "base58"

    excluded_base62 = ("Charset", "InputStream", "File", "OutputStream")
    if name == f"{ROOT}Base62" or (
        name.startswith(f"{ROOT}Base62::")
        and name != f"{ROOT}Base62::decodeStrGbk"
        and not any(excluded in signature for excluded in excluded_base62)
    ):
        return "base62"
    if name == f"{ROOT}Base62Codec" or name in {
        f"{ROOT}Base62Codec::encode",
        f"{ROOT}Base62Codec::decode",
    }:
        return "base62"
    return None


def rust_symbol(codec_family: str) -> str:
    return {
        "base32": "hitool_core::{base32_encode,base32_decode,base32_hex_encode,base32_hex_decode}",
        "base58": "hitool_core::{base58_encode,base58_decode,base58_encode_checked,base58_decode_checked}",
        "base62": "hitool_core::{base62_encode,base62_decode,base62_inverted_encode,base62_inverted_decode}",
        "rot": "hitool_core::{rot_encode,rot_decode}",
        "caesar": "hitool_core::{caesar_encode,caesar_decode}",
        "bcd": "hitool_core::{bcd_encode,bcd_decode}",
        "morse": "hitool_core::MorseCodec",
        "punycode": "hitool_core::{punycode_encode_prefixed,punycode_decode,idna_encode_domain,idna_decode_domain}",
        "hashids": "hitool_core::HashIds",
    }[codec_family]


def evidence(codec_family: str) -> str:
    test = {
        "base32": "base32_standard_and_hex_match_rfc_vectors",
        "base58": "base58_and_check_preserve_leading_zeroes",
        "base62": "base62_both_alphabets_round_trip_binary",
        "rot": "classical_codecs_are_reversible",
        "caesar": "classical_codecs_are_reversible",
        "bcd": "classical_codecs_are_reversible",
        "morse": "morse_handles_dictionary_custom_markers_and_unicode_fallback",
        "punycode": "punycode_and_idna_round_trip",
        "hashids": "hashids_support_numbers_hex_and_custom_alphabet",
    }[codec_family]
    return f"crates/hitool-core/src/advanced_codec.rs::{test}"


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
            "hitool_symbol": rust_symbol(codec_family),
            "test_evidence": evidence(codec_family),
            "notes": "Hutool overloads are consolidated into typed Rust APIs; official Hutool vectors are asserted.",
        }

    if selected != 73:
        raise SystemExit(f"expected 73 reviewed codec APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool codec APIs")


if __name__ == "__main__":
    main()
