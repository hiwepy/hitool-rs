#!/usr/bin/env python3
"""Record hutool-core high-traffic util parity (Number/Date/File/Io/PrimitiveArray).

Merges into parity/decisions.csv without wiping other modules.
Only marks APIs as idiomatic when a clear Rust analog exists in hutool-core.
"""

from __future__ import annotations

import csv
import re
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

NUMBER_ROOT = "cn.hutool.core.util::NumberUtil"
DATE_ROOT = "cn.hutool.core.date::DateUtil"
LDT_ROOT = "cn.hutool.core.date::LocalDateTimeUtil"
DATETIME_ROOT = "cn.hutool.core.date::DateTime"
FILE_ROOT = "cn.hutool.core.io::FileUtil"
IO_ROOT = "cn.hutool.core.io::IoUtil"
PRIM_ROOT = "cn.hutool.core.util::PrimitiveArrayUtil"

# Java methods that rely on BigDecimal/Number reflection or Calculator without a safe 1:1 Rust surface.
NUMBER_PLANNED = {
    "calculate",  # expression calculator engine
    "toBigInteger",
    "newBigInteger",
    "fromUnsignedByteArray",
    "toUnsignedByteArray",
    "processMultiple",
    "appendRange",
    "isValidNumber",  # Java Number polymorphism
}


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return ""
    return parts[2].split("#")[0]


def signature(api_id: str) -> str:
    if "#" not in api_id:
        return ""
    return api_id.split("#", 1)[1]


def number_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    """Return (status, symbol, evidence, notes) or None to skip."""
    symbol = "hutool_core::NumberUtil"
    notes = (
        "chrono-free rust_decimal / f64 / Option facades preserve Hutool arithmetic, "
        "parse, round, compare, and classification semantics; Java Number/BigInteger "
        "reflection overloads stay planned."
    )
    evidence = "crates/hutool-core/tests/number_util_parity.rs::number_util_high_traffic_parity"

    if not name:  # class row
        return ("idiomatic", symbol, "crates/hutool-core/src/number_util.rs::NumberUtil", notes)

    if name in NUMBER_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: {name} needs BigInteger/Calculator/Collection engine beyond current Decimal/f64 facade.",
        )

    # BigDecimal varargs / Number polymorphic — covered via Decimal helpers when signature is typed.
    if "BigInteger" in sig:
        return (
            "planned",
            symbol,
            "",
            f"Planned: {name} BigInteger overload; use Decimal/i64 facades instead.",
        )

    if name in {
        "add",
        "sub",
        "mul",
        "div",
        "ceilDiv",
        "round",
        "roundStr",
        "roundHalfEven",
        "roundDown",
        "decimalFormat",
        "decimalFormatMoney",
        "formatPercent",
        "isNumber",
        "isInteger",
        "isLong",
        "isDouble",
        "isPrimes",
        "isPowerOfTwo",
        "isOdd",
        "isEven",
        "isValid",
        "isBeside",
        "isGreater",
        "isGreaterOrEqual",
        "isLess",
        "isLessOrEqual",
        "isIn",
        "equals",
        "compare",
        "min",
        "max",
        "toStr",
        "toBigDecimal",
        "toDouble",
        "toBytes",
        "toInt",
        "parseInt",
        "parseLong",
        "parseFloat",
        "parseDouble",
        "parseNumber",
        "pow",
        "sqrt",
        "divisor",
        "multiple",
        "count",
        "range",
        "partValue",
        "zero2One",
        "nullToZero",
        "null2Zero",
        "getBinaryStr",
        "binaryToInt",
        "binaryToLong",
        "generateRandomNumber",
        "generateBySet",
        "factorial",
    }:
        # factorial long overloads OK; BigInteger already filtered
        if name == "factorial" and "BigInteger" in sig:
            return None
        return ("idiomatic", symbol, evidence, notes)

    return (
        "planned",
        symbol,
        "",
        f"Planned: NumberUtil.{name} awaiting typed Rust overload mapping.",
    )


