#!/usr/bin/env python3
"""Merge-only recorder for HttpGlobalConfig / cookie / body / downloader / interceptor / resource / streams."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]

# prefix -> (symbol, evidence, notes)
SURFACES: list[tuple[str, str, str, str]] = [
    (
        "cn.hutool.http::HttpGlobalConfig",
        "hitool_http::HttpGlobalConfig",
        "crates/hitool-http/src/global_config.rs::global_config_roundtrip_and_reset",
        "Process-scoped Hutool HttpGlobalConfig store; callers opt in — HttpRequest does not auto-apply.",
    ),
    (
        "cn.hutool.http.cookie::GlobalCookieManager",
        "hitool_http::GlobalCookieManager",
        "crates/hitool-http/src/cookie/mod.rs::thread_local_cookie_store_roundtrip",
        "Opt-in shared CookieJar handle; add/store/getCookies take URL strings instead of HttpConnection.",
    ),
    (
        "cn.hutool.http.cookie::ThreadLocalCookieStore",
        "hitool_http::ThreadLocalCookieStore",
        "crates/hitool-http/src/cookie/mod.rs::thread_local_cookie_store_roundtrip",
        "Thread-local CookieJar matching Hutool ThreadLocalCookieStore.",
    ),
    (
        "cn.hutool.http.body::BytesBody",
        "hitool_http::BytesBody",
        "crates/hitool-http/src/body/bytes_form.rs::bytes_and_form_bodies_write",
        "Raw byte RequestBody writer.",
    ),
    (
        "cn.hutool.http.body::FormUrlEncodedBody",
        "hitool_http::FormUrlEncodedBody",
        "crates/hitool-http/src/body/bytes_form.rs::bytes_and_form_bodies_write",
        "application/x-www-form-urlencoded body; toString returns encoded payload.",
    ),
    (
        "cn.hutool.http.body::ResourceBody",
        "hitool_http::ResourceBody",
        "crates/hitool-http/src/body/bytes_form.rs::resource_body_from_bytes",
        "Resource-backed body from bytes or filesystem path.",
    ),
    (
        "cn.hutool.http.body::RequestBody",
        "hitool_http::RequestBody",
        "crates/hitool-http/src/body/bytes_form.rs::resource_body_from_bytes",
        "RequestBody.write trait shared by BytesBody/FormUrlEncodedBody/ResourceBody.",
    ),
    (
        "cn.hutool.http::HttpDownloader",
        "hitool_http::HttpDownloader",
        "crates/hitool-http/src/downloader.rs::HttpDownloader",
        "Thin HttpUtil download facade with optional StreamProgress.",
    ),
    (
        "cn.hutool.http::GlobalInterceptor",
        "hitool_http::GlobalInterceptor",
        "crates/hitool-http/src/interceptor.rs::global_interceptor_add_clear_and_apply",
        "Process-scoped request/response interceptor registry with clear helpers.",
    ),
    (
        "cn.hutool.http::HttpInterceptor",
        "hitool_http::{RequestInterceptor,ResponseInterceptor,HttpInterceptorError}",
        "crates/hitool-http/src/interceptor.rs::global_interceptor_add_clear_and_apply",
        "Interceptor callbacks + Chain semantics via Vec/clear/apply on GlobalInterceptor.",
    ),
    (
        "cn.hutool.http::HttpResource",
        "hitool_http::HttpResource",
        "crates/hitool-http/src/resource.rs::http_resource_name_type_and_stream",
        "Named resource wrapper over ResourceBody with content-type and stream cursor.",
    ),
    (
        "cn.hutool.http::MultipartOutputStream",
        "hitool_http::MultipartOutputStream",
        "crates/hitool-http/src/body/multipart_stream.rs::multipart_output_stream_writes_fields",
        "Incremental multipart/form-data writer with finish/close.",
    ),
    (
        "cn.hutool.http::HttpInputStream",
        "hitool_http::HttpInputStream",
        "crates/hitool-http/src/input_stream.rs::http_input_stream_read_skip_reset",
        "Seekable Cursor over HttpResponse body bytes (Read/skip/reset/available).",
    ),
]


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    for row in inventory:
        if row["module"] != "hutool-http":
            continue
        api_id = row["api_id"]
        for prefix, symbol, evidence, notes in SURFACES:
            if api_id == prefix + "#" or api_id.startswith(prefix + "::") or api_id.startswith(prefix + "#"):
                indexed[api_id] = {
                    "api_id": api_id,
                    "status": "idiomatic",
                    "hitool_symbol": symbol,
                    "test_evidence": evidence,
                    "notes": notes,
                }
                selected += 1
                break

    before = len(indexed)
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS, quoting=csv.QUOTE_MINIMAL)
        writer.writeheader()
        writer.writerows(indexed.values())
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        after = len(list(csv.DictReader(stream)))
    if after != before:
        raise SystemExit(f"decisions round-trip lost rows: before={before} after={after}")
    print(f"recorded {selected} HTTP client-surface APIs (merge-only)")


if __name__ == "__main__":
    main()
