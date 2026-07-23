#!/usr/bin/env python3
"""Record reviewed Hutool HttpRequest facade APIs with executable Rust evidence."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
PREFIX = "cn.hutool.http::HttpRequest"

IDIOMATIC_MEMBERS = {
    "HttpRequest": (
        "hitool_http::HttpRequest",
        "crates/hitool-http/src/request.rs::factories_set_method_and_url",
        "Hutool-named request builder over reqwest with DenyLocalTargets, timeouts and max body.",
    ),
    "get": (
        "hitool_http::HttpRequest::get",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "Static GET factory.",
    ),
    "post": (
        "hitool_http::HttpRequest::post",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_body_json",
        "Static POST factory.",
    ),
    "head": (
        "hitool_http::HttpRequest::head",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static HEAD factory.",
    ),
    "options": (
        "hitool_http::HttpRequest::options",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static OPTIONS factory.",
    ),
    "put": (
        "hitool_http::HttpRequest::put",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static PUT factory.",
    ),
    "patch": (
        "hitool_http::HttpRequest::patch",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static PATCH factory.",
    ),
    "delete": (
        "hitool_http::HttpRequest::delete",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static DELETE factory.",
    ),
    "trace": (
        "hitool_http::HttpRequest::trace",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Static TRACE factory.",
    ),
    "of": (
        "hitool_http::HttpRequest::of",
        "crates/hitool-http/src/request.rs::factories_set_method_and_url",
        "of(String) supported; UrlBuilder/Charset overloads planned.",
    ),
    "getUrl": (
        "hitool_http::HttpRequest::get_url",
        "crates/hitool-http/src/request.rs::factories_set_method_and_url",
        "Owned URL accessor.",
    ),
    "setUrl": (
        "hitool_http::HttpRequest::set_url",
        "crates/hitool-http/src/request.rs::factories_set_method_and_url",
        "String URL setter; UrlBuilder overload planned.",
    ),
    "getMethod": (
        "hitool_http::HttpRequest::get_method",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Method accessor.",
    ),
    "setMethod": (
        "hitool_http::HttpRequest::set_method",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Method mutator.",
    ),
    "method": (
        "hitool_http::HttpRequest::method",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_method_factories",
        "Fluent method setter.",
    ),
    "contentType": (
        "hitool_http::HttpRequest::content_type",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_body_json",
        "Sets Content-Type header.",
    ),
    "form": (
        "hitool_http::HttpRequest::form",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_form",
        "Map/pair form helpers; multipart file overloads planned.",
    ),
    "body": (
        "hitool_http::HttpRequest::body",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_post_body_json",
        "String/bytes body with JSON/XML content-type detection.",
    ),
    "timeout": (
        "hitool_http::HttpRequest::timeout",
        "crates/hitool-http/src/request.rs::timeout_negative_keeps_default",
        "Millisecond timeout; negative keeps HttpConfig defaults.",
    ),
    "setConnectionTimeout": (
        "hitool_http::HttpRequest::set_connection_timeout",
        "crates/hitool-http/src/request.rs::timeout_negative_keeps_default",
        "Connect timeout in milliseconds.",
    ),
    "setReadTimeout": (
        "hitool_http::HttpRequest::set_read_timeout",
        "crates/hitool-http/src/request.rs::timeout_negative_keeps_default",
        "Read/total timeout in milliseconds.",
    ),
    "setFollowRedirects": (
        "hitool_http::HttpRequest::set_follow_redirects",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "Maps to redirect_limit 10 or 0 on the underlying client.",
    ),
    "execute": (
        "hitool_http::HttpRequest::execute",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "Async execute returning bounded HttpResponse; boolean isAsync overload folded into async-first API.",
    ),
    "basicAuth": (
        "hitool_http::HttpRequest::basic_auth",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_basic_auth_header",
        "Sets Authorization Basic header.",
    ),
    "bearerAuth": (
        "hitool_http::HttpRequest::bearer_auth",
        "crates/hitool-http/src/request.rs::basic_auth_header_is_set",
        "Sets Authorization Bearer header.",
    ),
    "auth": (
        "hitool_http::HttpRequest::auth",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_request_basic_auth_header",
        "Sets raw Authorization header content.",
    ),
    "cookie": (
        "hitool_http::HttpRequest::cookie",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "String Cookie header; HttpCookie collection overloads planned.",
    ),
    "enableDefaultCookie": (
        "hitool_http::HttpRequest::enable_default_cookie",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "Enables request-scoped reqwest cookie jar instead of a process global.",
    ),
    "setConfig": (
        "hitool_http::HttpRequest::set_config",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Overlays HttpConfig when building the per-request client.",
    ),
    "disableCache": (
        "hitool_http::HttpRequest::disable_cache",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Disables caches via HttpConfig.disable_cache on the request client.",
    ),
    "setMaxRedirectCount": (
        "hitool_http::HttpRequest::set_max_redirect_count",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Per-request redirect limit.",
    ),
    "setFollowRedirectsCookie": (
        "hitool_http::HttpRequest::set_follow_redirects_cookie",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Enables cookie jar when cookies should follow redirects.",
    ),
    "setRest": (
        "hitool_http::HttpRequest::set_rest",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "REST flag retained for Hutool callers.",
    ),
    "setHttpProxy": (
        "hitool_http::HttpRequest::set_http_proxy",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Host/port proxy URL applied to the request client.",
    ),
    "setProxy": (
        "hitool_http::HttpRequest::set_proxy",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Proxy URL string stand-in for Java Proxy.",
    ),
    "setSSLProtocol": (
        "hitool_http::HttpRequest::set_ssl_protocol",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "TLS 1.2/1.3 selection via HttpConfig.",
    ),
    "setHostnameVerifier": (
        "hitool_http::HttpRequest::set_hostname_verifier",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "HostnameVerification enum stand-in for Java HostnameVerifier.",
    ),
    "basicProxyAuth": (
        "hitool_http::HttpRequest::basic_proxy_auth",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Sets Proxy-Authorization Basic header.",
    ),
    "proxyAuth": (
        "hitool_http::HttpRequest::proxy_auth",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Sets raw Proxy-Authorization header.",
    ),
    "executeAsync": (
        "hitool_http::HttpRequest::execute_async",
        "crates/hitool-http/tests/http_util_request_facade_parity.rs::http_util_create_get_execute_body",
        "Alias of async execute().",
    ),
    "then": (
        "hitool_http::HttpRequest::then",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Async consumer callback after execute.",
    ),
    "thenFunction": (
        "hitool_http::HttpRequest::then_function",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Async map callback after execute.",
    ),
    "toString": (
        "hitool_http::HttpRequest",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Display summary of method/url/form/rest.",
    ),
    "formStr": (
        "hitool_http::HttpRequest::form_str",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Alias of form(Map) matching Hutool formStr.",
    ),
    "disableCookie": (
        "hitool_http::HttpRequest::disable_cookie",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Disables cookie jar and clears Cookie header.",
    ),
    "contentLength": (
        "hitool_http::HttpRequest::content_length",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Sets fixed Content-Length; content_length_str getter available.",
    ),
    "keepAlive": (
        "hitool_http::HttpRequest::keep_alive",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Stores keep-alive preference for Hutool parity (reqwest pool still manages connections).",
    ),
    "isKeepAlive": (
        "hitool_http::HttpRequest::is_keep_alive",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Returns keep-alive preference (default true).",
    ),
    "setFixedContentLength": (
        "hitool_http::HttpRequest::set_fixed_content_length",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Alias of content_length(long).",
    ),
    "addRequestInterceptor": (
        "hitool_http::HttpRequest::add_request_interceptor",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Attaches request interceptor onto request-local HttpConfig.",
    ),
    "addResponseInterceptor": (
        "hitool_http::HttpRequest::add_response_interceptor",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Attaches response interceptor onto request-local HttpConfig.",
    ),
    "addInterceptor": (
        "hitool_http::HttpRequest::add_request_interceptor",
        "crates/hitool-http/src/request.rs::request_config_and_callback_helpers",
        "Maps to add_request_interceptor for request-typed chains.",
    ),
    "fileForm": (
        "hitool_http::HttpRequest::form_file",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Multipart file parts via form_file/form_files; Resource map overload deferred.",
    ),
    "setChunkedStreamingMode": (
        "hitool_http::HttpRequest::set_chunked_streaming_mode",
        "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
        "Records preferred chunk size for Hutool callers (reqwest streams by default).",
    ),
    "setGlobalTimeout": (
        "hitool_http::HttpGlobalConfig::set_timeout",
        "crates/hitool-http/src/global_config.rs::global_config_roundtrip_and_reset",
        "HttpRequest.setGlobalTimeout maps to HttpGlobalConfig::set_timeout (opt-in; not auto-applied).",
    ),
    "getCookieManager": (
        "hitool_http::HttpGlobalConfig::get_cookie_manager",
        "crates/hitool-http/src/cookie/mod.rs::global_cookie_manager_store_and_close",
        "Delegates to HttpGlobalConfig / GlobalCookieManager opt-in handle.",
    ),
    "setCookieManager": (
        "hitool_http::HttpGlobalConfig::set_cookie_manager",
        "crates/hitool-http/src/cookie/mod.rs::global_cookie_manager_store_and_close",
        "Delegates to HttpGlobalConfig / GlobalCookieManager opt-in handle.",
    ),
    "closeCookie": (
        "hitool_http::HttpGlobalConfig::close_cookie",
        "crates/hitool-http/src/cookie/mod.rs::global_cookie_manager_store_and_close",
        "Delegates to GlobalCookieManager::close_cookie.",
    ),
}

UNSAFE_MEMBERS: dict[str, str] = {}

PLANNED_MEMBERS = {
    "setUrlHandler": "java.net.URLStreamHandler has no Rust equivalent.",
    "getConnection": "HttpURLConnection handle not exposed; use execute()/HttpResponse.",
    "header": "Inherited HttpBase header surface already covered; request-level header() exists but inventory may list separately.",
    "setSSLSocketFactory": "Java SSLSocketFactory has no direct reqwest peer; use HttpConfig TLS identity/roots.",
}


def member_of(api_id: str) -> str:
    rest = api_id.split("::", 2)[-1]
    if rest in {"HttpRequest#", "HttpRequest"} or rest.endswith("#") and rest.startswith("HttpRequest") and "::" not in rest:
        return "HttpRequest"
    # HttpRequest#(String url) constructor
    if rest.startswith("HttpRequest#"):
        return "HttpRequest"
    name = rest.split("#", 1)[0]
    if name.startswith("HttpRequest::"):
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
        # skip nested packages accidentally matching? PREFIX is exact class
        if not (
            row["api_id"] == "cn.hutool.http::HttpRequest#"
            or row["api_id"].startswith("cn.hutool.http::HttpRequest::")
            or row["api_id"].startswith("cn.hutool.http::HttpRequest#")
        ):
            continue
        member = member_of(row["api_id"])
        if member in UNSAFE_MEMBERS:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "unsafe-to-copy",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": UNSAFE_MEMBERS[member],
            }
            selected += 1
            continue
        if member in IDIOMATIC_MEMBERS:
            # UrlBuilder / URLStreamHandler stay planned; File/Resource/HttpCookie have Rust stand-ins.
            if any(token in row["api_id"] for token in ("UrlBuilder", "URLStreamHandler")):
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "planned",
                    "hitool_symbol": "",
                    "test_evidence": "",
                    "notes": f"{member} overload with Java-only types deferred; core String/Map/byte[] shape is idiomatic.",
                }
                selected += 1
                continue
            if "HttpCookie" in row["api_id"] and member == "cookie":
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "idiomatic",
                    "hitool_symbol": "hitool_http::HttpRequest::cookies",
                    "test_evidence": "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
                    "notes": "HttpCookie collection overload via cookies().",
                }
                selected += 1
                counted += 1
                continue
            if "File" in row["api_id"] and member in {"form", "fileForm"}:
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "idiomatic",
                    "hitool_symbol": "hitool_http::HttpRequest::form_file",
                    "test_evidence": "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
                    "notes": "File form overload via form_file/form_files path parts.",
                }
                selected += 1
                counted += 1
                continue
            if "Resource" in row["api_id"] and member in {"body", "form"}:
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "idiomatic",
                    "hitool_symbol": "hitool_http::HttpRequest::body_resource",
                    "test_evidence": "crates/hitool-http/src/request.rs::cookie_keepalive_content_length_and_form_helpers",
                    "notes": "Resource body overload via body_resource(ResourceBody).",
                }
                selected += 1
                counted += 1
                continue
            if member == "of" and "Charset" in row["api_id"]:
                indexed[row["api_id"]] = {
                    "api_id": row["api_id"],
                    "status": "planned",
                    "hitool_symbol": "",
                    "test_evidence": "",
                    "notes": "Charset-tagged of() deferred; URL strings are UTF-8.",
                }
                selected += 1
                continue
            symbol, evidence, notes = IDIOMATIC_MEMBERS[member]
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
            selected += 1
            counted += 1
            continue
        if member in PLANNED_MEMBERS:
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": PLANNED_MEMBERS[member],
            }
            selected += 1
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
    print(f"recorded {selected} HttpRequest APIs ({counted} counted toward parity)")


if __name__ == "__main__":
    main()