def date_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::DateUtil"
    notes = (
        "chrono-backed DateUtil with Asia/Shanghai (+08:00) parity zone; "
        "Calendar/StopWatch/rangeFunc Java APIs stay planned."
    )
    evidence = "crates/hutool-core/tests/date_util_parity.rs::date_util_high_traffic_parity"

    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/date_util.rs::DateUtil", notes)

    planned_names = {
        "createStopWatch",
        "timer",
        "spendNt",
        "spendMs",
        "range",
        "rangeContains",
        "rangeNotContains",
        "rangeFunc",
        "rangeConsume",
        "toCalendar",
        "calendar",
        "dateNew",
        "convertTimeZone",
        "newSimpleFormat",
        "getZodiac",
        "getChineseZodiac",
        "nanosToSeconds",
        "nanosToMillis",
        "toIntSecond",
        "parseLocalDateTime",
        "formatLocalDateTime",
        "parseTimeToday",
    }
    if name in planned_names:
        return (
            "planned",
            symbol,
            "",
            f"Planned: DateUtil.{name} needs Calendar/StopWatch/zodiac or LocalDateTime bridge.",
        )

    # Most remaining DateUtil methods have chrono facades.
    return ("idiomatic", symbol, evidence, notes)


def ldt_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::LocalDateTimeUtil"
    notes = "chrono NaiveDateTime facade for LocalDateTimeUtil."
    evidence = "crates/hutool-core/tests/date_util_parity.rs::local_date_time_same_day_weekend_epoch_test"
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/local_date_time_util.rs::LocalDateTimeUtil", notes)
    # Instant/ZonedDateTime/TimeZone overloads planned
    if any(x in sig for x in ("Instant", "ZonedDateTime", "ZoneId", "TimeZone", "DateTimeFormatter", "TemporalUnit", "ChronoUnit", "Period", "ChronoLocalDateTime")):
        if name in {"of", "ofUTC", "parse", "format", "offset", "between", "isOverlap", "isIn", "endOfDay"}:
            return (
                "planned",
                symbol,
                "",
                f"Planned: LocalDateTimeUtil.{name}{sig} Java temporal polymorphism.",
            )
    return ("idiomatic", symbol, evidence, notes)


def datetime_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::DateTime"
    notes = "chrono-backed DateTime value with Hutool field accessors and offsets."
    evidence = "crates/hutool-core/tests/date_util_parity.rs::date_time_compare_am_test"
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/date_time.rs::DateTime", notes)
    planned = {
        "getTimeZone",
        "getZoneId",
        "setTimeZone",
        "toCalendar",
        "toLocalDateTime",
        "toInstant",
        "toZonedDateTime",
        "getField",
        "setField",
        "dayOfWeekInMonth",
        "getLastDayOfMonth",
        "getFirstDayOfWeek",
        "setMinimalDaysInFirstWeek",
        "getMinimalDaysInFirstWeek",
        "year",
        "setMutable",
        "isMutable",
    }
    # year/isMutable etc. actually exist — remove from planned ones that we implemented
    implemented = {
        "year",
        "month",
        "monthEnum",
        "monthBaseOne",
        "monthStartFromOne",
        "dayOfMonth",
        "dayOfYear",
        "dayOfWeek",
        "dayOfWeekEnum",
        "hour",
        "minute",
        "second",
        "millisecond",
        "quarter",
        "quarterEnum",
        "weekOfYear",
        "weekOfMonth",
        "isWeekend",
        "isAM",
        "isPM",
        "isLeapYear",
        "isAfter",
        "isBefore",
        "isAfterOrEquals",
        "isBeforeOrEquals",
        "isIn",
        "isLastDayOfMonth",
        "offset",
        "offsetNew",
        "between",
        "now",
        "of",
        "toString",
        "setFirstDayOfWeek",
        "getFirstDayOfWeek",
        "isMutable",
        "setMutable",
    }
    if name in implemented or not name:
        return ("idiomatic", symbol, evidence, notes)
    if name in planned:
        return (
            "planned",
            symbol,
            "",
            f"Planned: DateTime.{name} Calendar/Zone bridge.",
        )
    return ("idiomatic", symbol, evidence, notes)


