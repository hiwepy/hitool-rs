//! SQL 方言 —— 对齐 Hutool `cn.hutool.db.dialect.*`。

pub mod driver_name_pool;
pub mod driver_util;
pub mod factory;
pub mod impls;
pub mod name;

pub use driver_util::identify_driver;
pub use factory::identify_driver as identify_driver_from_text;
pub use impls::{
    AnsiSqlDialect, Dialect, DmDialect, H2Dialect, HanaDialect, MysqlDialect, OracleDialect,
    PhoenixDialect, PostgresqlDialect, SqlServer2012Dialect, Sqlite3Dialect,
};
pub use name::DialectName;
