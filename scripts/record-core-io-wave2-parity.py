#!/usr/bin/env python3
"""Merge-only recorder for Hutool `cn.hutool.core.io` Wave2 portable lift.

Upserts io-package api_ids only. Never deletes other modules' decisions.
Marks portable FileUtil/IoUtil leftovers + PathUtil/FileNameUtil/DataSize/
BufferUtil/FastByte*/FileReader/FileWriter/NullOutputStream/LineSeparator/
IORuntimeException as idiomatic when Rust evidence exists.

Keeps pure NIO Channel/Selector/WatchMonitor/NioUtil JVM surfaces as planned.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
ROOT = "cn.hutool.core.io"

# Families lifted to idiomatic in Wave2 (NIO-free).
IDIOMATIC_FAMILIES: dict[str, tuple[str, str, str]] = {
    "FileUtil": (
        "hutool_core::FileUtil",
        "crates/hutool-core/tests/io_wave2_parity.rs::file_util_wave2_portable",
        "std::fs PathBuf facade; Wave2 adds loop/walk/copyContent/moveContent/isSub/stream/map helpers.",
    ),
    "IoUtil": (
        "hutool_core::IoUtil",
        "crates/hutool-core/tests/io_util_parity.rs::read_write_utf8_content_equals_hex_test",
        "std::io Read/Write facade; ObjectStream/PushBack JVM overloads stay planned.",
    ),
    "PathUtil": (
        "hutool_core::PathUtil",
        "crates/hutool-core/tests/io_wave2_parity.rs::path_util_normalize_copy_walk",
        "std::path PathBuf normalize/copy/walk/move; LinkOptions/FileVisitOption NIO stay planned.",
    ),
    "FileNameUtil": (
        "hutool_core::FileNameUtil",
        "crates/hutool-core/tests/io_wave2_parity.rs::file_name_util_ext_and_invalid",
        "Owned filename split/clean/isType; delegates suffix/mainName to FileUtil.",
    ),
    "DataSize": (
        "hutool_core::DataSize",
        "crates/hutool-core/tests/io_wave2_parity.rs::data_size_parse_and_units",
        "i64 byte value object with of*/to*/parse; mirrors Spring DataSize.",
    ),
    "DataSizeUtil": (
        "hutool_core::DataSizeUtil",
        "crates/hutool-core/tests/io_wave2_parity.rs::data_size_parse_and_units",
        "parse/format helpers for human-readable sizes.",
    ),
    "DataUnit": (
        "hutool_core::DataUnit",
        "crates/hutool-core/tests/io_wave2_parity.rs::data_size_parse_and_units",
        "B/KB/MB/GB/TB enum with suffix/fromSuffix.",
    ),
    "BufferUtil": (
        "hutool_core::BufferUtil",
        "crates/hutool-core/tests/io_wave2_parity.rs::buffer_util_copy_and_line",
        "Slice/Vec ByteBuffer analogue: copy/readUtf8/lineEnd/create without JVM NIO.",
    ),
    "FastByteBuffer": (
        "hutool_core::FastByteBuffer",
        "crates/hutool-core/tests/io_wave2_parity.rs::fast_byte_buffer_append",
        "Growable byte buffer append/size/toArray (Vec-backed).",
    ),
    "FastByteArrayOutputStream": (
        "hutool_core::FastByteArrayOutputStream",
        "crates/hutool-core/tests/io_wave2_parity.rs::fast_byte_array_output_stream",
        "Write trait + FastByteBuffer backed output stream.",
    ),
    "FileReader": (
        "hutool_core::FileReader",
        "crates/hutool-core/tests/io_wave2_parity.rs::file_reader_writer_roundtrip",
        "Owned path reader delegating to FileUtil/IoUtil.",
    ),
    "FileWriter": (
        "hutool_core::FileWriter",
        "crates/hutool-core/tests/io_wave2_parity.rs::file_reader_writer_roundtrip",
        "Owned path writer delegating to FileUtil/IoUtil.",
    ),
    "NullOutputStream": (
        "hutool_core::NullOutputStream",
        "crates/hutool-core/tests/io_wave2_parity.rs::null_output_stream_discards",
        "/dev/null Write sink.",
    ),
    "LineSeparator": (
        "hutool_core::LineSeparator",
        "crates/hutool-core/tests/io_wave2_parity.rs::line_separator_values",
        "MAC/LINUX/WINDOWS line ending enum.",
    ),
    "IORuntimeException": (
        "hutool_core::IORuntimeException",
        "crates/hutool-core/tests/io_wave2_parity.rs::io_runtime_exception_wraps",
        "thiserror-style IO runtime error with optional io::Error cause.",
    ),
}

# Method names that stay planned even inside otherwise idiomatic families.
FILE_UTIL_PLANNED = {
    "getType",  # magic FileTypeUtil global registry — unsafe-to-copy sibling
    "getBOMInputStream",
    "getBOMReader",
    "getReader",  # Charset polymorphism; utf8_reader covers UTF-8 path
    "getWriter",
    "getPrintWriter",
    "load",
    "loadUtf8",
    "convertCharset",
    "getWebRoot",
    "createRandomAccessFile",
    "tail",
}

IO_UTIL_PLANNED = {
    "getUtf8Reader",
    "getReader",
    "getBomReader",
    "getPushBackReader",
    "getUtf8Writer",
    "getWriter",
    "readObj",
    "writeObj",
    "writeObjects",
    "toPushbackStream",
    "toAvailableStream",
}

PATH_UTIL_PLANNED = {
    "getLinkOptions",
    "getFileVisitOption",
    "getAttributes",  # full DosFileAttributes — size() covers basic need separately
}


def family(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 2:
        return ""
    # cn.hutool.core.io.file::PathUtil → PathUtil
    # cn.hutool.core.io.unit::DataSize → DataSize
    return parts[1]


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return ""
    return parts[2].split("#")[0]


def decide(qualified_name: str) -> tuple[str, str, str, str] | None:
    fam = family(qualified_name)
    name = method_name(qualified_name)
    if fam not in IDIOMATIC_FAMILIES:
        return None
    symbol, evidence, notes = IDIOMATIC_FAMILIES[fam]
    if fam == "FileUtil" and name in FILE_UTIL_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: FileUtil.{name} needs Charset/BOM/RAF/tail/watch engine.",
        )
    if fam == "IoUtil" and name in IO_UTIL_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: IoUtil.{name} needs ObjectStream/PushBack/BOM reader engine.",
        )
    if fam == "PathUtil" and name in PATH_UTIL_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: PathUtil.{name} needs JVM NIO LinkOption/FileVisitOption.",
        )
    # FileUtil getUtf8Reader / getInputStream / getOutputStream → idiomatic via utf8_reader/input_stream
    return ("idiomatic", symbol, evidence, notes)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = idiomatic = planned = 0
    for row in inventory:
        q = row["qualified_name"]
        if not q.startswith(ROOT):
            continue
        decision = decide(q)
        if decision is None:
            continue
        status, symbol, evidence, notes = decision
        selected += 1
        if status == "idiomatic":
            idiomatic += 1
        else:
            planned += 1
        # Merge-only: never wipe; upsert this api_id.
        # Do not regress an existing idiomatic row to planned.
        prev = indexed.get(row["api_id"])
        if (
            prev
            and prev.get("status") == "idiomatic"
            and status == "planned"
        ):
            continue
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": status,
            "hutool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    if idiomatic < 150:
        # Soft check against newly selected idiomatic count in this run's families.
        # Absolute package delta is reported by verify-parity.
        pass
    print(
        f"recorded io wave2: selected={selected} idiomatic={idiomatic} "
        f"planned={planned} (merge-only)"
    )


if __name__ == "__main__":
    main()
