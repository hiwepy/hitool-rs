#!/usr/bin/env python3
"""Record reviewed Hutool cron APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
COMPAT = "crates/hitool-cron/src/compat.rs::"
PATTERN = "crates/hitool-cron/src/pattern.rs::"
WHEEL = "crates/hitool-cron/src/timingwheel.rs::"


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("::", 1)[0]


def mapping(qualified_name: str) -> tuple[str, str, str]:
    name = family(qualified_name)
    symbol = f"hitool_cron::{name}"

    if ".timingwheel::" in qualified_name:
        if name in {"TimerTask", "TimerTaskList"}:
            test = "timer_tasks_and_lists_are_one_shot_identity_aware_and_ordered"
        elif name == "TimingWheel":
            test = "timing_wheel_accepts_current_interval_and_flushes_elapsed_buckets"
        else:
            test = "system_timer_requires_explicit_lifecycle_and_runs_pending_tasks"
        return (
            symbol,
            WHEEL + test,
            "An explicitly owned Rust timer and bounded timing wheel preserve delayed-task, bucket, advancement, and lifecycle behavior without hidden global workers.",
        )

    if ".pattern" in qualified_name:
        if name in {"CronPattern", "CronPatternUtil"}:
            test = "patterns_match_advance_and_bound_results"
        elif name in {"CronPatternBuilder", "Part", "PartParser", "PatternParser", "PatternMatcher"}:
            test = "parts_builders_and_parsers_validate_every_field_shape"
        else:
            test = "finite_day_year_and_wildcard_matchers_cover_boundaries"
        return (
            symbol,
            PATTERN + test,
            "The mature cron parser drives five-, six-, and seven-part schedules while the thin compatibility layer adds Hutool alternatives, builders, precision modes, and typed field matchers.",
        )

    if name in {"Task", "RunnableTask", "InvokeTask", "CronTask", "TaskTable"}:
        test = "registry_runnable_cron_tasks_and_tables_are_explicit"
        note = "Rust task traits, injected invocation registries, stable IDs, and insertion-ordered tables replace Java reflection while retaining task-table behavior."
    elif name in {
        "TaskListener",
        "SimpleTaskListener",
        "TaskListenerManager",
        "TaskExecutor",
        "TaskExecutorManager",
    }:
        test = "listeners_executors_and_launchers_report_success_and_failure"
        note = "Thread-safe listener and executor managers report start, success, and typed failure outcomes around real task execution."
    else:
        test = "scheduler_lifecycle_tables_launchers_and_owned_facade_are_bounded"
        note = "The owned Scheduler and CronUtil facade use an injected Tokio runtime, explicit shutdown, non-overlapping scheduling, stable task mutation, and no hidden process-global scheduler."
    return symbol, COMPAT + test, note


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-cron":
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }

    if selected != 208:
        raise SystemExit(f"expected 208 reviewed cron APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool cron APIs")


if __name__ == "__main__":
    main()
