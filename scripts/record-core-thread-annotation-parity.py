#!/usr/bin/env python3
"""Record hutool-core `cn.hutool.core.thread` + `cn.hutool.core.annotation` Wave-2 parity (merge-only).

Never deletes existing decisions. Only upserts api_ids under thread / annotation.
- thread: portable ExecutorBuilder / ThreadUtil / ConcurrencyTester / lock / async → idiomatic;
  JVM ThreadLocal globals + ThreadGroup → planned.
- annotation: AnnotationUtil stays unsafe-to-copy; non-reflective helpers / mirror model → idiomatic;
  dynamic proxy stubs stay planned.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

THREAD_PREFIX = "cn.hutool.core.thread"
ANNOTATION_PREFIX = "cn.hutool.core.annotation"

# Thread classes that stay planned (JVM ThreadLocal / ThreadGroup / WeakSafe internals).
THREAD_PLANNED_CLASSES = {
    "NamedInheritableThreadLocal",
    "NamedThreadLocal",
}

THREAD_PLANNED_METHODS = {
    "ThreadUtil": {
        "createThreadLocal",
        "getThreads",
        "getMainThread",
        "currentThreadGroup",
        "newCompletionService",
        # Object monitor wait — Condvar approx exists as sync()/notify_sync but Object identity differs
    },
}

# Annotation: keep reflective util + dynamic proxies planned/unsafe.
ANNOTATION_UNSAFE_CLASSES = {
    "AnnotationUtil",
}

ANNOTATION_PLANNED_CLASSES = {
    "AnnotationProxy",
    "SynthesizedAnnotationProxy",
    "AbstractAnnotationSynthesizer",
}


def class_name(api_id: str) -> str:
    rest = api_id.split("::", 1)[1] if "::" in api_id else api_id
    return rest.split("::", 1)[0].rstrip("#").split(".")[-1]


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    if len(parts) < 3:
        return ""
    return parts[2].split("#")[0]


def snake(name: str) -> str:
    out: list[str] = []
    for i, ch in enumerate(name):
        if ch.isupper() and i > 0:
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


def thread_symbol(api_id: str, cls: str) -> str:
    if "thread.lock" in api_id or ".lock." in api_id:
        return f"hitool_core::thread::lock::{snake(cls)}"
    if "threadlocal" in api_id.lower():
        return f"hitool_core::thread::threadlocal::{snake(cls)}"
    return f"hitool_core::thread::{snake(cls)}"


def annotation_symbol(cls: str) -> str:
    if cls.endswith("Scanner") or cls.startswith("AbstractType") or cls in {
        "AnnotationScanner",
        "EmptyAnnotationScanner",
        "ElementAnnotationScanner",
        "FieldAnnotationScanner",
        "MethodAnnotationScanner",
        "MetaAnnotationScanner",
        "TypeAnnotationScanner",
        "GenericAnnotationScanner",
    }:
        return f"hitool_core::annotation::scanner::{snake(cls)}"
    return f"hitool_core::annotation::{snake(cls)}"


def decide_thread(api_id: str, qualified_name: str) -> tuple[str, str, str, str]:
    cls = class_name(api_id)
    name = method_name(qualified_name)
    symbol = thread_symbol(api_id, cls)

    if "threadlocal" in api_id.lower() or cls in THREAD_PLANNED_CLASSES:
        return (
            "planned",
            symbol,
            "",
            "Planned: JVM ThreadLocal / inheritable ThreadLocal globals — use std::thread_local! / task-local instead.",
        )

    if "WeakSafe" in api_id or "WeakSafe" in qualified_name:
        return (
            "planned",
            symbol,
            "",
            "Planned: Java WeakReference segment lock internals — Rust SegmentLock uses strong Arc.",
        )

    if name in THREAD_PLANNED_METHODS.get(cls, set()):
        return (
            "planned",
            symbol,
            "",
            f"Planned: ThreadUtil.{name} needs JVM ThreadGroup/ThreadLocal/CompletionService surface.",
        )

    if "ThreadLocal" in name or "ThreadGroup" in api_id:
        return (
            "planned",
            symbol,
            "",
            "Planned: JVM ThreadLocal/ThreadGroup API.",
        )

    evidence = "crates/hitool-core/tests/thread_parity.rs"
    if cls == "AsyncUtil":
        evidence = "crates/hitool-core/tests/async_util_parity.rs"
        notes = (
            "Idiomatic Tokio JoinHandle waitAll/waitAny/get behind feature `async`; "
            "maps CompletableFuture to JoinHandle/Future."
        )
    elif cls in {"ThreadUtil", "ExecutorBuilder", "ConcurrencyTester", "GlobalThreadPool"}:
        notes = (
            f"Idiomatic std::thread facade for Hutool `{cls}`; "
            "ExecutorBuilder builds SimpleExecutor; ThreadLocal/ThreadGroup stay planned."
        )
    elif "lock" in api_id:
        notes = (
            f"Idiomatic parking_lot / NoLock / SegmentLock facade for Hutool `{cls}`."
        )
    else:
        notes = f"Idiomatic Rust facade for Hutool `{cls}` under cn.hutool.core.thread."

    return ("idiomatic", symbol, evidence, notes)


def decide_annotation(api_id: str, qualified_name: str, existing: dict[str, str] | None) -> tuple[str, str, str, str]:
    cls = class_name(api_id)
    symbol = annotation_symbol(cls)

    # Preserve / enforce AnnotationUtil unsafe-to-copy
    if cls in ANNOTATION_UNSAFE_CLASSES or "AnnotationUtil" in api_id:
        status = "unsafe-to-copy"
        if existing and existing.get("status") == "unsafe-to-copy":
            status = "unsafe-to-copy"
        return (
            status,
            "hitool_core::annotation::AnnotationUtil",
            "crates/hitool-core/tests/annotation_parity.rs",
            "[reflection] AnnotationUtil mirrors JVM reflective annotation lookup; "
            "Rust ElementHandle/AnnotationMirror model is available but decision stays unsafe-to-copy per Wave-2 policy.",
        )

    if cls in ANNOTATION_PLANNED_CLASSES or "Proxy" in cls:
        return (
            "planned",
            symbol,
            "",
            f"Planned: `{cls}` needs JVM dynamic proxy / synthesizer runtime; mirror attribute model covers portable subset.",
        )

    evidence = "crates/hitool-core/tests/annotation_parity.rs"
    if "scanner" in api_id or cls.endswith("Scanner"):
        evidence = "crates/hitool-core/tests/annotation_parity_gap.rs"
    if cls in {
        "Alias",
        "AliasFor",
        "ForceAliasFor",
        "MirrorFor",
        "PropIgnore",
        "RelationType",
        "Link",
    }:
        evidence = "crates/hitool-core/tests/annotation_helpers_parity.rs"
    notes = (
        f"Idiomatic non-reflective annotation helper `{cls}` via AnnotationMirror/ElementHandle; "
        "no JVM Class/AnnotatedElement reflection."
    )
    return ("idiomatic", symbol, evidence, notes)


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    counts: dict[str, dict[str, int]] = {
        "thread": {"idiomatic": 0, "planned": 0, "unsafe-to-copy": 0},
        "annotation": {"idiomatic": 0, "planned": 0, "unsafe-to-copy": 0},
    }

    for row in inventory:
        api_id = row["api_id"]
        if api_id.startswith(THREAD_PREFIX):
            pkg = "thread"
            status, symbol, evidence, notes = decide_thread(api_id, row["qualified_name"])
        elif api_id.startswith(ANNOTATION_PREFIX):
            pkg = "annotation"
            status, symbol, evidence, notes = decide_annotation(
                api_id, row["qualified_name"], indexed.get(api_id)
            )
        else:
            continue

        selected += 1
        counts[pkg][status] = counts[pkg].get(status, 0) + 1
        indexed[api_id] = {
            "api_id": api_id,
            "status": status,
            "hitool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(
        f"recorded {selected} thread+annotation APIs (merge-only): "
        f"thread={counts['thread']} annotation={counts['annotation']}"
    )


if __name__ == "__main__":
    main()
