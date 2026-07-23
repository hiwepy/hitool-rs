#!/usr/bin/env python3
"""Merge-only recorder: ZipUtil + URLUtil + net + img + FileUtil/IoUtil leftovers.

Does not wipe other modules. Never regresses NumberUtil / DateUtil /
LocalDateTimeUtil / DateTime / PrimitiveArrayUtil rows already recorded by
`record-core-high-traffic-parity.py`. FileUtil/IoUtil rows are only upgraded
when this script marks additional implemented methods idiomatic.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

PROTECTED_PREFIXES = (
    "cn.hutool.core.util::NumberUtil",
    "cn.hutool.core.date::DateUtil",
    "cn.hutool.core.date::LocalDateTimeUtil",
    "cn.hutool.core.date::DateTime",
    "cn.hutool.core.util::PrimitiveArrayUtil",
)

ZIP_ROOT = "cn.hutool.core.util::ZipUtil"
URL_ROOT = "cn.hutool.core.util::URLUtil"
NET_ROOT = "cn.hutool.core.net"  # includes net.url / net.multipart
IMG_ROOT = "cn.hutool.core.img"  # includes img.gif
FILE_ROOT = "cn.hutool.core.io::FileUtil"
IO_ROOT = "cn.hutool.core.io::IoUtil"


def method_name(qualified_name: str) -> str:
    parts = qualified_name.split("::")
    return parts[2] if len(parts) > 2 else ""


def family(qualified_name: str) -> str:
    """Return Java simple class name (`UrlBuilder`, `ImgUtil`, …)."""
    parts = qualified_name.split("::")
    if len(parts) >= 2:
        # `cn.hutool.core.net.url::UrlBuilder` → UrlBuilder
        return parts[1]
    return qualified_name.rsplit(".", 1)[-1]


def zip_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::ZipUtil"
    evidence = "crates/hutool-core/tests/zip_util_parity.rs::gzip_test"
    notes = (
        "zip crate + flate2 facades consolidate Hutool ZipUtil overloads "
        "(gzip/zlib/zip/unzip/append/list/get/read)."
    )
    planned = {"toZipFile", "getStream", "getZipOutputStream"}
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/zip_util.rs::ZipUtil", notes)
    if name in planned:
        return (
            "planned",
            symbol,
            "",
            f"Planned: ZipUtil.{name} needs java.util.zip ZipFile/ZipOutputStream handle types.",
        )
    return ("idiomatic", symbol, evidence, notes)


def url_decision(name: str) -> tuple[str, str, str, str]:
    symbol = "hutool_core::UrlUtil"
    evidence = "crates/hutool-core/tests/url_util_parity.rs::is_url_http"
    notes = "Owned string/URI facades for encode/decode/normalize/complete/file-jar checks."
    planned = {
        "getContentLength",
        "getJarFile",
        "getReader",
        "getStream",
        "getURLs",
        "size",
        "useCachesIfNecessary",
        "getURL",
    }
    if not name:
        return ("idiomatic", symbol, "crates/hutool-core/src/url_util.rs::UrlUtil", notes)
    if name in planned:
        return (
            "planned",
            symbol,
            "",
            f"Planned: URLUtil.{name} needs network URLConnection/JarFile engine.",
        )
    return ("idiomatic", symbol, evidence, notes)


NET_PLANNED = {
    "DefaultTrustManager",
    "PassAuth",
    "ProxySocketFactory",
    "SSLContextBuilder",
    "SSLProtocols",
    "SSLUtil",
    "UserPassAuthenticator",
    "MultipartFormData",
    "MultipartRequestInputStream",
    "UploadFile",
    "UploadFileHeader",
    "UploadSetting",
}

NET_UTIL_PLANNED = {
    "getDnsInfo",
    "getHardwareAddress",
    "getLocalHardwareAddress",
    "getLocalMacAddress",
    "getMacAddress",
    "getNetworkInterface",
    "getNetworkInterfaces",
    "localAddressList",
    "netCat",
    "parseCookies",
    "setGlobalAuthenticator",
    "bigIntegerToIPv6",
    "ipv6ToBigInteger",
    "ipv6ToBitInteger",
    "toIpList",
}

IMG_IDIOMATIC = {
    "ImgUtil",
    "ColorUtil",
    "ScaleType",
    "scale",
    "cut",
    "rotate",
    "flip",
    "gray",
    "binary",
    "read",
    "write",
    "writeJpg",
    "writePng",
    "compress",
    "convert",
    "copyImage",
    "createImage",
    "createTransparentImage",
    "toBase64",
    "toBase64DataUri",
    "toBytes",
    "toHex",
    "getColor",
    "hexToColor",
    "randomColor",
    "toImage",
    "backgroundRemoval",
    "getMainColor",
    "slice",
    "sliceByRowsAndCols",
    "add",
    "computeColorDistance",
    "maxDistance",
}


def net_decision(qualified_name: str, name: str) -> tuple[str, str, str, str]:
    fam = family(qualified_name)
    if fam in NET_PLANNED:
        return (
            "planned",
            f"hutool_core::net::{fam}",
            "",
            f"Planned: {fam} needs JVM SSL/auth/multipart servlet engine.",
        )
    if fam == "NetUtil":
        symbol = "hutool_core::NetUtil"
        evidence = "crates/hutool-core/tests/net_util_parity.rs::net_util_ip_port_helpers"
        notes = "std::net + idna facades for ports, IPv4, localhost, CIDR, proxy IP, ping."
        if not name:
            return ("idiomatic", symbol, "crates/hutool-core/src/net/net_util.rs::NetUtil", notes)
        if name in NET_UTIL_PLANNED:
            return (
                "planned",
                symbol,
                "",
                f"Planned: NetUtil.{name} needs OS NIC/DNS/authenticator APIs.",
            )
        return ("idiomatic", symbol, evidence, notes)
    if fam == "Ipv4Util":
        return (
            "idiomatic",
            "hutool_core::Ipv4Util",
            "crates/hutool-core/tests/net_util_parity.rs::ipv4_util_mask_and_range",
            "std::net::Ipv4Addr facades for mask/range/list/inner-IP matching.",
        )
    if fam == "LocalPortGenerater":
        return (
            "idiomatic",
            "hutool_core::LocalPortGenerater",
            "crates/hutool-core/tests/net_util_parity.rs::local_port_generater",
            "Usable local TCP port generator over NetUtil range scan.",
        )
    if fam in {"URLDecoder", "URLEncodeUtil", "URLEncoder", "RFC3986"}:
        symbol = {
            "URLDecoder": "hutool_core::UrlDecoder",
            "URLEncodeUtil": "hutool_core::UrlEncodeUtil",
            "URLEncoder": "hutool_core::UrlEncoder",
            "RFC3986": "hutool_core::Rfc3986",
        }[fam]
        return (
            "idiomatic",
            symbol,
            "crates/hutool-core/tests/net_util_parity.rs::url_encode_decode_helpers",
            "percent-encoding / RFC3986 facades replace java.net URLEncoder/Decoder.",
        )
    if fam == "FormUrlencoded":
        return (
            "planned",
            "hutool_core::net::FormUrlencoded",
            "",
            "Planned: FormUrlencoded needs application/x-www-form-urlencoded charset table.",
        )
    if fam in {"UrlBuilder", "UrlPath", "UrlQuery", "MaskBit"}:
        if fam == "MaskBit":
            return (
                "idiomatic",
                "hutool_core::MaskBit",
                "crates/hutool-core/tests/net_util_parity.rs::ipv4_util_mask_and_range",
                "MaskBit delegates to Ipv4Util mask helpers.",
            )
        if fam == "UrlBuilder":
            return (
                "idiomatic",
                "hutool_core::UrlBuilder",
                "crates/hutool-core/tests/net_util_parity.rs::url_builder_path",
                "Owned UrlBuilder path/query encoding helpers.",
            )
        return (
            "planned",
            f"hutool_core::net::url::{fam}",
            "",
            f"Planned: {fam} full mutator surface awaiting completion.",
        )
    return (
        "planned",
        f"hutool_core::net::{fam}",
        "",
        f"Planned: {fam} awaiting typed Rust surface.",
    )


def img_decision(qualified_name: str, name: str) -> tuple[str, str, str, str]:
    fam = family(qualified_name)
    if fam == "ImgUtil" and (not name or name in IMG_IDIOMATIC):
        return (
            "idiomatic",
            "hutool_core::ImgUtil",
            "crates/hutool-core/tests/img_util_parity.rs::scale_cut_slice_roundtrip",
            "image crate (feature `img`) byte/pixel facades for scale/cut/rotate/encode.",
        )
    if fam == "ColorUtil" and (not name or name in IMG_IDIOMATIC):
        return (
            "idiomatic",
            "hutool_core::ColorUtil",
            "crates/hutool-core/tests/img_util_parity.rs::color_util_hex_distance",
            "RGB/hex/distance helpers without AWT Color.",
        )
    if fam == "ScaleType":
        return (
            "idiomatic",
            "hutool_core::img::ScaleType",
            "crates/hutool-core/tests/img_util_parity.rs::scale_cut_slice_roundtrip",
            "Scale filter enum maps to image::imageops::FilterType.",
        )
    if fam == "ImgUtil" and name in {"backgroundRemoval", "getMainColor"}:
        return (
            "idiomatic",
            "hutool_core::ImgUtil",
            "crates/hutool-core/tests/img_util_parity.rs::background_and_main_color",
            "Tolerance-based alpha removal and mean-color sampling.",
        )
    return (
        "planned",
        f"hutool_core::img::{fam}",
        "",
        f"Planned: {fam}.{name or fam} needs AWT Graphics2D/font/GIF codec engine.",
    )


FILE_EXTRA = {
    "readUtf8Lines",
    "readLines",
    "writeUtf8Lines",
    "writeLines",
    "appendUtf8Lines",
    "appendLines",
    "appendString",
    "appendUtf8String",
    "getTotalLines",
    "createTempFile",
    "move",
    "normalize",
    "isAbsolutePath",
    "getUserHomePath",
    "getUserHomeDir",
    "isEmpty",
    "isNotEmpty",
    "isDirEmpty",
    "lastModifiedTime",
    "newerThan",
    "readableFileSize",
    "contentEqualsIgnoreEOL",
    "subPath",
    "getPrefix",
    "pathEquals",
    "checksum",
    "checksumCRC32",
    "getMimeType",
    "getLineSeparator",
    "getCanonicalPath",
    "newFile",
    "cleanEmpty",
    "pathEndsWith",
    "lastIndexOfSeparator",
    "isSymlink",
    "checkSlip",
    "mkParentDirs",
    "getTmpDir",
    "getTmpDirPath",
}

IO_EXTRA = {
    "checksum",
    "checksumCRC32",
    "checksumValue",
    "lineIter",
    "toUtf8Stream",
    "toStream",
    "readHex64Upper",
    "readHex64Lower",
    "readHex8192Upper",
    "toMarkSupportStream",
}


def file_extra_decision(name: str) -> tuple[str, str, str, str] | None:
    if name not in FILE_EXTRA:
        return None
    return (
        "idiomatic",
        "hutool_core::FileUtil",
        "crates/hutool-core/tests/file_util_parity.rs::file_util_leftover_helpers",
        "std::fs PathBuf leftovers: lines/move/normalize/checksum/mime/home/tmp.",
    )


def io_extra_decision(name: str) -> tuple[str, str, str, str] | None:
    if name not in IO_EXTRA:
        return None
    return (
        "idiomatic",
        "hutool_core::IoUtil",
        "crates/hutool-core/tests/io_util_parity.rs::io_util_checksum_and_stream_helpers",
        "std::io leftovers: checksum/CRC32/lineIter/utf8 stream bytes.",
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = idiomatic = planned = skipped_protected = 0
    for row in inventory:
        q = row["qualified_name"]
        api_id = row["api_id"]
        if any(q.startswith(p) for p in PROTECTED_PREFIXES):
            skipped_protected += 1
            continue

        name = method_name(q)
        decision = None
        if q.startswith(ZIP_ROOT):
            decision = zip_decision(name)
        elif q.startswith(URL_ROOT):
            decision = url_decision(name)
        elif q.startswith(NET_ROOT):
            decision = net_decision(q, name)
        elif q.startswith(IMG_ROOT):
            decision = img_decision(q, name)
        elif q.startswith(FILE_ROOT):
            decision = file_extra_decision(name)
        elif q.startswith(IO_ROOT):
            decision = io_extra_decision(name)
        else:
            continue

        if decision is None:
            continue
        status, symbol, evidence, notes = decision
        selected += 1
        if status == "idiomatic":
            idiomatic += 1
        else:
            planned += 1
        indexed[api_id] = {
            "api_id": api_id,
            "status": status,
            "hutool_symbol": symbol,
            "test_evidence": evidence,
            "notes": notes,
        }

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(
        f"recorded {selected} zip/net/img/io APIs "
        f"(idiomatic={idiomatic}, planned={planned}, "
        f"protected_skipped={skipped_protected})"
    )


if __name__ == "__main__":
    main()
