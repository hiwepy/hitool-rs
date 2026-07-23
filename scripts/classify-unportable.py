#!/usr/bin/env python3
"""Classify Hutool APIs as portable vs unportable for parity ledger defaults.

Tags (notes prefix): jvm_only | reflection | awt_swing | javax_servlet |
javax_sql_spi | jndi | bouncycastle_only | soap_server | portable

Usage:
  python3 scripts/classify-unportable.py
  python3 scripts/classify-unportable.py --summary
  python3 scripts/classify-unportable.py --write-csv parity/unportable-tags.csv
"""

from __future__ import annotations

import argparse
import csv
import re
from collections import Counter
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
FIELDS = ["api_id", "module", "package", "tag", "reason"]

# Path / name heuristics → unportable (or deferred) tags.
RULES: list[tuple[str, re.Pattern[str], str]] = [
    ("awt_swing", re.compile(r"/swing/|cn\.hutool\.core\.swing"), "AWT/Swing desktop UI"),
    ("javax_servlet", re.compile(r"/servlet/|javax\.servlet|jakarta\.servlet"), "Servlet API"),
    ("jndi", re.compile(r"Jndi|jndi|/jndi/"), "JNDI naming"),
    ("reflection", re.compile(
        r"ReflectUtil|ClassUtil|ConstructorUtil|MethodUtil|FieldUtil|"
        r"AnnotationUtil|ProxyUtil|MapProxy|BeanPath|BeanDesc|"
        r"TypeReference|TypeUtil|ModifierUtil|ClassLoaderUtil"
    ), "Java reflection / dynamic proxies"),
    ("javax_sql_spi", re.compile(
        r"StatementWrapper|ConnectionWraper|ConnectionWrapper|"
        r"AbstractDataSource|PooledConnection|ThreadLocalConnection|"
        r"GlobalDbConfig|GlobalDSFactory|DaoTemplate|StatementUtil|"
        r"prepareStatement|prepareCall|javax\.sql"
    ), "JDBC SPI / globals"),
    ("bouncycastle_only", re.compile(
        r"\bZUC\b|CipherWrapper|ProviderFactory|BCUtil#toDomainParams|"
        r"BCUtil#toParams|X9ECParameters|ECParameterSpec"
    ), "BouncyCastle-only / JCE SPI"),
    ("soap_server", re.compile(
        r"webservice|SoapClient|JakartaSoap|SimpleServer|HttpServer|"
        r"CustomProtocolsSSLFactory|HttpConnection"
    ), "SOAP / embedded HTTP server / URLConnection peer"),
    ("jvm_only", re.compile(
        r"/ssh/|/ftp/|/spring/|/cglib/|/expression/|"
        r"JndiDSFactory|RuntimeUtil|SystemPropsUtil|JdkUtil|"
        r"ServiceLoaderUtil|JaxbUtil|CompilerUtil"
    ), "JVM / Java EE ecosystem glue"),
]


def package_of(row: dict[str, str]) -> str:
    fp = row.get("file_path", "")
    marker = "/cn/hutool/"
    if marker in fp:
        rest = fp.split(marker, 1)[1]
        parts = rest.split("/")
        if len(parts) >= 2:
            return f"{parts[0]}.{parts[1]}"
        return parts[0]
    qn = row.get("qualified_name", "")
    if "cn.hutool." in qn:
        mid = qn.split("cn.hutool.", 1)[1]
        return mid.split("::")[0].rsplit(".", 1)[0] if "::" in mid else mid
    return row.get("module", "?")


def classify_row(row: dict[str, str]) -> tuple[str, str]:
    """Return (tag, reason) for one inventory row."""
    blob = " ".join(
        [
            row.get("api_id", ""),
            row.get("qualified_name", ""),
            row.get("signature", ""),
            row.get("file_path", ""),
        ]
    )
    # Module-level defaults for known Java-glue crates.
    module = row.get("module", "")
    if module == "hutool-poi":
        return "jvm_only", "poi deferred until easy* engines"
    if module == "hutool-extra":
        fp = row.get("file_path", "")
        for seg, tag, reason in (
            ("/template/", "jvm_only", "template engines"),
            ("/tokenizer/", "jvm_only", "CJK tokenizer"),
            ("/ssh/", "jvm_only", "SSH client"),
            ("/ftp/", "jvm_only", "FTP client"),
            ("/servlet/", "javax_servlet", "Servlet extras"),
            ("/spring/", "jvm_only", "Spring integration"),
            ("/cglib/", "jvm_only", "CGLIB proxies"),
            ("/expression/", "jvm_only", "expression engines"),
        ):
            if seg in fp:
                return tag, reason
    for tag, pattern, reason in RULES:
        if pattern.search(blob):
            return tag, reason
    return "portable", "candidate for idiomatic Rust"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--inventory", type=Path, default=INVENTORY)
    parser.add_argument("--summary", action="store_true")
    parser.add_argument("--write-csv", type=Path, default=None)
    parser.add_argument(
        "--missing-only",
        action="store_true",
        help="Only classify APIs absent from parity/decisions.csv",
    )
    parser.add_argument(
        "--decisions",
        type=Path,
        default=Path("parity/decisions.csv"),
    )
    args = parser.parse_args()

    with args.inventory.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))

    decided: set[str] = set()
    if args.missing_only and args.decisions.exists():
        with args.decisions.open(encoding="utf-8", newline="") as stream:
            decided = {r["api_id"] for r in csv.DictReader(stream)}

    rows_out: list[dict[str, str]] = []
    counts: Counter[str] = Counter()
    for row in inventory:
        if args.missing_only and row["api_id"] in decided:
            continue
        tag, reason = classify_row(row)
        counts[tag] += 1
        rows_out.append(
            {
                "api_id": row["api_id"],
                "module": row["module"],
                "package": package_of(row),
                "tag": tag,
                "reason": reason,
            }
        )

    if args.summary or not args.write_csv:
        print("unportable classification summary:")
        for tag, n in counts.most_common():
            print(f"  {tag:20s} {n:5d}")
        print(f"  {'TOTAL':20s} {sum(counts.values()):5d}")

    if args.write_csv:
        args.write_csv.parent.mkdir(parents=True, exist_ok=True)
        with args.write_csv.open("w", encoding="utf-8", newline="") as stream:
            writer = csv.DictWriter(stream, fieldnames=FIELDS)
            writer.writeheader()
            writer.writerows(rows_out)
        print(f"wrote {len(rows_out)} rows → {args.write_csv}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
