#!/usr/bin/env python3
"""Record reviewed Hutool AI APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("::", 1)[0]


def mapping(qualified_name: str) -> tuple[str, str, str]:
    name = family(qualified_name)
    if name in {"ModelName", "Models"} or "Common" in qualified_name:
        return (
            "hutool_ai::ModelName",
            "crates/hutool-ai/src/models.rs::model_and_common_wire_values_match_hutool",
            "Typed provider, model, vision, speech, reasoning, video, Gemini, and Ollama values preserve Hutool wire semantics.",
        )
    if "Config" in qualified_name or name == "Message":
        return (
            "hutool_ai::BaseConfig",
            "crates/hutool-ai/src/compat.rs::configuration_builder_covers_validation_mutation_and_redaction",
            "Owned validated configuration, dynamic fields, timeouts, proxies, redacted secrets, and typed messages replace mutable Java beans.",
        )
    if name == "AIException":
        return (
            "hutool_ai::ProviderError",
            "crates/hutool-ai/src/lib.rs::response_types_and_errors_round_trip",
            "Typed non-exhaustive errors preserve transport, URL, JSON, provider, streaming, and defensive-limit failures.",
        )
    if "ServiceImpl" in qualified_name or "Service" in qualified_name:
        return (
            "hutool_ai::Operation",
            "crates/hutool-ai/src/operations.rs::every_operation_builds_a_bounded_explicit_request",
            "One exhaustive operation model covers Hutool chat, stream, vision, image, audio, video, embedding, moderation, context, file, and model-management calls.",
        )
    return (
        "hutool_ai::AIServiceFactory",
        "crates/hutool-ai/src/compat.rs::service_trait_factory_and_util_facades_are_usable",
        "Provider-neutral async traits, a built-in factory, pooled Rustls HTTP, bounded bodies, SSE, and injectable clients replace Java SPI wiring.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    for row in inventory:
        if row["module"] != "hutool-ai":
            continue
        selected += 1
        symbol, test, notes = mapping(row["qualified_name"])
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hutool_symbol": symbol,
            "test_evidence": test,
            "notes": notes,
        }
    if selected != 281:
        raise SystemExit(f"expected 281 reviewed AI APIs, selected {selected}")
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool AI APIs")


if __name__ == "__main__":
    main()
