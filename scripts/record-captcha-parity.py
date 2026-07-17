#!/usr/bin/env python3
"""Record reviewed Hutool CAPTCHA APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str] | None:
    prefixes = ("cn.hutool.captcha::", "cn.hutool.captcha.generator::")
    prefix = next((value for value in prefixes if qualified_name.startswith(value)), None)
    if prefix is None:
        return None
    family = qualified_name.removeprefix(prefix).split("::", 1)[0]
    if family in {"AbstractGenerator", "CodeGenerator", "MathGenerator", "RandomGenerator"}:
        return (
            f"hitool_captcha::{family}",
            "crates/hitool-captcha/src/compat.rs::generators_cover_validation_and_math",
        )
    if family in {"GifCaptcha"}:
        return (
            f"hitool_captcha::{family}",
            "crates/hitool-captcha/src/compat.rs::gif_and_constructor_variants_are_usable",
        )
    if family in {
        "AbstractCaptcha",
        "CaptchaUtil",
        "CircleCaptcha",
        "ICaptcha",
        "LineCaptcha",
        "ShearCaptcha",
    }:
        return (
            f"hitool_captcha::{family}",
            "crates/hitool-captcha/src/compat.rs::raster_variants_generate_real_media_and_common_facade_works",
        )
    return None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-captcha":
            continue
        target = mapping(row["qualified_name"])
        if target is None:
            raise SystemExit(f"unmapped Hutool CAPTCHA family: {row['qualified_name']}")
        selected += 1
        symbol, test = target
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": (
                "Rust traits and explicit Results preserve Hutool generator, verification, "
                "state, file/stream, Base64, Line, Circle, Shear, and animated GIF capabilities; "
                "raster media is feature-gated and backed by image/font8x8/base64."
            ),
        }

    if selected != 87:
        raise SystemExit(f"expected 87 reviewed CAPTCHA APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool CAPTCHA APIs")


if __name__ == "__main__":
    main()
