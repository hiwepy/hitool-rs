#!/usr/bin/env python3
"""Record Hutool db APIs against hutool-db idiomatic surfaces / planned gaps."""

from __future__ import annotations

import csv
from pathlib import Path

INVENTORY = Path("parity/hutool-v5.8.46-api.csv")
DECISIONS = Path("parity/decisions.csv")
FIELDS = ["api_id", "status", "hutool_symbol", "test_evidence", "notes"]
MODULE = "hutool-db"
EXPECTED = 1041

# Class → (symbol, evidence, notes)
IDIOMATIC: dict[str, tuple[str, str, str]] = {
    "Entity": (
        "hutool_db::Entity",
        "crates/hutool-db/tests/db_parity_gap.rs::entity_parse_test",
        "Entity is an owned column map; Hutool bean reflection overloads map to typed field setters.",
    ),
    "ActiveEntity": (
        "hutool_db::ActiveEntity",
        "crates/hutool-db/tests/db_parity_gap.rs::entity_parse_test",
        "ActiveEntity wraps Entity with session-bound CRUD without Java proxy magic.",
    ),
    "Db": (
        "hutool_db::Db",
        "crates/hutool-db/tests/db_parity_gap.rs::db_query_test",
        "Db facade runs over sqlx pools; Hutool DataSource injection becomes PoolConfig/runtime selection.",
    ),
    "Session": (
        "hutool_db::Session",
        "crates/hutool-db/tests/db_parity_gap.rs::session_trans_test",
        "Session provides explicit transaction scopes instead of ThreadLocal Connection.",
    ),
    "Page": (
        "hutool_db::Page",
        "crates/hutool-db/tests/db_parity.rs::page_test",
        "Page is a generic owned result container matching Hutool page math.",
    ),
    "PageResult": (
        "hutool_db::PageResult",
        "crates/hutool-db/tests/db_parity.rs::page_test",
        "PageResult preserves total/pageSize/records semantics for query pages.",
    ),
    "Condition": (
        "hutool_db::Condition",
        "crates/hutool-db/tests/db_parity_gap.rs::condition_to_string_test",
        "Condition builds parameterized SQL fragments without JDBC Statement mutation.",
    ),
    "ConditionBuilder": (
        "hutool_db::ConditionBuilder",
        "crates/hutool-db/tests/db_parity_gap.rs::condition_builder_build_test",
        "ConditionBuilder concatenates AND/OR groups into one SQL + param list.",
    ),
    "ConditionGroup": (
        "hutool_db::ConditionGroup",
        "crates/hutool-db/tests/db_parity_gap.rs::condition_group_condition_group_to_string_test",
        "ConditionGroup nests logical groups with explicit LogicalOperator.",
    ),
    "SqlBuilder": (
        "hutool_db::SqlBuilder",
        "crates/hutool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "SqlBuilder is a fluent owned builder; Wrapper quoting replaces dialect-specific IdentifierQuote.",
    ),
    "NamedSql": (
        "hutool_db::NamedSql",
        "crates/hutool-db/tests/db_parity_gap.rs::named_sql_parse_test",
        "NamedSql expands :name placeholders into positional params.",
    ),
    "Order": (
        "hutool_db::Order",
        "crates/hutool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "Order pairs field + Direction for ORDER BY clauses.",
    ),
    "Direction": (
        "hutool_db::Direction",
        "crates/hutool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "ASC/DESC enum mirrors Hutool Direction.",
    ),
    "LogicalOperator": (
        "hutool_db::LogicalOperator",
        "crates/hutool-db/tests/db_parity_gap.rs::condition_builder_build_test",
        "AND/OR operators are an exhaustive Rust enum.",
    ),
    "Wrapper": (
        "hutool_db::Wrapper",
        "crates/hutool-db/tests/db_parity_gap.rs::wrapper_test",
        "Wrapper applies identifier quoting without Dialect SPI factories.",
    ),
    "DataSourceWrapper": (
        "hutool_db::DataSourceWrapper",
        "crates/hutool-db/tests/db_parity_gap.rs::db_query_test",
        "DataSourceWrapper holds pool metadata; Java javax.sql.DataSource adapters are not mirrored.",
    ),
    "Table": (
        "hutool_db::Table",
        "crates/hutool-db/tests/db_parity_gap.rs::meta_util_get_table_meta_test",
        "Table metadata carries primary-key and index summary for SQLite-backed MetaUtil.",
    ),
    "MetaUtil": (
        "hutool_db::{get_tables,get_table_meta,get_column_names}",
        "crates/hutool-db/tests/db_parity_gap.rs::meta_util_get_tables_test",
        "MetaUtil statics become free functions over sqlx connections.",
    ),
    "SqlUtil": (
        "hutool_db::{remove_outer_order_by,is_in_clause,build_like_value,build_conditions}",
        "crates/hutool-db/tests/db_parity_gap.rs::sql_builder_like_test",
        "SqlUtil helpers are pure functions for LIKE/IN/ORDER BY rewriting.",
    ),
    "SqlFormatter": (
        "hutool_db::sql::formatter::format",
        "crates/hutool-db/tests/db_parity_gap.rs::sql_builder_order_by_test",
        "SqlFormatter is a lightweight pretty-printer without Java token streams.",
    ),
    "DbUtil": (
        "hutool_db::{Db,memory_pool,seed_hutool_user_fixture}",
        "crates/hutool-db/tests/db_parity_gap.rs::db_query_test",
        "DbUtil factory methods map to pool helpers and Db constructors.",
    ),
    "DbRuntimeException": (
        "hutool_db::{DbError,DbRuntimeError}",
        "crates/hutool-db/tests/db_parity.rs::page_request_invalid_size_test",
        "Checked DbRuntimeException maps to typed DbError/DbRuntimeError.",
    ),
    "DriverNamePool": (
        "hutool_db::dialect::driver_name_pool",
        "crates/hutool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "JDBC driver class-name constants are preserved as Rust consts.",
    ),
    "DriverUtil": (
        "hutool_db::identify_driver",
        "crates/hutool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "DriverUtil.identifyDriver maps to identify_driver text matching.",
    ),
    "DialectFactory": (
        "hutool_db::{identify_driver,identify_driver_from_text}",
        "crates/hutool-db/tests/db_parity_gap.rs::dialect_factory_identify_driver_test",
        "DialectFactory.identifyDriver is a pure string classifier without ClassLoader.",
    ),
    "MongoDS": (
        "hutool_db::MongoDs",
        "crates/hutool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "MongoDS is a lightweight config holder; full Mongo client wiring remains optional.",
    ),
    "MongoFactory": (
        "hutool_db::MongoDs",
        "crates/hutool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "MongoFactory.getDS collapses to MongoDs::new construction.",
    ),
    "RedisDS": (
        "hutool_db::RedisDs",
        "crates/hutool-db/tests/db_parity_gap.rs::mongo_db_mongo_ds_test",
        "RedisDS is a config holder aligned with Hutool RedisDS create helpers.",
    ),
    # --- Agent-4 facades: map planned classes onto existing session/sql/meta surfaces ---
    "Query": (
        "hutool_db::Query",
        "crates/hutool-db/tests/db_facade_parity.rs::query_and_sql_log_facades",
        "Query carries tables/fields/where/page; builds over Entity + Condition without JDBC.",
    ),
    "SqlLog": (
        "hutool_db::SqlLog",
        "crates/hutool-db/tests/db_facade_parity.rs::query_and_sql_log_facades",
        "SqlLog is an explicit injectable logger; no global INSTANCE singleton.",
    ),
    "DbConfig": (
        "hutool_db::DbConfig",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DbConfig maps to PoolConfig + DataSourceWrapper without opening JDBC connections.",
    ),
    "DbSetting": (
        "hutool_db::DbSetting",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DbSetting builds DbConfig from key/value maps instead of Hutool Setting globals.",
    ),
    "SimpleDataSource": (
        "hutool_db::SimpleDataSource",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "SimpleDataSource holds URL/user/pass/driver metadata for explicit pool injection.",
    ),
    "PooledDataSource": (
        "hutool_db::PooledDataSource",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "PooledDataSource exposes DbConfig → PoolConfig mapping; SQLx owns real pools.",
    ),
    "DSFactory": (
        "hutool_db::DsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DSFactory creates owned SimpleDataSource configs; GlobalDSFactory remains planned.",
    ),
    "AbstractDSFactory": (
        "hutool_db::AbstractDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "AbstractDSFactory is a type alias of DsFactory under explicit injection.",
    ),
    "SimpleDSFactory": (
        "hutool_db::SimpleDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Named factory facade over DsFactory without JDBC SPI.",
    ),
    "PooledDSFactory": (
        "hutool_db::PooledDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Named factory facade over DsFactory without JDBC SPI.",
    ),
    "HikariDSFactory": (
        "hutool_db::HikariDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Hikari-named factory returns config only; does not embed HikariCP.",
    ),
    "DruidDSFactory": (
        "hutool_db::DruidDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Druid-named factory returns config only.",
    ),
    "DbcpDSFactory": (
        "hutool_db::DbcpDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "DBCP-named factory returns config only.",
    ),
    "C3p0DSFactory": (
        "hutool_db::C3p0DsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "C3P0-named factory returns config only.",
    ),
    "TomcatDSFactory": (
        "hutool_db::TomcatDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "Tomcat-named factory returns config only.",
    ),
    "BeeDSFactory": (
        "hutool_db::BeeDsFactory",
        "crates/hutool-db/tests/db_facade_parity.rs::ds_config_and_factory_are_explicit",
        "BeeCP-named factory returns config only.",
    ),
    "DialectName": (
        "hutool_db::DialectName",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "DialectName enum mirrors Hutool dialect identifiers.",
    ),
    "Dialect": (
        "hutool_db::Dialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "Dialect trait exposes dialect_name + Wrapper without JDBC PreparedStatement factories.",
    ),
    "AnsiSqlDialect": (
        "hutool_db::AnsiSqlDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "AnsiSqlDialect provides Wrapper + LIMIT/OFFSET page wrapping.",
    ),
    "MysqlDialect": (
        "hutool_db::MysqlDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "MysqlDialect uses backtick Wrapper; upsert hint is metadata only.",
    ),
    "PostgresqlDialect": (
        "hutool_db::PostgresqlDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "PostgresqlDialect is a thin DialectName + Wrapper facade.",
    ),
    "OracleDialect": (
        "hutool_db::OracleDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "OracleDialect includes is_next_val sequence detection.",
    ),
    "H2Dialect": (
        "hutool_db::H2Dialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "H2Dialect is a thin DialectName + Wrapper facade.",
    ),
    "Sqlite3Dialect": (
        "hutool_db::Sqlite3Dialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "Sqlite3Dialect aligns with the default SQLx SQLite engine.",
    ),
    "SqlServer2012Dialect": (
        "hutool_db::SqlServer2012Dialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "SqlServer2012Dialect is a thin DialectName + Wrapper facade.",
    ),
    "DmDialect": (
        "hutool_db::DmDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "DmDialect is a thin DialectName + Wrapper facade.",
    ),
    "HanaDialect": (
        "hutool_db::HanaDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "HanaDialect is a thin DialectName + Wrapper facade.",
    ),
    "PhoenixDialect": (
        "hutool_db::PhoenixDialect",
        "crates/hutool-db/tests/db_facade_parity.rs::dialect_name_and_impls_align",
        "PhoenixDialect is a thin DialectName + Wrapper facade.",
    ),
    "Column": (
        "hutool_db::Column",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "Column metadata struct mirrors Hutool getters/setters without JDBC ResultSetMetaData.",
    ),
    "IndexInfo": (
        "hutool_db::IndexInfo",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "IndexInfo holds index/column metadata for MetaUtil consumers.",
    ),
    "ColumnIndexInfo": (
        "hutool_db::ColumnIndexInfo",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "ColumnIndexInfo describes a single index column and sort direction.",
    ),
    "TableType": (
        "hutool_db::TableType",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "TableType classifies TABLE/VIEW/OTHER.",
    ),
    "JdbcType": (
        "hutool_db::JdbcType",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "JdbcType is a common-type subset used by Column.type_enum.",
    ),
    "HandleHelper": (
        "hutool_db::HandleHelper",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "HandleHelper converts column maps / Entity rows without java.sql.ResultSet.",
    ),
    "EntityHandler": (
        "hutool_db::EntityHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntityHandler returns the first Entity row.",
    ),
    "EntityListHandler": (
        "hutool_db::EntityListHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntityListHandler returns all Entity rows.",
    ),
    "EntitySetHandler": (
        "hutool_db::EntitySetHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "EntitySetHandler deduplicates Entity rows by field signature.",
    ),
    "PageResultHandler": (
        "hutool_db::PageResultHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "PageResultHandler wraps rows into PageResult metadata.",
    ),
    "BeanHandler": (
        "hutool_db::BeanHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "BeanHandler deserializes the first Entity via serde.",
    ),
    "BeanListHandler": (
        "hutool_db::BeanListHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "BeanListHandler deserializes Entity lists via serde.",
    ),
    "NumberHandler": (
        "hutool_db::NumberHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "NumberHandler reads the first cell as i64.",
    ),
    "StringHandler": (
        "hutool_db::StringHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "StringHandler reads the first cell as String.",
    ),
    "ValueListHandler": (
        "hutool_db::ValueListHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "ValueListHandler returns per-row value vectors.",
    ),
    "RsHandler": (
        "hutool_db::RsHandler",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
        "RsHandler trait is the Rust stand-in for Hutool RsHandler<T>.",
    ),
    "AbstractDb": (
        "hutool_db::AbstractDb",
        "crates/hutool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "AbstractDb delegates CRUD/page/query to Db over an injected SQLx pool.",
    ),
    "SqlConnRunner": (
        "hutool_db::SqlConnRunner",
        "crates/hutool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "SqlConnRunner is a connection-scoped facade over AbstractDb/Db.",
    ),
    "DialectRunner": (
        "hutool_db::DialectRunner",
        "crates/hutool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "DialectRunner pairs DialectName/Wrapper with SqlConnRunner.",
    ),
    "SqlExecutor": (
        "hutool_db::SqlExecutor",
        "crates/hutool-db/tests/db_facade_parity.rs::runners_delegate_to_db",
        "SqlExecutor static helpers require an explicit Db (no ThreadLocal Connection).",
    ),
    "TransactionLevel": (
        "hutool_db::TransactionLevel",
        "crates/hutool-db/tests/db_facade_parity.rs::meta_column_index_and_handlers",
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
                "hutool_symbol": symbol,
                "test_evidence": evidence,
                "notes": notes,
            }
        else:
            planned += 1
            indexed[row["api_id"]] = {
                "api_id": row["api_id"],
                "status": "planned",
                "hutool_symbol": "",
                "test_evidence": "",
                "notes": (
                    f"{class_name} not yet covered by hutool-db; "
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