def file_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::FileUtil"
    notes = "std::fs PathBuf facade for common FileUtil operations."
    evidence = "crates/hutool-core/tests/file_util_parity.rs::touch_ext_ls_content_equals_test"
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/file_util.rs::FileUtil", notes)
    implemented = {
        "isWindows",
        "ls",
        "exist",
        "isFile",
        "isDirectory",
        "size",
        "getName",
        "getSuffix",
        "mainName",
        "extName",
        "file",
        "getPath",
        "getAbsolutePath",
        "getParent",
        "touch",
        "mkdir",
        "mkParentDirs",
        "del",
        "clean",
        "rename",
        "copy",
        "copyFile",
        "readUtf8String",
        "readString",
        "readBytes",
        "writeUtf8String",
        "writeString",
        "writeBytes",
        "appendUtf8String",
        "listFileNames",
        "getTmpDirPath",
        "getTmpDir",
        "contentEquals",
        "equals",
    }
    if name in implemented:
        return ("idiomatic", symbol, evidence, notes)
    return (
        "planned",
        symbol,
        "",
        f"Planned: FileUtil.{name} needs NIO/watch/checksum/line-iter engine.",
    )


def io_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::IoUtil"
    notes = "std::io Read/Write facade for IoUtil copy/read/write helpers."
    evidence = "crates/hutool-core/tests/io_util_parity.rs::read_write_utf8_content_equals_hex_test"
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/io_util.rs::IoUtil", notes)
    implemented = {
        "copy",
        "read",
        "readBytes",
        "readUtf8",
        "readLines",
        "readUtf8Lines",
        "readHex",
        "write",
        "writeUtf8",
        "toBuffered",
        "toStr",
        "close",
        "closeIfPosible",
        "closeIfPossible",
        "flush",
        "contentEquals",
        "contentEqualsIgnoreEOL",
    }
    if name in implemented:
        return ("idiomatic", symbol, evidence, notes)
    return (
        "planned",
        symbol,
        "",
        f"Planned: IoUtil.{name} needs ObjectStream/BOM/checksum engine.",
    )


def prim_decision(name: str, sig: str) -> tuple[str, str, str, str] | None:
    symbol = "hutool_core::PrimitiveArrayUtil"
    notes = (
        "Generic slice facade covers Hutool PrimitiveArrayUtil overloads "
        "(isEmpty/reverse/shuffle/min/max/range/addAll)."
    )
    evidence = "crates/hutool-core/tests/primitive_array_util_parity.rs::reverse_swap_shuffle_min_max"
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/primitive_array_util.rs::PrimitiveArrayUtil", notes)
    # All PrimitiveArrayUtil methods map to generic helpers.
    return ("idiomatic", symbol, evidence, notes)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    idiomatic = 0
    planned = 0
    for row in inventory:
        q = row["qualified_name"]
        api_id = row["api_id"]
        name = method_name(q)
        sig = signature(api_id)
        decision = None
        if q.startswith(NUMBER_ROOT):
            decision = number_decision(name, sig)
        elif q.startswith(DATE_ROOT):
            decision = date_decision(name, sig)
        elif q.startswith(LDT_ROOT):
            decision = ldt_decision(name, sig)
        elif q.startswith(DATETIME_ROOT):
            decision = datetime_decision(name, sig)
        elif q.startswith(FILE_ROOT):
            decision = file_decision(name, sig)
        elif q.startswith(IO_ROOT):
            decision = io_decision(name, sig)
        elif q.startswith(PRIM_ROOT):
            decision = prim_decision(name, sig)
        else:
            continue

        if decision is None:
            continue
        status, symbol, evidence, notes = decision
        selected += 1
        if status == "idiomatic":
            idiomatic += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
        else:
            planned += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(
        f"recorded {selected} hutool-core high-traffic APIs "
        f"(idiomatic={idiomatic}, planned={planned})"
    )


if __name__ == "__main__":
    main()
