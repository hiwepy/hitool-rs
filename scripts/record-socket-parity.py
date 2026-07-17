#!/usr/bin/env python3
"""Record reviewed Hutool Socket APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path


INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def mapping(qualified_name: str) -> tuple[str, str] | None:
    base = "cn.hutool.socket"
    if qualified_name.startswith(f"{base}::"):
        tail = qualified_name.removeprefix(f"{base}::")
    elif qualified_name.startswith(f"{base}."):
        package, symbol = qualified_name.removeprefix(f"{base}.").split("::", 1)
        tail = f"{package}::{symbol}"
    else:
        return None
    family = tail.split("::", 1)[0]

    if family in {"SocketConfig", "SocketRuntimeException"}:
        return (
            f"hitool_socket::{family}",
            "crates/hitool-socket/src/compat.rs::config_errors_operations_and_formatting_are_explicit",
        )
    if family == "ChannelUtil":
        test = (
            "config_errors_operations_and_formatting_are_explicit"
            if "createFixedGroup" in tail or "::" not in tail
            else "session_limits_timeouts_and_close_aliases_are_bounded"
        )
        return (f"hitool_socket::{family}", f"crates/hitool-socket/src/compat.rs::{test}")
    if family == "SocketUtil":
        return (
            f"hitool_socket::{family}",
            "crates/hitool-socket/src/compat.rs::aio_server_client_session_and_protocol_use_real_loopback_io",
        )
    if family == "aio":
        nested = tail.split("::", 2)[1]
        if nested in {"AcceptHandler", "ReadHandler"}:
            test = "completion_handlers_and_nio_facades_delegate_to_tokio"
        elif nested in {"AioClient", "AioServer"}:
            test = "aio_server_client_session_and_protocol_use_real_loopback_io"
        elif nested == "AioSession":
            test = "session_limits_timeouts_and_close_aliases_are_bounded"
        else:
            test = "aio_client_read_dispatches_server_bytes"
        return (
            f"hitool_socket::aio::{nested}",
            f"crates/hitool-socket/src/compat.rs::{test}",
        )
    if family == "nio":
        nested = tail.split("::", 2)[1]
        test = (
            "config_errors_operations_and_formatting_are_explicit"
            if nested == "Operation"
            else "completion_handlers_and_nio_facades_delegate_to_tokio"
        )
        return (
            f"hitool_socket::nio::{nested}",
            f"crates/hitool-socket/src/compat.rs::{test}",
        )
    if family == "protocol":
        nested = tail.split("::", 2)[1]
        return (
            f"hitool_socket::protocol::{nested}",
            "crates/hitool-socket/src/compat.rs::aio_server_client_session_and_protocol_use_real_loopback_io",
        )
    return None


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-socket":
            continue
        target = mapping(row["qualified_name"])
        if target is None:
            raise SystemExit(f"unmapped Hutool Socket family: {row['qualified_name']}")
        selected += 1
        symbol, test = target
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": "idiomatic",
            "hitool_symbol": symbol,
            "test_evidence": test,
            "notes": (
                "Tokio-backed bounded TCP sessions preserve Hutool Socket configuration, "
                "AIO/NIO lifecycle, callbacks, protocol traits, timeouts, gathered writes, "
                "structured failures, and managed shutdown with Rust async Results."
            ),
        }

    if selected != 102:
        raise SystemExit(f"expected 102 reviewed Socket APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} reviewed Hutool Socket APIs")


if __name__ == "__main__":
    main()
