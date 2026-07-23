#!/usr/bin/env python3
"""Record reviewed Hutool core.bean APIs (merge-only).

BeanUtil Serde/From paths → idiomatic.
Java Introspector / reflection field copy → planned or unsafe-to-copy.
Never deletes existing decisions.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

BEAN_UTIL = "cn.hutool.core.bean::BeanUtil"

# Serde / string helpers — idiomatic.
SERDE_METHODS = {
    "",
    "beanToMap",
    "mapToBean",
    "mapToBeanIgnoreCase",
    "fillBeanWithMap",
    "fillBeanWithMapIgnoreCase",
    "toBean",
    "toBeanIgnoreError",
    "toBeanIgnoreCase",
    "copyProperties",
    "copyToList",
    "isEmpty",
    "isNotEmpty",
    "isCommonFieldsEqual",
    "getFieldName",
    "isMatchName",
    "fillBean",  # ValueProvider → Serde map merge when caller supplies map values
}

# Runtime Class / PropertyDescriptor / Field reflection — not safely portable.
UNSAFE_OR_PLANNED = {
    "isReadableBean": "planned",
    "isBean": "planned",
    "hasSetter": "planned",
    "hasGetter": "planned",
    "hasPublicField": "planned",
    "createDynaBean": "planned",
    "findEditor": "planned",
    "getBeanDesc": "unsafe-to-copy",
    "descForEach": "unsafe-to-copy",
    "getPropertyDescriptors": "unsafe-to-copy",
    "getPropertyDescriptorMap": "unsafe-to-copy",
    "getPropertyDescriptor": "unsafe-to-copy",
    "getFieldValue": "unsafe-to-copy",
    "setFieldValue": "unsafe-to-copy",
    "getProperty": "unsafe-to-copy",
    "setProperty": "unsafe-to-copy",
    "edit": "unsafe-to-copy",
    "trimStrFields": "unsafe-to-copy",
    "hasNullField": "unsafe-to-copy",
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
    deferred_n = 0
    symbol = "hutool_core::bean::BeanUtil"
    evidence = "crates/hutool-core/src/bean/bean_util.rs::bean_util_serde_map_copy_and_field_name"
    serde_notes = (
        "Serde Serialize/DeserializeOwned facades cover beanToMap/mapToBean/toBean/"
        "copyProperties/copyToList without JavaBeans Introspector."
    )

    for row in inventory:
        qn = row["qualified_name"]
        if not (qn == BEAN_UTIL or qn.startswith(BEAN_UTIL + "::")):
            # Register other bean package types as planned stubs (merge-only, never delete).
            if qn.startswith("cn.hutool.core.bean::") or qn.startswith("cn.hutool.core.bean."):
                selected += 1
                deferred_n += 1
                name = qn.split("::")[1].split(".")[-1] if "::" in qn else qn
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "planned",
                    "hutool_symbol": "hutool_core::bean",
                    "test_evidence": "",
                    "notes": (
                        f"Planned: {name} depends on JavaBeans Introspector / PropertyEditor / "
                        "copier SPI; use BeanUtil Serde paths instead."
                    ),
                }
            continue

        selected += 1
        name = method_name(qn)
        if name in UNSAFE_OR_PLANNED:
            deferred_n += 1
            status = UNSAFE_OR_PLANNED[name]
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": "",
                "notes": (
                    f"{status}: BeanUtil.{name} requires Java reflection / Introspector; "
                    "Rust uses explicit Serde structs instead of runtime field copy."
                ),
            }
        elif name in SERDE_METHODS:
            idiomatic_n += 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": serde_notes,
            }
        else:
            deferred_n += 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hutool_symbol": symbol,
                "test_evidence": "",
                "notes": f"Planned: BeanUtil.{name} not yet mapped to a Serde/From facade.",
            }

    if idiomatic_n < 15:
        raise SystemExit(f"expected >=15 idiomatic BeanUtil APIs, got {idiomatic_n}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(
        f"recorded {selected} bean-package APIs "
        f"(idiomatic={idiomatic_n}, deferred={deferred_n}, merge-only)"
    )


if __name__ == "__main__":
    main()
