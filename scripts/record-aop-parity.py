#!/usr/bin/env python3
"""Record reviewed Hutool AOP APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
FAMILIES = {
    "ProxyUtil": (
        "hitool_aop::proxy::ProxyUtil",
        "methods_and_handler_proxies_are_typed_and_explicit",
    ),
    "Aspect": (
        "hitool_aop::aspects::Aspect",
        "simple_and_timing_aspects_preserve_hutool_defaults",
    ),
    "SimpleAspect": (
        "hitool_aop::aspects::SimpleAspect",
        "simple_and_timing_aspects_preserve_hutool_defaults",
    ),
    "TimeIntervalAspect": (
        "hitool_aop::aspects::TimeIntervalAspect",
        "simple_and_timing_aspects_preserve_hutool_defaults",
    ),
    "CglibInterceptor": (
        "hitool_aop::interceptor::CglibInterceptor",
        "cglib_variants_run_after_when_before_rejects",
    ),
    "JdkInterceptor": (
        "hitool_aop::interceptor::JdkInterceptor",
        "jdk_interceptor_matches_callback_and_suppression_semantics",
    ),
    "SpringCglibInterceptor": (
        "hitool_aop::interceptor::SpringCglibInterceptor",
        "cglib_variants_run_after_when_before_rejects",
    ),
    "CglibProxyFactory": (
        "hitool_aop::proxy::CglibProxyFactory",
        "factories_select_all_backends_and_facade_overloads",
    ),
    "JdkProxyFactory": (
        "hitool_aop::proxy::JdkProxyFactory",
        "factories_select_all_backends_and_facade_overloads",
    ),
    "ProxyFactory": (
        "hitool_aop::proxy::ProxyFactory",
        "factories_select_all_backends_and_facade_overloads",
    ),
    "SpringCglibProxyFactory": (
        "hitool_aop::proxy::SpringCglibProxyFactory",
        "factories_select_all_backends_and_facade_overloads",
    ),
}


def family(qualified_name: str) -> str | None:
    if "::" not in qualified_name:
        return None
    package, remainder = qualified_name.split("::", 1)
    if not package.startswith("cn.hutool.aop"):
        return None
    candidate = remainder.split("::", 1)[0]
    return candidate if candidate in FAMILIES else None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-aop":
            continue
        aop_family = family(row["qualified_name"])
        if aop_family is None:
            raise SystemExit(f"unmapped Hutool AOP family: {row['qualified_name']}")
        selected += 1
        symbol, test = FAMILIES[aop_family]
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": f"crates/hitool-aop/src/aop_tests.rs::{test}",
            "notes": (
                "JVM reflection proxies map to explicit typed Rust wrappers while preserving "
                "Hutool before/after/error propagation and JDK/CGLIB callback ordering."
            ),
        }

    if selected != 37:
        raise SystemExit(f"expected 37 reviewed AOP APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool AOP APIs")


if __name__ == "__main__":
    main()
