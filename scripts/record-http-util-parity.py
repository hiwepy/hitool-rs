#!/usr/bin/env python3
"""Record reviewed Hutool HttpUtil APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpUtil"

# Member name (before signature) -> (status, symbol, evidence, notes)
DECISIONS_BY_MEMBER = {
    "HttpUtil": (
        "idiomatic",
        "hitool_http::HttpUtil",
        "crates/hitool-http/tests/http_util_offline_parity.rs::is_http_test",
        "Stateless Hutool HttpUtil facade over offline param helpers and secure HttpRequest networking.",
    ),
    "isHttps": (
        "idiomatic",
        "hitool_http::HttpUtil::is_https",
        "crates/hitool-http/tests/http_util_offline_parity.rs::is_https_test",
        "Case-insensitive https: scheme prefix check.",
    ),
    "isHttp": (
        "idiomatic",
        "hitool_http::HttpUtil::is_http",
        "crates/hitool-http/tests/http_util_offline_parity.rs::is_http_test",
        "Case-insensitive http: scheme prefix check.",
    ),
    "createRequest": (
        "idiomatic",
        "hitool_http::HttpUtil::create_request",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Returns HttpRequest builder with DenyLocalTargets by default.",
    ),
    "createGet": (
        "idiomatic",
        "hitool_http::HttpUtil::create_get",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "GET request factory; redirect overload maps to set_follow_redirects.",
    ),
    "createPost": (
        "idiomatic",
        "hitool_http::HttpUtil::create_post",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_body_json",
        "POST request factory delegated to HttpRequest::post.",
    ),
    "get": (
        "idiomatic",
        "hitool_http::HttpUtil::get",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_get_with_form_appends_query",
        "Async GET facades (timeout/form overloads) over HttpRequest with max-body and URL policy.",
    ),
    "post": (
        "idiomatic",
        "hitool_http::HttpUtil::post_form",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_form",
        "Async POST form/body facades; charset overload folded into UTF-8 body decode.",
    ),
    "downloadString": (
        "idiomatic",
        "hitool_http::HttpUtil::download_string",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_download_bytes",
        "Downloads bytes then decodes via encoding_rs; StreamProgress overload is planned.",
    ),
    "downloadBytes": (
        "idiomatic",
        "hitool_http::HttpUtil::download_bytes",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_download_bytes",
        "Bounded byte download via HttpRequest::execute_bytes.",
    ),
    "downloadFile": (
        "idiomatic",
        "hitool_http::HttpUtil::download_file",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_download_bytes",
        "Buffers then writes with std::fs; timeout overload via download_file_timeout; StreamProgress variants remain planned.",
    ),
    "downloadFileFromUrl": (
        "idiomatic",
        "hitool_http::HttpUtil::download_file_from_url",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_download_writer_and_file_from_url",
        "Buffers via HttpRequest then HttpResponse::write_body_for_file; StreamProgress overloads remain planned.",
    ),
    "download": (
        "idiomatic",
        "hitool_http::HttpUtil::download",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_download_writer_and_file_from_url",
        "Writes downloaded bytes to a Write sink; StreamProgress overload remains planned.",
    ),
    "toParams": (
        "idiomatic",
        "hitool_http::HttpUtil::to_params",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_to_params_test",
        "Query encoding with optional form-urlencoded mode; Charset args use UTF-8.",
    ),
    "encodeParams": (
        "idiomatic",
        "hitool_http::HttpUtil::encode_params",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_encode_param_test",
        "Normalizes URL query fragments; Charset folded to UTF-8 percent-encoding.",
    ),
    "normalizeParams": (
        "idiomatic",
        "hitool_http::HttpUtil::normalize_params",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_normalize_params_test",
        "Parameter pair normalization with optional encoding.",
    ),
    "decodeParamMap": (
        "idiomatic",
        "hitool_http::HttpUtil::decode_param_map",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_decode_param_map_test",
        "Single-value query map decode from URL or raw query.",
    ),
    "decodeParams": (
        "idiomatic",
        "hitool_http::HttpUtil::decode_params",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_decode_params_test",
        "Multi-value query decode including form-urlencoded mode overloads.",
    ),
    "urlWithForm": (
        "idiomatic",
        "hitool_http::HttpUtil::url_with_form",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_test_url_with_form_test",
        "Appends map or raw query to URL with optional encoding.",
    ),
    "urlWithFormUrlEncoded": (
        "idiomatic",
        "hitool_http::HttpUtil::url_with_form_url_encoded",
        "crates/hitool-http/tests/http_util_offline_parity.rs::issue3536_test_url_with_form_url_encoded_test",
        "Issue #3536 form-urlencoded append semantics.",
    ),
    "getCharset": (
        "idiomatic",
        "hitool_http::HttpUtil::get_charset",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_get_charset_string_test",
        "Content-Type charset extraction; HttpURLConnection overload is Java-specific planned.",
    ),
    "getString": (
        "idiomatic",
        "hitool_http::HttpUtil::get_string",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_get_string_bytes_test",
        "encoding_rs decode with optional HTML meta charset re-detect; InputStream overload uses bytes.",
    ),
    "getMimeType": (
        "idiomatic",
        "hitool_http::HttpUtil::get_mime_type",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_get_mime_type_default_test",
        "Extension-based MIME guess with default-value overload.",
    ),
    "getContentTypeByRequestBody": (
        "idiomatic",
        "hitool_http::HttpUtil::get_content_type_by_request_body",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_get_content_type_by_request_body_test",
        "Delegates to ContentType::detect for JSON/XML bodies.",
    ),
    "buildBasicAuth": (
        "idiomatic",
        "hitool_http::HttpUtil::build_basic_auth",
        "crates/hitool-http/tests/http_util_offline_parity.rs::http_util_build_basic_auth_test",
        "UTF-8 Basic Authorization header via hitool_core base64.",
    ),
    "createServer": (
        "planned",
        "",
        "",
        "SimpleServer is out of scope for the client-focused hitool-http crate.",
    ),
    "closeCookie": (
        "idiomatic",
        "hitool_http::GlobalCookieManager::close_cookie",
        "crates/hitool-http/src/cookie/mod.rs::global_cookie_manager_store_and_close",
        "Delegates to opt-in GlobalCookieManager::close_cookie.",
    ),
}


def member_of(api_id: str) -> str:
    # cn.hutool.http::HttpUtil::get#String (String urlString) -> get
    # cn.hutool.http::HttpUtil# -> HttpUtil
    rest = api_id.split("::", 2)[-1]
    if rest.endswith("#") and "#" not in rest[:-1]:
        return "HttpUtil"
    name = rest.split("#", 1)[0]
    if name.startswith("HttpUtil::"):
        name = name.split("::", 1)[1]
    return name


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}
    selected = 0
    counted = 0
    for row in inventory:
        if row["module"] != "hutool-http" or not row["api_id"].startswith(PREFIX):
            continue
        member = member_of(row["api_id"])
        # Special-case: HttpURLConnection getCharset overload
        if member == "getCharset" and "HttpURLConnection" in row["api_id"]:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": "Java HttpURLConnection-specific; use get_charset(Content-Type).",
            }
            selected += 1
            continue
        # StreamProgress download overloads — implemented via *_with_progress helpers
        if member == "downloadString" and "StreamProgress" in row["api_id"]:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": "hitool_http::HttpUtil::download_string_with_progress",
                "test_evidence": "crates/hitool-http/src/http_util.rs::download_string_with_progress",
                "notes": "StreamProgress downloadString via download_string_with_progress.",
            }
            selected += 1
            counted += 1
            continue
        if member == "downloadFile" and "StreamProgress" in row["api_id"]:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": "hitool_http::HttpUtil::download_file_with_progress",
                "test_evidence": "crates/hitool-http/src/http_util.rs::download_file_with_progress",
                "notes": "StreamProgress downloadFile via download_file_with_progress / timeout variant.",
            }
            selected += 1
            counted += 1
            continue
        if member == "downloadFileFromUrl" and "StreamProgress" in row["api_id"]:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": "hitool_http::HttpUtil::download_file_from_url_with_progress",
                "test_evidence": "crates/hitool-http/src/http_util.rs::download_file_from_url_with_progress",
                "notes": "StreamProgress downloadFileFromUrl via download_file_from_url_with_progress.",
            }
            selected += 1
            counted += 1
            continue
        if member == "download" and "StreamProgress" in row["api_id"]:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": "hitool_http::HttpUtil::download_with_progress",
                "test_evidence": "crates/hitool-http/src/http_util.rs::download_with_progress",
                "notes": "StreamProgress download(OutputStream) via download_with_progress.",
            }
            selected += 1
            counted += 1
            continue
        if member == "closeCookie":
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": "hitool_http::GlobalCookieManager::close_cookie",
                "test_evidence": "crates/hitool-http/src/cookie/mod.rs::global_cookie_manager_store_and_close",
                "notes": "Delegates to opt-in GlobalCookieManager::close_cookie (not silent process defaults).",
            }
            selected += 1
            counted += 1
            continue
        decision = DECISIONS_BY_MEMBER.get(member)
        if decision is None:
            continue
        status, symbol, evidence, notes = decision
        indexed[row["api_id"]] = {
            "api_id": row["api_id"],
            "status": status,
            "hitool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }
        selected += 1
        if status in {"implemented", "native", "idiomatic"}:
            counted += 1
    before = len(indexed)
    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS, quoting=csv.QUOTE_MINIMAL)
        writer.writeheader()
        writer.writerows(indexed.values())
    # round-trip sanity
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        after = len(list(csv.DictReader(stream)))
    if after != before:
        raise SystemExit(f"decisions round-trip lost rows: before={before} after={after}")
    print(f"recorded {selected} HttpUtil APIs ({counted} counted toward parity)")


if __name__ == "__main__":
    main()
