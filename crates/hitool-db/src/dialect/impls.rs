//! 具体方言实现 —— 对齐 Hutool `cn.hutool.db.dialect.impl.*`。
//!
//! JDBC `PreparedStatement` 工厂在 Rust 侧折叠为：方言名 + `Wrapper` 标识符引用，
//! 实际 SQL 由 `SqlBuilder` / `Db` 执行。

use crate::dialect::name::DialectName;
use crate::wrapper::Wrapper;

/// 对齐 Hutool `Dialect` 接口的最小表面：方言名与标识符包装。
pub trait Dialect: Send + Sync {
    /// 对齐 Java: `Dialect.dialectName()`。
    fn dialect_name(&self) -> DialectName;

    /// 对齐 Java: `Dialect.getWrapper()`。
    fn wrapper(&self) -> Wrapper;

    /// 对齐 Java: `Dialect.setWrapper(Wrapper)`。
    fn set_wrapper(&mut self, wrapper: Wrapper);
}

/// 通用 ANSI SQL 方言 —— 对齐 `AnsiSqlDialect`。
#[derive(Debug, Clone)]
pub struct AnsiSqlDialect {
    wrapper: Wrapper,
}

impl AnsiSqlDialect {
    /// 默认双引号标识符包装。
    #[must_use]
    pub fn new() -> Self {
        Self {
            wrapper: Wrapper::new('"'),
        }
    }

    /// 对齐 Java: `getWrapper()`。
    #[must_use]
    pub fn get_wrapper(&self) -> Wrapper {
        self.wrapper
    }

    /// 对齐 Java: `setWrapper(Wrapper)`。
    pub fn set_wrapper_value(&mut self, wrapper: Wrapper) {
        self.wrapper = wrapper;
    }

    /// 为分页 SQL 追加 `LIMIT/OFFSET`（ANSI/SQLite/MySQL/PG 风格）。
    #[must_use]
    pub fn wrap_page_sql(&self, sql: &str, page_size: u32, offset: u32) -> String {
        format!("{sql} LIMIT {page_size} OFFSET {offset}")
    }
}

impl Default for AnsiSqlDialect {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialect for AnsiSqlDialect {
    fn dialect_name(&self) -> DialectName {
        DialectName::Ansi
    }

    fn wrapper(&self) -> Wrapper {
        self.wrapper
    }

    fn set_wrapper(&mut self, wrapper: Wrapper) {
        self.wrapper = wrapper;
    }
}

macro_rules! simple_dialect {
    ($(#[$meta:meta])* $name:ident, $dialect:expr, $quote:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub struct $name {
            inner: AnsiSqlDialect,
        }

        impl $name {
            /// 对齐 Java 无参构造。
            #[must_use]
            pub fn new() -> Self {
                let mut inner = AnsiSqlDialect::new();
                inner.set_wrapper_value(Wrapper::new($quote));
                Self { inner }
            }

            /// 对齐 Java: `getWrapper()`。
            #[must_use]
            pub fn get_wrapper(&self) -> Wrapper {
                self.inner.get_wrapper()
            }

            /// 方言级 upsert 提示：返回建议 SQL 关键字（非 JDBC Statement）。
            #[must_use]
            pub fn upsert_hint(&self) -> &'static str {
                match $dialect {
                    DialectName::Mysql | DialectName::H2 | DialectName::Dm | DialectName::Hana => {
                        "INSERT ... ON DUPLICATE KEY UPDATE / MERGE"
                    }
                    DialectName::Postgresql => "INSERT ... ON CONFLICT DO UPDATE",
                    DialectName::Sqlite3 => "INSERT OR REPLACE",
                    DialectName::Phoenix => "UPSERT",
                    _ => "INSERT OR REPLACE / MERGE",
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Dialect for $name {
            fn dialect_name(&self) -> DialectName {
                $dialect
            }

            fn wrapper(&self) -> Wrapper {
                self.inner.get_wrapper()
            }

            fn set_wrapper(&mut self, wrapper: Wrapper) {
                self.inner.set_wrapper_value(wrapper);
            }
        }
    };
}

simple_dialect!(
    /// 对齐 Hutool `MysqlDialect`。
    MysqlDialect,
    DialectName::Mysql,
    '`'
);
simple_dialect!(
    /// 对齐 Hutool `PostgresqlDialect`。
    PostgresqlDialect,
    DialectName::Postgresql,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `OracleDialect`。
    OracleDialect,
    DialectName::Oracle,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `H2Dialect`。
    H2Dialect,
    DialectName::H2,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `Sqlite3Dialect`。
    Sqlite3Dialect,
    DialectName::Sqlite3,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `SqlServer2012Dialect`。
    SqlServer2012Dialect,
    DialectName::Sqlserver2012,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `DmDialect`。
    DmDialect,
    DialectName::Dm,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `HanaDialect`。
    HanaDialect,
    DialectName::Hana,
    '"'
);
simple_dialect!(
    /// 对齐 Hutool `PhoenixDialect`。
    PhoenixDialect,
    DialectName::Phoenix,
    '"'
);

impl OracleDialect {
    /// 对齐 Java: `OracleDialect.isNextVal(String)` —— 检测 `xxx.nextval` 序列表达式。
    #[must_use]
    pub fn is_next_val(value: &str) -> bool {
        value.to_ascii_lowercase().ends_with(".nextval")
    }
}
