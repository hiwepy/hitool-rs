#!/usr/bin/env python3
"""Record Hutool db APIs against hitool-db idiomatic surfaces / planned gaps."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hitool_symbol", "test_evidence", "notes"]
MODULE = "hutool-db"
EXPECTED = 1041

# Class → (symbol, evidence, notes)
IDIOMATIC: dict[str, tuple[str, str, str]] = {
    "Entity": (
        "hitool_db::Entity",
        "crates/hitool-db/tests/db_parity_gap.rs::entity_parse_test",
        "Entity is an owned column map; Hutool bean reflection overloads map to typed field setters.",
    ),
    "ActiveEntity": (
        "hitool_db::ActiveEntity",
        "crates/hitool-db/tests/db_parity_gap.rs::entity_parse_test",
        "ActiveEntity wraps Entity with session-bound CRUD without Java proxy magic.",
    ),
    "Db": (
        "hitool_db::Db",
        "crates/hitool-db/tests/db_parity_gap.rs::db_query_test",
        "Db facade runs over sqlx pools; Hutool DataSource injection becomes PoolConfig/runtime selection.",
    ),
    "Session": (
        "hitool_db::Session",
        "crates/hitool-db/tests/db_parity_gap.rs::session_trans_test",
        "Session provides explicit transaction scopes instead of ThreadLocal Connection.",
    ),
    "Page": (
        "hitool_db::Page",
        "crates/hitool-db/tests/db_parity.rs::page_test",
        "Page is a generic owned result container matching Hutool page math.",
    ),
    "PageResult": (
        "hitool_db::PageResult",
        "crates/hitool-db/tests/db_parity.rs::page_test",
        "PageResult preserves total/pageSize/records semantics for query pages.",
    ),
    "Condition": (
        "hitool_db::Condition",
        "crates/hitool-db/tests/db_parity_gap.rs::condition_to_string_test",
        "Condition builds parameterized SQL fragments without JDBC Statement mutation.",
    ),
    "ConditionBuilder": (
        "hitool_db::ConditionBuilder",
        "crates/hitool-db/tests/db_parity_gap.rs::condition_builder_build_test",
        "ConditionBuilder concatenates AND/OR groups into one SQL + param list.",
    ),
    "ConditionGroup": (
        "hitool_db::ConditionGroup",
        "crates/hitool-db/tests/db_parity_gap.rs::condition_group_condition_group_to_string_test",
        "ConditionGroup nests logical groups with explicit LogicalOperator.",
    ),
    "SqlBuilder": (
        "hitool_db::SqlBuilder",
        "crates/hitool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "SqlBuilder is a fluent owned builder; Wrapper quoting replaces dialect-specific IdentifierQuote.",
    ),
    "NamedSql": (
        "hitool_db::NamedSql",
        "crates/hitool-db/tests/db_parity_gap.rs::named_sql_parse_test",
        "NamedSql expands :name placeholders into positional params.",
    ),
    "Order": (
        "hitool_db::Order",
        "crates/hitool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "Order pairs field + Direction for ORDER BY clauses.",
    ),
    "Direction": (
        "hitool_db::Direction",
        "crates/hitool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "ASC/DESC enum mirrors Hutool Direction.",
    ),
    "LogicalOperator": (
        "hitool_db::LogicalOperator",
        "crates/hitool-db/tests/db_parity_gap.rs::condition_builder_build_test",
        "AND/OR operators are an exhaustive Rust enum.",
    ),
    "Wrapper": (
        "hitool_db::Wrapper",
        "crates/hitool-db/tests/db_parity_gap.rs::wrapper_test",
        "Wrapper applies identifier quoting without Dialect SPI factories.",
    ),
    "DataSourceWrapper": (
        "hitool_db::DataSourceWrapper",
        "crates/hitool-db/tests/db_parity_gap.rs::db_query_test",
        "DataSourceWrapper holds pool metadata; Java javax.sql.DataSource adapters are not mirrored.",
    ),
    "Table": (
        "hitool_db::Table",
        "crates/hitool-db/tests/db_parity_gap.rs::meta_util_get_table_meta_test",
        "Table metadata carries primary-key and index summary for SQLite-backed MetaUtil.",
    ),
    "MetaUtil": (
        "hitool_db::{get_tables,get_table_meta,get_column_names}",
        "crates/hitool-db/tests/db_parity_gap.rs::meta_util_get_tables_test",
        "MetaUtil statics become free functions over sqlx connections.",
    ),
    "SqlUtil": (
        "hitool_db::{remove_outer_order_by,is_in_clause,build_like_value,build_conditions}",
        "crates/hitool-db/tests/db_parity_gap.rs::sql_builder_like_test",
        "SqlUtil helpers are pure functions for LIKE/IN/ORDER BY rewriting.",
    ),
    "SqlFormatter": (
        "hitool_db::sql::formatter::format",
        "crates/hitool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "SqlFormatter is a lightweight pretty-printer without Java token streams.",
    ),
    "DbUtil": (
        "hitool_db::{Db,memory_pool,seed_hutool_user_fixture}",
        "crates/hitool-db/tests/db_parity_gap.rs::db_query_test",
        "DbUtil factory methods map to pool helpers and Db constructors.",
    ),
    "DbRuntimeException": (
        "hitool_db::{DbError,DbRuntimeError}",
        "crates/hitool-db/tests/db_parity.rs::page_request_invalid_size_test",
        "Checked DbRuntimeException maps to typed DbError/DbRuntimeError.",
    ),
    "DriverNamePool": (
        "hitool_db::dialect::driver_name_pool",
        "crates/hitool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "JDBC driver class-name constants are preserved as Rust consts.",
    ),
    "DriverUtil": (
        "hitool_db::identify_driver",
        "crates/hitool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "DriverUtil.identifyDriver maps to identify_driver text matching.",
    ),
    "DialectFactory": (
        "hitool_db::{identify_driver,identify_driver_from_text}",
        "crates/hitool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "DialectFactory.identifyDriver is a pure string classifier without ClassLoader.",
    ),
    "MongoDS": (
        "hitool_db::MongoDs",
        "crates/hitool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "MongoDS is a lightweight config holder; full Mongo client wiring remains optional.",
    ),
    "MongoFactory": (
        "hitool_db::MongoDs",
        "crates/hitool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "MongoFactory.getDS collapses to MongoDs::new construction.",
    ),
    "RedisDS": (
        "hitool_db::RedisDs",
        "crates/hitool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "RedisDS is a config holder aligned with Hutool RedisDS create helpers.",
    ),
    # --- Agent-4 facades: map planned classes onto existing session/sql/meta surfaces ---
    "Query": (
        "hitool_db::Query",
        "crates/hitool-db/tests/db_facade_parity.rs::query_and_sql_log_facades",
        "Query carries tables/fields/where/page; builds over Entity + Condition without JDBC.",
    ),
    "SqlLog": (
        "hitool_db::SqlLog",
        "crates/hitool-db/tests/db_facade_parity.rs::query_and_sql_log_facades",
        "SqlLog is an explicit injectable logger; no global INSTANCE singleton.",
    ),
    "DbConfig": (
        "hitool_db::DbConfig",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DbConfig maps to PoolConfig + DataSourceWrapper without opening JDBC connections.",
    ),
    "DbSetting": (
        "hitool_db::DbSetting",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DbSetting builds DbConfig from key/value maps instead of Hutool Setting globals.",
    ),
    "SimpleDataSource": (
        "hitool_db::SimpleDataSource",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "SimpleDataSource holds URL/user/pass/driver metadata for explicit pool injection.",
    ),
    "PooledDataSource": (
        "hitool_db::PooledDataSource",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "PooledDataSource exposes DbConfig → PoolConfig mapping; SQLx owns real pools.",
    ),
    "DSFactory": (
        "hitool_db::DsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DSFactory creates owned SimpleDataSource configs; GlobalDSFactory remains planned.",
    ),
    "AbstractDSFactory": (
        "hitool_db::AbstractDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "AbstractDSFactory is a type alias of DsFactory under explicit injection.",
    ),
    "SimpleDSFactory": (
        "hitool_db::SimpleDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Named factory facade over DsFactory without JDBC SPI.",
    ),
    "PooledDSFactory": (
        "hitool_db::PooledDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Named factory facade over DsFactory without JDBC SPI.",
    ),
    "HikariDSFactory": (
        "hitool_db::HikariDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Hikari-named factory returns config only; does not embed HikariCP.",
    ),
    "DruidDSFactory": (
        "hitool_db::DruidDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Druid-named factory returns config only.",
    ),
    "DbcpDSFactory": (
        "hitool_db::DbcpDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DBCP-named factory returns config only.",
    ),
    "C3p0DSFactory": (
        "hitool_db::C3p0DsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "C3P0-named factory returns config only.",
    ),
    "TomcatDSFactory": (
        "hitool_db::TomcatDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Tomcat-named factory returns config only.",
    ),
    "BeeDSFactory": (
        "hitool_db::BeeDsFactory",
        "crates/hitool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "BeeCP-named factory returns config only.",
    ),
    "DialectName": (
        "hitool_db::DialectName",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "DialectName enum mirrors Hutool dialect identifiers.",
    ),
    "Dialect": (
        "hitool_db::Dialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "Dialect trait exposes dialect_name + Wrapper without JDBC PreparedStatement factories.",
    ),
    "AnsiSqlDialect": (
        "hitool_db::AnsiSqlDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "AnsiSqlDialect provides Wrapper + LIMIT/OFFSET page wrapping.",
    ),
    "MysqlDialect": (
        "hitool_db::MysqlDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "MysqlDialect uses backtick Wrapper; upsert hint is metadata only.",
    ),
    "PostgresqlDialect": (
        "hitool_db::PostgresqlDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "PostgresqlDialect is a thin DialectName + Wrapper facade.",
    ),
    "OracleDialect": (
        "hitool_db::OracleDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "OracleDialect includes is_next_val sequence detection.",
    ),
    "H2Dialect": (
        "hitool_db::H2Dialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "H2Dialect is a thin DialectName + Wrapper facade.",
    ),
    "Sqlite3Dialect": (
        "hitool_db::Sqlite3Dialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "Sqlite3Dialect aligns with the default SQLx SQLite engine.",
    ),
    "SqlServer2012Dialect": (
        "hitool_db::SqlServer2012Dialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "SqlServer2012Dialect is a thin DialectName + Wrapper facade.",
    ),
    "DmDialect": (
        "hitool_db::DmDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "DmDialect is a thin DialectName + Wrapper facade.",
    ),
    "HanaDialect": (
        "hitool_db::HanaDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "HanaDialect is a thin DialectName + Wrapper facade.",
    ),
    "PhoenixDialect": (
        "hitool_db::PhoenixDialect",
        "crates/hitool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "PhoenixDialect is a thin DialectName + Wrapper facade.",
    ),
    "Column": (
        "hitool_db::Column",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "Column metadata struct mirrors Hutool getters/setters without JDBC ResultSetMetaData.",
    ),
    "IndexInfo": (
        "hitool_db::IndexInfo",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "IndexInfo holds index/column metadata for MetaUtil consumers.",
    ),
    "ColumnIndexInfo": (
        "hitool_db::ColumnIndexInfo",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "ColumnIndexInfo describes a single index column and sort direction.",
    ),
    "TableType": (
        "hitool_db::TableType",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "TableType classifies TABLE/VIEW/OTHER.",
    ),
    "JdbcType": (
        "hitool_db::JdbcType",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "JdbcType is a common-type subset used by Column.type_enum.",
    ),
    "HandleHelper": (
        "hitool_db::HandleHelper",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "HandleHelper converts column maps / Entity rows without java.sql.ResultSet.",
    ),
    "EntityHandler": (
        "hitool_db::EntityHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntityHandler returns the first Entity row.",
    ),
    "EntityListHandler": (
        "hitool_db::EntityListHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntityListHandler returns all Entity rows.",
    ),
    "EntitySetHandler": (
        "hitool_db::EntitySetHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntitySetHandler deduplicates Entity rows by field signature.",
    ),
    "PageResultHandler": (
        "hitool_db::PageResultHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "PageResultHandler wraps rows into PageResult metadata.",
    ),
    "BeanHandler": (
        "hitool_db::BeanHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "BeanHandler deserializes the first Entity via serde.",
    ),
    "BeanListHandler": (
        "hitool_db::BeanListHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "BeanListHandler deserializes Entity lists via serde.",
    ),
    "NumberHandler": (
        "hitool_db::NumberHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "NumberHandler reads the first cell as i64.",
    ),
    "StringHandler": (
        "hitool_db::StringHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "StringHandler reads the first cell as String.",
    ),
    "ValueListHandler": (
        "hitool_db::ValueListHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "ValueListHandler returns per-row value vectors.",
    ),
    "RsHandler": (
        "hitool_db::RsHandler",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "RsHandler trait is the Rust stand-in for Hutool RsHandler<T>.",
    ),
    "AbstractDb": (
        "hitool_db::AbstractDb",
        "crates/hitool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "AbstractDb delegates CRUD/page/query to Db over an injected SQLx pool.",
    ),
    "SqlConnRunner": (
        "hitool_db::SqlConnRunner",
        "crates/hitool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "SqlConnRunner is a connection-scoped facade over AbstractDb/Db.",
    ),
    "DialectRunner": (
        "hitool_db::DialectRunner",
        "crates/hitool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "DialectRunner pairs DialectName/Wrapper with SqlConnRunner.",
    ),
    "SqlExecutor": (
        "hitool_db::SqlExecutor",
        "crates/hitool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "SqlExecutor static helpers require an explicit Db (no ThreadLocal Connection).",
    ),
    "TransactionLevel": (
        "hitool_db::TransactionLevel",
        "crates/hitool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "TransactionLevel enumerates JDBC isolation constants as metadata.",
    ),
}


def family(qualified_name: str) -> str:
    return qualified_name.split("::", 1)[1].split("#", 1)[0].split("::", 1)[0]


def main() -> None:
    with INVENTORY.open(encoding="utf-8", newline="") as stream:
        inventory = list(csv.DictReader(stream))
    with DECISIONS.open(encoding="utf-8", newline="") as stream:
        indexed = {row["api_id"]: row for row in csv.DictReader(stream)}

    selected = idiomatic = planned = 0
    for row in inventory:
        if row["module"] != MODULE:
            continue
        selected += 1
        class_name = family(row["qualified_name"])
        if class_name in IDIOMATIC:
            idiomatic += 1
            symbol, evidence, notes = IDIOMATIC[class_name]
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "idiomatic",
                "hitool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
        else:
            planned += 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hitool_symbol": "",
                "test_evidence": "",
                "notes": (
                    f"{class_name} not yet covered by hitool-db; "
                    "pool/handler/dialect SPI surfaces remain deferred."
                ),
            }

    if selected != EXPECTED:
        raise SystemExit(f"expected {EXPECTED} {MODULE} APIs, selected {selected}")

    with DECISIONS.open("w", encoding="utf-8", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(indexed.values())
    print(f"recorded {selected} {MODULE} APIs (idiomatic={idiomatic}, planned={planned})")


if __name__ == "__main__":
    main()
