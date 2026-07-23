#!/usr/bin/env python3
"""YOLO Wave2 merge-only recorder: core.util leftovers + core.date StopWatch/range/formatters.

Lifts planned → idiomatic for portable ArrayUtil / ObjectUtil / RandomUtil / ReUtil
surfaces and chrono-backed date StopWatch / DateRange / BetweenFormatter / DatePattern /
TimeInterval / GroupTimeInterval / Temporal* / related DateUtil helpers.

Does NOT wipe other modules. Never regresses already-idiomatic rows.
Leaves ReflectUtil / ClassUtil as unsafe-to-copy. Leaves ZipUtil JVM ZipFile handles planned.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

ARRAY_ROOT = "cn.hutool.core.util::ArrayUtil"
OBJECT_ROOT = "cn.hutool.core.util::ObjectUtil"
RANDOM_ROOT = "cn.hutool.core.util::RandomUtil"
RE_ROOT = "cn.hutool.core.util::ReUtil"
ZIP_ROOT = "cn.hutool.core.util::ZipUtil"

STOP_ROOT = "cn.hutool.core.date::StopWatch"
RANGE_ROOT = "cn.hutool.core.date::DateRange"
BETWEEN_ROOT = "cn.hutool.core.date::BetweenFormatter"
PATTERN_ROOT = "cn.hutool.core.date::DatePattern"
TI_ROOT = "cn.hutool.core.date::TimeInterval"
GTI_ROOT = "cn.hutool.core.date::GroupTimeInterval"
TEMP_ROOT = "cn.hutool.core.date::TemporalUtil"
TEMPA_ROOT = "cn.hutool.core.date::TemporalAccessorUtil"
DATE_UTIL_ROOT = "cn.hutool.core.date::DateUtil"

# Object / Class reflective ArrayUtil overloads stay planned.
ARRAY_PLANNED_SIG_MARKERS = (
    "(Object array",
    "(Object array,",
    "Class<?>",
    "Class<T>",
    "Class<R>",
    "Iterator<T>",
    "Iterable<T>",
    "Collection<T>",
)

OBJECT_PLANNED = {
    "serialize",
    "deserialize",
    "cloneByStream",
    "getTypeArgument",
}

RANDOM_PLANNED = {
    "getRandom",
    "createSecureRandom",
    "getSecureRandom",
    "getSHA1PRNGRandom",
    "getSecureRandomStrong",
    "randomDate",  # needs DateTime + LocalDateTime coupling beyond day offset
}

RE_PLANNED = {
    "extractMultiAndDelPre",  # Mutable CharSequence holder
}

DATE_UTIL_IDIOMATIC = {
    "createStopWatch",
    "timer",
    "range",
    "rangeContains",
    "rangeNotContains",
    "rangeFunc",
    "rangeConsume",
    "spendNt",
    "spendMs",
    "toIntSecond",
    "nanosToMillis",
    "nanosToSeconds",
    "getZodiac",
    "getChineseZodiac",
    "formatLocalDateTime",
    "parseLocalDateTime",
}

DATE_UTIL_PLANNED = {
    "dateNew",
    "parseTimeToday",
    "convertTimeZone",
    "newSimpleFormat",
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


def array_decision(name: str, sig: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::ArrayUtil"
    evidence = "crates/hutool-core/tests/array_util_parity.rs::wave2_array_util_portable_parity"
    notes = (
        "Owned slice/Vec facades cover Hutool ArrayUtil portable overloads; "
        "Class/Object reflective array engines remain planned."
    )
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/array_util.rs::ArrayUtil", notes)
    if any(m in sig for m in ARRAY_PLANNED_SIG_MARKERS):
        return (
            "planned",
            symbol,
            "",
            f"Planned: ArrayUtil.{name} Object/Class reflective overload needs runtime array meta.",
        )
    return ("idiomatic", symbol, evidence, notes)


def object_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::ObjectUtil"
    evidence = "crates/hutool-core/tests/object_util_parity.rs::wave2_object_util_portable_parity"
    notes = (
        "Option/trait facades preserve Hutool null/empty/equals/length/contains semantics; "
        "Java serialization and TypeArgument reflection stay planned."
    )
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/object_util.rs::ObjectUtil", notes)
    if name in OBJECT_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: ObjectUtil.{name} needs Java serialization/reflection engine.",
        )
    return ("idiomatic", symbol, evidence, notes)


def random_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::RandomUtil"
    evidence = "crates/hutool-core/tests/random_util_parity.rs::wave2_random_util_portable_parity"
    notes = (
        "rand + rust_decimal facades cover Hutool numeric/string/collection random APIs; "
        "SecureRandom/ThreadLocalRandom handles stay planned."
    )
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/random_util.rs::RandomUtil", notes)
    if name in RANDOM_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: RandomUtil.{name} needs SecureRandom / full DateTime random engine.",
        )
    return ("idiomatic", symbol, evidence, notes)


def re_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::ReUtil"
    evidence = "crates/hutool-core/tests/re_util_parity.rs::wave2_re_util_portable_parity"
    notes = (
        "regex crate facade consolidates Hutool ReUtil get/find/del/replace/escape overloads "
        "with Pattern/String dual entry points."
    )
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/re_util.rs::ReUtil", notes)
    if name in RE_PLANNED:
        return (
            "planned",
            symbol,
            "",
            f"Planned: ReUtil.{name} needs Mutable CharSequence holder semantics.",
        )
    return ("idiomatic", symbol, evidence, notes)


def stop_decision(name: str, family: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::{StopWatch,TaskInfo}"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_stop_watch_parity"
    notes = (
        "std::time::Instant Spring-style StopWatch with TaskInfo, prettyPrint, and TimeUnit totals."
    )
    if family == "TaskInfo" or name.startswith("TaskInfo") or "::TaskInfo" in family:
        pass
    if not name or name == "TaskInfo":
        return ("idiomatic", symbol, "crates/hutool-core/src/date/stop_watch.rs::StopWatch", notes)
    return ("idiomatic", symbol, evidence, notes)


def date_range_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::DateRange"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::date_range_issue_3783_test_2"
    notes = "Chrono DateTime offset stepping with includeStart/includeEnd and step<=0 issue#3783."
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/date_range.rs::DateRange", notes)
    return ("idiomatic", symbol, evidence, notes)


def between_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::{BetweenFormatter,BetweenFormatterLevel}"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::between_formatter_format_test_2"
    notes = "Owned BetweenFormatter with Level units, max-count, separator, and Display."
    if not name or name == "Level":
        return (
            "idiomatic",
            symbol,
            "crates/hutool-core/src/date/between_formatter.rs::BetweenFormatter",
            notes,
        )
    return ("idiomatic", symbol, evidence, notes)


def pattern_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::DatePattern"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_date_pattern_formatter"
    notes = "Java SimpleDateFormat subset → chrono strftime via DatePattern::create_formatter."
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/date_pattern.rs::DatePattern", notes)
    return ("idiomatic", symbol, evidence, notes)


def time_interval_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::TimeInterval"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_time_interval_parity"
    notes = "Instant-backed TimeInterval with ms/nano modes and BetweenFormatter pretty output."
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/time_interval.rs::TimeInterval", notes)
    return ("idiomatic", symbol, evidence, notes)


def group_time_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::GroupTimeInterval"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_group_time_interval_parity"
    notes = "HashMap<id, Instant> group timer with DateUnit interval helpers."
    if not name:
        return (
            "idiomatic",
            symbol,
            "crates/hutool-core/src/date/group_time_interval.rs::GroupTimeInterval",
            notes,
        )
    return ("idiomatic", symbol, evidence, notes)


def temporal_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::TemporalUtil"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_temporal_util_parity"
    notes = "chrono Duration/offset/weekday helpers for Hutool TemporalUtil portable subset."
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/date/temporal_util.rs::TemporalUtil", notes)
    return ("idiomatic", symbol, evidence, notes)


def temporal_acc_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::TemporalAccessorUtil"
    evidence = "crates/hutool-core/tests/date_types_parity.rs::wave2_temporal_accessor_parity"
    notes = "chrono NaiveDateTime field/format/isIn facades for TemporalAccessorUtil."
    if not name:
        return (
            "idiomatic",
            symbol,
            "crates/hutool-core/src/date/temporal_accessor_util.rs::TemporalAccessorUtil",
            notes,
        )
    return ("idiomatic", symbol, evidence, notes)


def date_util_leftover(name: str) -> tuple[str, str, str, str] | None:
    """Only upgrade Wave2-owned DateUtil leftovers; leave already-idiomatic alone."""
    if name in DATE_UTIL_IDIOMATIC:
        return (
            "idiomatic",
            "hutool_core::DateUtil",
            "crates/hutool-core/tests/date_types_parity.rs::wave2_date_util_timer_range_parity",
            "Chrono/StopWatch/DateRange/Zodiac facades for remaining portable DateUtil helpers.",
        )
    if name in DATE_UTIL_PLANNED:
        return (
            "planned",
            "hutool_core::DateUtil",
            "",
            f"Planned: DateUtil.{name} needs ZoneId/SimpleDateFormat JVM engine.",
        )
    return None


def decide(api_id: str) -> tuple[str, str, str, str] | None:
    if api_id.startswith(ARRAY_ROOT):
        return array_decision(method_name(api_id), signature(api_id))
    if api_id.startswith(OBJECT_ROOT):
        return object_decision(method_name(api_id))
    if api_id.startswith(RANDOM_ROOT):
        return random_decision(method_name(api_id))
    if api_id.startswith(RE_ROOT):
        return re_decision(method_name(api_id))
    if api_id.startswith(STOP_ROOT):
        # TaskInfo nested: cn.hutool.core.date::StopWatch::TaskInfo::getTaskName
        return stop_decision(method_name(api_id), api_id)
    if api_id.startswith(RANGE_ROOT):
        return date_range_decision(method_name(api_id))
    if api_id.startswith(BETWEEN_ROOT):
        return between_decision(method_name(api_id))
    if api_id.startswith(PATTERN_ROOT):
        return pattern_decision(method_name(api_id))
    if api_id.startswith(TI_ROOT):
        return time_interval_decision(method_name(api_id))
    if api_id.startswith(GTI_ROOT):
        return group_time_decision(method_name(api_id))
    if api_id.startswith(TEMP_ROOT):
        return temporal_decision(method_name(api_id))
    if api_id.startswith(TEMPA_ROOT):
        return temporal_acc_decision(method_name(api_id))
    if api_id.startswith(DATE_UTIL_ROOT):
        return date_util_leftover(method_name(api_id))
    # Zip leftovers stay planned (already recorded); do not touch.
    if api_id.startswith(ZIP_ROOT):
        return None
    return None


def main() -> None:
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))

    selected = idiomatic = planned = skipped_protected = 0
    by_family: dict[str, int] = {}

    for row in inventory:
        api_id = row["api_id"]
        decision = decide(api_id)
        if decision is None:
            continue

        # Never regress existing idiomatic.
        existing = indexed.get(api_id)
        if existing and existing.get("status") == "idiomatic" and decision[0] != "idiomatic":
            skipped_protected += 1
            continue

        status, symbol, evidence, notes = decision
        selected += 1
        family = api_id.split("::")[1].split("#")[0]
        by_family[family] = by_family.get(family, 0) + 1
        if status == "idiomatic":
            idiomatic += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": "idiomatic",
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
        else:
            planned += 1
            indexed[api_id] = {
                "api_id": api_id,
                "status": "planned",
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(
        f"wave2 util+date merge-only: selected={selected} "
        f"(idiomatic={idiomatic}, planned={planned}, protected_skip={skipped_protected}) "
        f"families={by_family}"
    )


if __name__ == "__main__":
    main()
