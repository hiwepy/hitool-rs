#!/usr/bin/env python3
"""Record reviewed Hutool JWT APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
PREFIX = "crates/hutool-jwt/src/compat.rs::"


def mapping(qualified_name: str) -> tuple[str, str, str] | None:
    base = "cn.hutool.jwt"
    if qualified_name.startswith(f"{base}::"):
        tail = qualified_name.removeprefix(f"{base}::")
    elif qualified_name.startswith(f"{base}.signers::"):
        tail = "signers::" + qualified_name.removeprefix(f"{base}.signers::")
    else:
        return None

    family = tail.split("::", 1)[0]
    if family in {"Claims", "JWTException", "JWTHeader", "JWTPayload", "RegisteredPayload"}:
        test = "claims_headers_payloads_and_errors_are_dynamic_but_bounded"
        note = (
            "Serde JSON-backed claims preserve Hutool's dynamic header and payload model; "
            "Rust UTF-8 strings replace mutable Charset state and failures use typed Results."
        )
    elif family == "JWT":
        test = "jwt_builder_parser_verifier_and_util_round_trip"
        note = (
            "The Rust builder/parser preserves Hutool JWT construction, mutation, signing, "
            "verification, and leeway validation with explicit signer ownership and typed errors."
        )
    elif family == "JWTUtil":
        test = "jwt_util_supports_keys_headers_and_explicit_signers"
        note = (
            "Hutool JWTUtil overloads map to named Rust functions for shared keys, headers, "
            "explicit signers, parsing, and verification."
        )
    elif family == "JWTValidator":
        test = "validator_checks_all_registered_time_boundaries_and_types"
        note = (
            "Owned validators support token/JWT construction, explicit or configured signers, "
            "and nbf/exp/iat checks with saturating leeway arithmetic."
        )
    elif family != "signers":
        return None
    else:
        signer_family = tail.split("::", 2)[1]
        symbol = f"hutool_jwt::{signer_family}"
        legacy = any(
            marker in tail
            for marker in (
                "::es512#",
                "::hmd5#",
                "::hsha1#",
                "::sm4cmac#",
                "::rmd2#",
                "::rmd5#",
                "::rsha1#",
                "::dnone#",
                "::dsha1#",
                "::enone#",
                "::esha1#",
            )
        )
        if legacy:
            return (
                symbol,
                PREFIX + "legacy_and_non_jose_signer_aliases_are_explicitly_rejected",
                "The Hutool-named compatibility entry exists but rejects obsolete, insecure, "
                "or non-JOSE algorithms instead of enabling them in production.",
            )
        if signer_family in {"AlgorithmUtil", "JWTSignerUtil"}:
            test = "algorithm_names_and_signer_factories_are_explicit"
        else:
            test = "hmac_none_rsa_and_ecdsa_signers_use_real_crypto"
        note = (
            "RustCrypto-backed JOSE signers cover none, HS256/384/512, RS256/384/512, and "
            "ES256/384 with real PEM keys, signature verification, and malformed-input tests."
        )
        return symbol, PREFIX + test, note

    return f"hutool_jwt::{family}", PREFIX + test, note


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-jwt":
            continue
        target = mapping(row["qualified_name"])
        if target is None:
            raise SystemExit(f"unmapped Hutool JWT family: {row['qualified_name']}")
        selected += 1
        symbol, test, notes = target
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }

    if selected != 121:
        raise SystemExit(f"expected 121 reviewed JWT APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool JWT APIs")


if __name__ == "__main__":
    main()
