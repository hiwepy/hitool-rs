#!/usr/bin/env python3
"""Record reviewed Hutool core.lang portable APIs (merge-only).

Wave1 families: Assert, Validator, RegexPool, PatternPool, Dict, Opt, Pair.
Wave2 leftovers: Snowflake/Pid/Console/UUID/Tuple/Weight*/Cache/Singleton/Range/
ObjectId/Version/ConsistentHash/Segment/Editor/Filter/Matcher/Replacer/Chain/
EnumItem/hash/* /NanoId/IdConstants.

Reflection-heavy (ClassScanner/JarClassLoader/reflect/caller/LambdaUtil/TypeReference)
stay planned/unsafe and are intentionally omitted.

Never deletes existing decisions; only upserts matching api_id rows.
"""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]

# (root_prefix, status, symbol, evidence, notes)
FAMILIES: list[tuple[str, str, str, str, str]] = [
    # --- Wave1 (already idiomatic; kept for merge-only reaffirm) ---
    (
        "cn.hutool.core.lang::Assert",
        "idiomatic",
        "hutool_core::Assert",
        "crates/hutool-core/src/lang/assert_.rs::assert_core_paths_cover_true_null_empty_between_and_equals",
        "Result/AssertError facades cover isTrue/isFalse/null/empty/blank/contain/index/between/equals without Java exception throwing.",
    ),
    (
        "cn.hutool.core.lang::Validator",
        "idiomatic",
        "hutool_core::Validator",
        "crates/hutool-core/tests/validator_parity.rs",
        "PatternPool/RegexPool-backed validators return bool or ValidateException; Java overload shapes map to Option/&str facades.",
    ),
    (
        "cn.hutool.core.lang::RegexPool",
        "idiomatic",
        "hutool_core::RegexPool",
        "crates/hutool-core/src/lang/regex_pool.rs::regex_pool_core_constants_compile_and_match",
        "Static &str constants mirror Hutool RegexPool; compilation/caching lives in PatternPool.",
    ),
    (
        "cn.hutool.core.lang::PatternPool",
        "idiomatic",
        "hutool_core::PatternPool",
        "crates/hutool-core/src/lang/pattern_pool.rs::pattern_pool_get_remove_clear_and_flag_key",
        "Mutex-backed Regex cache with CASE_INSENSITIVE flag keys; Arc sharing preserves get/remove/clear semantics.",
    ),
    (
        "cn.hutool.core.lang::Dict",
        "idiomatic",
        "hutool_core::DictUtil",
        "crates/hutool-core/src/dict.rs::dict_create_set_get_filter_parse_and_to_bean",
        "HashMap<String, serde_json::Value> bag with DictUtil getters and Serde parse/toBean; LinkedHashMap order and JavaBeans field inject stay adapted.",
    ),
    (
        "cn.hutool.core.lang::Opt",
        "idiomatic",
        "hutool_core::Opt",
        "crates/hutool-core/tests/opt_parity.rs",
        "Option-backed Opt with optional exception context covers of/ofNullable/ofTry/map/flatMap/orElse/throw paths.",
    ),
    (
        "cn.hutool.core.lang::Pair",
        "idiomatic",
        "hutool_core::Pair",
        "crates/hutool-core/src/lang/pair.rs::pair_of_getters_equals_and_display",
        "Owned generic Pair with of/getKey/getValue/equals/Display and XOR-style hashing.",
    ),
    # --- Wave2 portable leftovers ---
    (
        "cn.hutool.core.lang::Snowflake",
        "idiomatic",
        "hutool_core::Snowflake",
        "crates/hutool-core/src/lang/snowflake.rs::snowflake_next_id_and_field_extract",
        "Mutex-backed Snowflake with worker/datacenter/sequence encoding; Date epoch overloads map to Option<i64> millis.",
    ),
    (
        "cn.hutool.core.lang::Pid",
        "idiomatic",
        "hutool_core::Pid",
        "crates/hutool-core/src/lang/pid.rs::pid_get_is_positive_and_stable",
        "Cached std::process::id() replaces ManagementFactory MXBean pid@host parsing.",
    ),
    (
        "cn.hutool.core.lang::Console",
        "idiomatic",
        "hutool_core::Console",
        "crates/hutool-core/src/lang/console.rs::console_format_and_progress",
        "StrFormatter-backed log/print/error/progress return strings for testability; Scanner/input stay OS adapted.",
    ),
    (
        "cn.hutool.core.lang::ConsoleTable",
        "idiomatic",
        "hutool_core::ConsoleTable",
        "crates/hutool-core/src/lang/console_table.rs::console_table_render",
        "Header/body table render + Display; SBC width metrics simplified to join rendering.",
    ),
    (
        "cn.hutool.core.lang::UUID",
        "idiomatic",
        "hutool_core::UUID",
        "crates/hutool-core/src/lang/uuid_fast.rs::uuid_bits_from_string_and_compare",
        "uuid crate backed UUID with bits/fromString/version/variant/toString(simple); secure RNG overload folds into random_uuid.",
    ),
    (
        "cn.hutool.core.lang::Tuple",
        "idiomatic",
        "hutool_core::Tuple",
        "crates/hutool-core/src/lang/tuple.rs::tuple_get_size_contains_sub_and_display",
        "serde_json::Value members preserve Eq/Display; Stream/Spliterator map to iterators/to_list.",
    ),
    (
        "cn.hutool.core.lang::WeightRandom",
        "idiomatic",
        "hutool_core::WeightRandom",
        "crates/hutool-core/src/lang/weight_random.rs::weight_random_add_next_clear_and_weight_obj",
        "Weighted sampling with WeightObj nested type; Java constructors map to create/from_obj/from_iter.",
    ),
    (
        "cn.hutool.core.lang::WeightListRandom",
        "idiomatic",
        "hutool_core::WeightListRandom",
        "crates/hutool-core/src/lang/weight_list_random.rs::weight_list_random_samples",
        "Thin WeightRandom facade for list-style weighted picks.",
    ),
    (
        "cn.hutool.core.lang::SimpleCache",
        "idiomatic",
        "hutool_core::SimpleCache",
        "crates/hutool-core/src/lang/simple_cache.rs::simple_cache_put_get_or_put_and_clear",
        "RwLock HashMap cache with get/put/getOrPut/validPredicate/remove/clear; Mutable key map init adapted.",
    ),
    (
        "cn.hutool.core.lang::Singleton",
        "idiomatic",
        "hutool_core::Singleton",
        "crates/hutool-core/src/lang/singleton.rs::singleton_get_put_exists_remove",
        "TypeId-keyed reentrant singleton store; Class/String key overloads fold into typed get/put.",
    ),
    (
        "cn.hutool.core.lang::Range",
        "idiomatic",
        "hutool_core::Range",
        "crates/hutool-core/src/lang/range.rs::int_range_to_list",
        "Stepper closure Range with hasNext/next/to_list; lock/disableLock no-ops under Rust ownership.",
    ),
    (
        "cn.hutool.core.lang::ObjectId",
        "idiomatic",
        "hutool_core::ObjectId",
        "crates/hutool-core/src/lang/object_id.rs::object_id_valid_and_next",
        "Mongo-style 12-byte ObjectId with isValid/next/nextBytes/hyphen formatting.",
    ),
    (
        "cn.hutool.core.lang::Version",
        "idiomatic",
        "hutool_core::Version",
        "crates/hutool-core/src/lang/version.rs::version_of_compare_and_equals",
        "ModuleDescriptor-style Version parse/compare for sequence/pre/build tokens.",
    ),
    (
        "cn.hutool.core.lang::ConsistentHash",
        "idiomatic",
        "hutool_core::ConsistentHash",
        "crates/hutool-core/src/lang/consistent_hash.rs::consistent_hash_add_get_remove",
        "BTreeMap ring with FNV32 default hash and replica virtual nodes.",
    ),
    (
        "cn.hutool.core.lang::DefaultSegment",
        "idiomatic",
        "hutool_core::DefaultSegment",
        "crates/hutool-core/src/lang/default_segment.rs::default_segment_indexes_and_length",
        "Owned start/end segment implementing Segment.length().",
    ),
    (
        "cn.hutool.core.lang::Segment",
        "idiomatic",
        "hutool_core::Segment",
        "crates/hutool-core/src/lang/default_segment.rs::default_segment_indexes_and_length",
        "Trait for start/end index segments with default abs length.",
    ),
    (
        "cn.hutool.core.lang::Editor",
        "idiomatic",
        "hutool_core::Editor",
        "crates/hutool-core/src/lang/editor.rs::editor_maps_and_filters",
        "FnMut editor trait; None discards like Java Editor returning null.",
    ),
    (
        "cn.hutool.core.lang::Filter",
        "idiomatic",
        "hutool_core::Filter",
        "crates/hutool-core/src/lang/filter.rs::filter_accepts_matching",
        "Predicate Filter trait with filter_all helper.",
    ),
    (
        "cn.hutool.core.lang::Matcher",
        "idiomatic",
        "hutool_core::Matcher",
        "crates/hutool-core/src/lang/matcher.rs::matcher_filters_matching",
        "Functional Matcher trait (match -> match_item) with match_all helper.",
    ),
    (
        "cn.hutool.core.lang::Replacer",
        "idiomatic",
        "hutool_core::Replacer",
        "crates/hutool-core/src/lang/replacer.rs::replacer_maps_values",
        "Functional Replacer trait with replace_all helper.",
    ),
    (
        "cn.hutool.core.lang::Chain",
        "idiomatic",
        "hutool_core::Chain",
        "crates/hutool-core/src/lang/chain.rs::vec_chain_add_and_iterate",
        "Chain trait + VecChain iterable responsibility-chain container.",
    ),
    (
        "cn.hutool.core.lang::EnumItem",
        "idiomatic",
        "hutool_core::EnumItem",
        "crates/hutool-core/src/lang/enum_item.rs::enum_item_from_int_and_str",
        "EnumItem trait with name/text/intVal/fromInt/fromStr without Java enum reflection.",
    ),
    (
        "cn.hutool.core.lang.hash::Number128",
        "idiomatic",
        "hutool_core::Number128",
        "crates/hutool-core/src/lang/hash/metro_hash.rs::number128_getters_and_conversions",
        "128-bit low/high Number with Java Number conversion facades.",
    ),
    (
        "cn.hutool.core.lang.hash::MurmurHash",
        "idiomatic",
        "hutool_core::MurmurHash",
        "crates/hutool-core/src/lang/hash/murmur_hash.rs::murmur_hash32_hash64_and_hash128",
        "murmur3 hash32 + Hutool hash64 variant; hash128 derived from dual seeded hash64.",
    ),
    (
        "cn.hutool.core.lang.hash::CityHash",
        "idiomatic",
        "hutool_core::CityHash",
        "crates/hutool-core/src/lang/hash/city_hash.rs::hash128_matches_hutool_vector",
        "CityHash32/64/128 port with Hutool vector evidence.",
    ),
    (
        "cn.hutool.core.lang.hash::MetroHash",
        "idiomatic",
        "hutool_core::MetroHash",
        "crates/hutool-core/src/lang/hash/metro_hash.rs::number128_getters_and_conversions",
        "MetroHash64/128 returning Number128; seed overloads preserved.",
    ),
    (
        "cn.hutool.core.lang.hash::KetamaHash",
        "idiomatic",
        "hutool_core::KetamaHash",
        "crates/hutool-core/src/lang/hash/ketama_hash.rs::ketama_hash32_matches_hash64_low_bits",
        "MD5-based Ketama hash64/hash32 for consistent-hash node placement.",
    ),
    (
        "cn.hutool.core.lang.hash::Hash",
        "idiomatic",
        "hutool_core::Hash",
        "crates/hutool-core/src/lang/hash/ketama_hash.rs::ketama_hash32_matches_hash64_low_bits",
        "Generic Hash<T> trait returning i64 (Java Number).",
    ),
    (
        "cn.hutool.core.lang.hash::Hash32",
        "idiomatic",
        "hutool_core::Hash32",
        "crates/hutool-core/src/lang/hash/ketama_hash.rs::ketama_hash32_matches_hash64_low_bits",
        "Generic Hash32<T> trait for 32-bit digests.",
    ),
    (
        "cn.hutool.core.lang.hash::Hash64",
        "idiomatic",
        "hutool_core::Hash64",
        "crates/hutool-core/src/lang/hash/ketama_hash.rs::ketama_hash32_matches_hash64_low_bits",
        "Generic Hash64<T> trait for 64-bit digests.",
    ),
    (
        "cn.hutool.core.lang.hash::Hash128",
        "idiomatic",
        "hutool_core::Hash128",
        "crates/hutool-core/src/lang/hash/metro_hash.rs::number128_getters_and_conversions",
        "Generic Hash128<T> trait returning Number128.",
    ),
    (
        "cn.hutool.core.lang.id::NanoId",
        "idiomatic",
        "hutool_core::NanoId",
        "crates/hutool-core/src/lang/id/nano_id.rs::nano_id_default_and_sized",
        "URL-safe NanoId with default/size/alphabet+JavaRandom overloads.",
    ),
    (
        "cn.hutool.core.lang.id::IdConstants",
        "idiomatic",
        "hutool_core::IdConstants",
        "crates/hutool-core/src/lang/id/id_constants.rs::id_constants_default_snowflake",
        "Default worker/datacenter IDs + shared Snowflake singleton.",
    ),
]


def matches(qualified_name: str, root: str) -> bool:
    return (
        qualified_name == root
        or qualified_name.startswith(root + "::")
        or qualified_name.startswith(root + "#")
    )


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = 0
    by_family: dict[str, int] = {}
    for row in inventory:
        qn = row["qualified_name"]
        for root, status, symbol, evidence, notes in FAMILIES:
            if not matches(qn, root):
                continue
            selected += 1
            by_family[root] = by_family.get(root, 0) + 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": status,
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
            break

    # Wave1 alone was ~255; Wave2 adds ~170 portable leftovers → expect >=400.
    if selected < 400:
        raise SystemExit(f"expected >=400 lang core APIs, selected {selected}: {by_family}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())

    print(f"recorded {selected} Hutool lang core APIs (merge-only): {by_family}")


if __name__ == "__main__":
    main()
