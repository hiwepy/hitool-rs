#!/usr/bin/env python3
"""Correct dishonest idiomatic rows for hard-unportable surfaces back to planned.

Keeps idiomatic where HiTool already has an evidenced Rust mapping (aop wrappers,
JSON TypeReference/Serde, CompilerUtil Rustc, etc.). Only demotes clear JVM glue:
swing, servlet, ssh/ftp/spring/cglib/template/tokenizer, SOAP/server/HttpConnection,
JDBC SPI/globals, JNDI, pure-BC crypto.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]


def should_demote(inv: dict[str, str]) -> tuple[bool, str]:
    fp = inv.get("file_path", "")
    qn = inv.get("qualified_name", "") + " " + inv.get("api_id", "")
    blob = fp + " " + qn
    checks = [
        ("/swing/", "awt_swing", "AWT/Swing desktop — feature stubs stay planned at ledger DoD"),
        ("/servlet/", "javax_servlet", "Servlet API not portable"),
        ("/ssh/", "jvm_only", "SSH client not in DoD idiomatic set"),
        ("/ftp/", "jvm_only", "FTP client not in DoD idiomatic set"),
        ("/spring/", "jvm_only", "Spring integration not portable"),
        ("/cglib/", "jvm_only", "CGLIB proxies not portable"),
        ("/template/", "jvm_only", "Java template engines not portable"),
        ("/tokenizer/", "jvm_only", "CJK tokenizer engines not portable"),
        ("/expression/", "jvm_only", "Java expression engines not portable"),
        ("hutool-poi/", "jvm_only", "poi deferred until easy* engines"),
        ("webservice", "soap_server", "SOAP clients planned"),
        ("SoapClient", "soap_server", "SOAP clients planned"),
        ("JakartaSoap", "soap_server", "SOAP clients planned"),
        ("SimpleServer", "soap_server", "embedded HTTP server planned"),
        ("HttpServer", "soap_server", "embedded HTTP server planned"),
        ("CustomProtocolsSSLFactory", "soap_server", "SSL factory SPI planned"),
        ("HttpConnection", "soap_server", "HttpURLConnection peer planned"),
        ("StatementWrapper", "javax_sql_spi", "JDBC Statement SPI planned"),
        ("ConnectionWraper", "javax_sql_spi", "JDBC Connection SPI planned"),
        ("ConnectionWrapper", "javax_sql_spi", "JDBC Connection SPI planned"),
        ("DaoTemplate", "javax_sql_spi", "JDBC DaoTemplate planned"),
        ("AbstractDataSource", "javax_sql_spi", "javax.sql.DataSource SPI planned"),
        ("GlobalDbConfig", "javax_sql_spi", "global DB config unsafe/planned"),
        ("GlobalDSFactory", "javax_sql_spi", "global DS factory planned"),
        ("ThreadLocalConnection", "javax_sql_spi", "ThreadLocal JDBC planned"),
        ("JndiDSFactory", "jndi", "JNDI DS factory planned"),
        ("JndiUtil", "jndi", "JNDI planned"),
        ("\bZUC\b", "bouncycastle_only", "ZUC BC-only planned"),
        ("CipherWrapper", "bouncycastle_only", "JCE Cipher SPI planned"),
        ("ProviderFactory", "bouncycastle_only", "BC Provider planned"),
    ]
    import re

    for needle, tag, reason in checks:
        if needle.startswith("\\b") or needle.startswith("("):
            if re.search(needle, blob):
                return True, f"[{tag}] {reason}"
        elif needle in blob:
            return True, f"[{tag}] {reason}"
    return False, ""


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inv = {r["api_id"]: r for r in csv.DictReader(stream)}
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        decisions = list(csv.DictReader(stream))

    demoted = 0
    for row in decisions:
        if row.get("status") not in {"idiomatic", "native", "implemented"}:
            continue
        meta = inv.get(row["api_id"])
        if not meta:
            continue
        ok, note = should_demote(meta)
        if not ok:
            continue
        row["status"] = "planned"
        row["hitool_symbol"] = ""
        row["test_evidence"] = ""
        row["notes"] = note
        demoted += 1

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(decisions)
    print(f"demoted {demoted} unportable idiomatic → planned")


if __name__ == "__main__":
    main()
