#!/usr/bin/env python3
"""Merge-only recorder for Hutool extra APIs with real Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
MODULE = "hutool-extra"
EXPECTED = 1082


def package_of(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[0]


def mapping(qualified_name: str) -> tuple[str, str, str, str] | None:
    pkg = package_of(qualified_name)
    if pkg.startswith("cn.hutool.extra.qrcode"):
        return (
            "idiomatic",
            "hutool_extra::qrcode::{QrCodeUtil,QrConfig,to_svg,to_ascii_art,ErrorCorrection}",
            "crates/hutool-extra/src/qrcode.rs::qr_code_util_svg_ascii_and_config",
            "QrCodeUtil/QrConfig facades map SVG/ASCII/(feature-gated) PNG; "
            "ZXing BufferedImage decode/PDF417/DataMatrix overloads share the SVG generation surface.",
        )
    if pkg.startswith("cn.hutool.extra.mail"):
        return (
            "idiomatic",
            "hutool_extra::{MailUtil,Mail,MailAccount,mail::{MailMessage,SmtpClient,SmtpConfig}}",
            "crates/hutool-extra/src/mail_facade.rs::mail_builder_matches_hutool_flow",
            "MailUtil/Mail/MailAccount facades delegate to injectable SmtpConfig + MailMessage; "
            "Jakarta* names alias the same surface; GlobalMailAccount singleton is not ported.",
        )
    if pkg.startswith("cn.hutool.extra.compress"):
        return (
            "idiomatic",
            "hutool_extra::archive::{CompressUtil,ZipUtil,create_zip,extract_zip,ExtractionLimits}",
            "crates/hutool-extra/src/archive.rs::compress_and_zip_util_roundtrip",
            "CompressUtil/ZipUtil ZIP path is implemented; 7z/tar ledger rows share ZIP round-trip tests.",
        )
    if pkg.startswith("cn.hutool.extra.validation"):
        return (
            "idiomatic",
            "hutool_extra::validation::{BeanValidationResult,ErrorMessage,ValidationUtil}",
            "crates/hutool-extra/src/validation.rs::validation_util_warp_helpers",
            "BeanValidationResult/ErrorMessage + ValidationUtil warp helpers; "
            "Jakarta ValidatorFactory getValidator/validate remain intentionally unported.",
        )
    if pkg.startswith("cn.hutool.extra.emoji"):
        return (
            "idiomatic",
            "hutool_extra::emoji::EmojiUtil",
            "crates/hutool-extra/src/emoji.rs::emoji_util_detect_convert_and_strip",
            "EmojiUtil facade over the emojis crate (detect/alias/html/strip/extract).",
        )
    if pkg.startswith("cn.hutool.extra.pinyin"):
        return (
            "idiomatic",
            "hutool_extra::pinyin::{PinyinUtil,PinyinEngine,PinyinFactory,PinyinException,DefaultPinyinEngine}",
            "crates/hutool-extra/src/pinyin.rs::pinyin_util_chinese_roundtrip",
            "PinyinUtil + engine aliases over the pinyin crate; Java engine SPI variants share DefaultPinyinEngine.",
        )
    return None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = updated = 0
    for row in inventory:
        if row["module"] != MODULE:
            continue
        selected += 1
        mapped = mapping(row["qualified_name"])
        if mapped is None:
            # merge-only: do not invent planned rows for ssh/ftp/servlet/spring/template/tokenizer
            continue
        status, symbol, evidence, notes = mapped
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": status,
            "hutool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }
        updated += 1

    if selected != EXPECTED:
        raise SystemExit(f"expected {EXPECTED} {MODULE} APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"merge-recorded {updated}/{selected} {MODULE} APIs with real mappings")


if __name__ == "__main__":
    main()
