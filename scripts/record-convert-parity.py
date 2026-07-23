#!/usr/bin/env python3
"""Record reviewed Hutool core.convert APIs (merge-only).

Common typed Convert.* paths → idiomatic.
Class/Type reflection convert / converter registry engines → planned.
Never deletes existing decisions.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

CONVERT_ROOT = "cn.hutool.core.convert::Convert"
CAST_ROOT = "cn.hutool.core.convert::CastUtil"
NCF_ROOT = "cn.hutool.core.convert::NumberChineseFormatter"
NWF_ROOT = "cn.hutool.core.convert::NumberWordFormatter"
NWFMT_ROOT = "cn.hutool.core.convert::NumberWithFormat"
REG_ROOT = "cn.hutool.core.convert::ConverterRegistry"
BASIC_ROOT = "cn.hutool.core.convert::BasicType"

# Method names that need Java Class/Type reflection engines.
CONVERT_PLANNED = {
    "convertByClassName",
    "convert",  # Type/Class/TypeReference overloads — keep planned; typed helpers are idiomatic
    "convertQuietly",
    "convertWithCheck",
    "toEnum",
    "toCollection",
    "toMap",
    "wrap",
    "unWrap",
}

# Explicit typed helpers we ship under ConvertValue facades.
CONVERT_IDIOMATIC = {
    "",  # class row
    "toStr",
    "toStrArray",
    "toChar",
    "toCharArray",
    "toByte",
    "toByteArray",
    "toPrimitiveByteArray",
    "toShort",
    "toShortArray",
    "toNumber",
    "toNumberArray",
    "toInt",
    "toIntArray",
    "toLong",
    "toLongArray",
    "toDouble",
    "toDoubleArray",
    "toFloat",
    "toFloatArray",
    "toBool",
    "toBooleanArray",
    "toBigInteger",
    "toBigDecimal",
    "toDate",
    "toLocalDateTime",
    "toInstant",
    "toList",
    "toSet",
    "toSBC",
    "toDBC",
    "toHex",
    "hexToBytes",
    "hexToStr",
    "strToUnicode",
    "unicodeToStr",
    "convertCharset",
    "convertTime",
    "numberToWord",
    "numberToSimple",
    "numberToChinese",
    "chineseToNumber",
    "digitToChinese",
    "chineseMoneyToNumber",
    "intToByte",
    "byteToUnsignedInt",
    "bytesToShort",
    "shortToBytes",
    "bytesToInt",
    "intToBytes",
    "longToBytes",
    "bytesToLong",
}


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return ""
    return parts[2].split("#", 1)[0]


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    idiomatic_n = 0
    planned_n = 0

    convert_notes = (
        "ConvertValue-tagged facades cover common primitives/collections/hex/unicode/"
        "SBC-DBC/time/number formatters; Java Class/Type reflection convert stays planned."
    )
    convert_evidence = "crates/hitool-core/tests/convert_parity.rs"
    convert_symbol = "hitool_core::convert::Convert"

    for row in inventory:
        qn = row["qualified_name"]
        api_id = row["api_id"]

        if qn == CONVERT_ROOT or qn.startswith(CONVERT_ROOT + "::"):
            name = method_name(qn)
            selected += 1
            if name in CONVERT_PLANNED or (
                name not in CONVERT_IDIOMATIC and name not in {"", *CONVERT_IDIOMATIC}
            ):
                # planned for reflection / unknown
                if name in CONVERT_IDIOMATIC and name not in CONVERT_PLANNED:
                    status = "idiomatic"
                elif name in CONVERT_PLANNED:
                    status = "planned"
                elif name in CONVERT_IDIOMATIC:
                    status = "idiomatic"
                else:
                    status = "planned"
            else:
                status = "idiomatic"

            # refine: CONVERT_PLANNED wins
            if name in CONVERT_PLANNED:
                status = "planned"
            elif name in CONVERT_IDIOMATIC or name == "":
                status = "idiomatic"
            else:
                status = "planned"

            if status == "idiomatic":
                idiomatic_n += 1
                indexed[api_id] = {
                    "api_id": api_id,
                    "status": "idiomatic",
                    "hitool_symbol": convert_symbol,
                    "test_evidence": convert_evidence,
                    "notes": convert_notes,
                }
            else:
                planned_n += 1
                indexed[api_id] = {
                    "api_id": api_id,
                    "status": "planned",
                    "hitool_symbol": convert_symbol,
                    "test_evidence": "",
                    "notes": (
                        f"Planned: Convert.{name or 'class'} needs Java Class/Type/TypeReference "
                        "or registry polymorphism beyond ConvertValue facades."
                    ),
                }
            continue

        for root, symbol, evidence, notes, status in (
            (
                CAST_ROOT,
                "hitool_core::convert::CastUtil",
                "crates/hitool-core/tests/convert_parity.rs::cast_util_test_test_cast_to_super",
                "Identity cast_up/cast_down generics preserve collection type views without unsafe.",
                "idiomatic",
            ),
            (
                NCF_ROOT,
                "hitool_core::convert::NumberChineseFormatter",
                convert_evidence,
                "Chinese number formatter mirrors Hutool digit/money conversion helpers.",
                "idiomatic",
            ),
            (
                NWF_ROOT,
                "hitool_core::convert::NumberWordFormatter",
                convert_evidence,
                "English number-word formatter covers numberToWord/numberToSimple paths.",
                "idiomatic",
            ),
            (
                NWFMT_ROOT,
                "hitool_core::convert::NumberWithFormat",
                convert_evidence,
                "NumberWithFormat value wrapper feeds Convert typed numeric paths.",
                "idiomatic",
            ),
            (
                BASIC_ROOT,
                "hitool_core::convert::BasicType",
                convert_evidence,
                "BasicType wrap/unwrap maps primitive class names to Rust scalar tags.",
                "idiomatic",
            ),
            (
                REG_ROOT,
                "hitool_core::convert::ConverterRegistry",
                "",
                "Planned: full ConverterRegistry SPI / Class-keyed converter table beyond typed facades.",
                "planned",
            ),
        ):
            if qn == root or qn.startswith(root + "::"):
                selected += 1
                if status == "idiomatic":
                    idiomatic_n += 1
                else:
                    planned_n += 1
                indexed[api_id] = {
                    "api_id": api_id,
                    "status": status,
                    "hitool_symbol": symbol,
                    "test_evidence": evidence,
                    "notes": notes,
                }
                break

    if idiomatic_n < 60:
        raise SystemExit(f"expected >=60 idiomatic convert APIs, got {idiomatic_n} (selected={selected})")

    # Remaining convert.impl / SPI types → planned (merge-only, never delete).
    for row in inventory:
        qn = row["qualified_name"]
        api_id = row["api_id"]
        if not qn.startswith("cn.hutool.core.convert"):
            continue
        if api_id in indexed:
            continue
        selected += 1
        planned_n += 1
        indexed[api_id] = {
            "api_id": api_id,
            "status": "planned",
            "hitool_symbol": "hitool_core::convert",
            "test_evidence": "",
            "notes": (
                "Planned: convert.impl / SPI converter type; common Convert facades "
                "already idiomatic via ConvertValue."
            ),
        }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(
        f"recorded {selected} convert-package APIs "
        f"(idiomatic={idiomatic_n}, planned={planned_n}, merge-only)"
    )


if __name__ == "__main__":
    main()
