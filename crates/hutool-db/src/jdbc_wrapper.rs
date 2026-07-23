//! JDBC Wrapper facade，对齐 hutool 的 `cn.hutool.db.sql.StatementWrapper`、
//! `cn.hutool.db.ds.pooled.ConnectionWraper`、`cn.hutool.db.ds.simple.AbstractDataSource`。
//!
//! **仅提供 trait 抽象 + 类型占位**。具体 JDBC 操作（基于 `java.sql.PreparedStatement`
//! / `java.sql.Connection`）是 `javax_sql_spi` unsafe-to-copy，Rust 用 SQLx 替代。
//!
//! 这些 trait 让 hutool 的 JDBC API 在 Rust 端"有迹可循"，方便迁移用户理解 API 形状。

use std::any::Any;

/// JDBC Statement 包装器 trait，对齐 `cn.hutool.db.sql.StatementWrapper`。
///
/// Java 继承 `PreparedStatementWrapper implements PreparedStatement`；
/// Rust 用 trait 提供完整 JDBC API 形状。具体实现需要 SQLx 或 rusqlite。
pub trait StatementWrapper: Send + Sync {
    // ─── 通用 Statement 方法 ───
    fn execute_query_sql(&self, sql: &str) -> Result<Box<dyn Any>, DbWrapperError>;
    fn execute_update_sql(&self, sql: &str) -> Result<i64, DbWrapperError>;
    fn close(&self) -> Result<(), DbWrapperError>;
    fn get_max_field_size(&self) -> Result<i32, DbWrapperError>;
    fn set_max_field_size(&self, max: i32) -> Result<(), DbWrapperError>;
    fn get_max_rows(&self) -> Result<i32, DbWrapperError>;
    fn set_max_rows(&self, max: i32) -> Result<(), DbWrapperError>;
    fn set_escape_processing(&self, enable: bool) -> Result<(), DbWrapperError>;
    fn get_query_timeout(&self) -> Result<i32, DbWrapperError>;
    fn set_query_timeout(&self, seconds: i32) -> Result<(), DbWrapperError>;
    fn cancel(&self) -> Result<(), DbWrapperError>;
    fn set_cursor_name(&self, name: &str) -> Result<(), DbWrapperError>;
    fn execute_sql(&self, sql: &str) -> Result<bool, DbWrapperError>;
    fn get_update_count(&self) -> Result<i64, DbWrapperError>;
    fn set_fetch_direction(&self, direction: i32) -> Result<(), DbWrapperError>;
    fn get_fetch_direction(&self) -> Result<i32, DbWrapperError>;
    fn set_fetch_size(&self, rows: i32) -> Result<(), DbWrapperError>;
    fn get_fetch_size(&self) -> Result<i32, DbWrapperError>;
    fn add_batch(&self, sql: &str) -> Result<(), DbWrapperError>;
    fn clear_batch(&self) -> Result<(), DbWrapperError>;
    fn execute_batch(&self) -> Result<Vec<i64>, DbWrapperError>;
    fn is_closed(&self) -> bool;
    fn set_poolable(&self, poolable: bool) -> Result<(), DbWrapperError>;
    fn is_poolable(&self) -> bool;

    // ─── PreparedStatement 参数绑定 ───
    fn set_null(&self, parameter_index: i32, sql_type: i32) -> Result<(), DbWrapperError>;
    fn set_boolean(&self, parameter_index: i32, x: bool) -> Result<(), DbWrapperError>;
    fn set_byte(&self, parameter_index: i32, x: i8) -> Result<(), DbWrapperError>;
    fn set_short(&self, parameter_index: i32, x: i16) -> Result<(), DbWrapperError>;
    fn set_int(&self, parameter_index: i32, x: i32) -> Result<(), DbWrapperError>;
    fn set_long(&self, parameter_index: i32, x: i64) -> Result<(), DbWrapperError>;
    fn set_float(&self, parameter_index: i32, x: f32) -> Result<(), DbWrapperError>;
    fn set_double(&self, parameter_index: i32, x: f64) -> Result<(), DbWrapperError>;
    fn set_string(&self, parameter_index: i32, x: &str) -> Result<(), DbWrapperError>;
    fn set_bytes(&self, parameter_index: i32, x: &[u8]) -> Result<(), DbWrapperError>;
    fn clear_parameters(&self) -> Result<(), DbWrapperError>;
    fn set_object(&self, parameter_index: i32, x: &dyn Any) -> Result<(), DbWrapperError>;
    fn execute_prepared(&self) -> Result<bool, DbWrapperError>;
    fn execute_query_prepared(&self) -> Result<Box<dyn Any>, DbWrapperError>;
    fn execute_update_prepared(&self) -> Result<i64, DbWrapperError>;
}

/// JDBC Connection 包装器 trait，对齐 `cn.hutool.db.ds.pooled.ConnectionWraper`。
///
/// Java 实现 `Connection`；Rust 用 trait 提供 API 形状。具体实现需要 SQLx Pool。
pub trait ConnectionWraper: Send + Sync {
    fn create_statement(&self) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    fn prepare_statement(&self, sql: &str) -> Result<Box<dyn StatementWrapper>, DbWrapperError>;
    fn set_auto_commit(&self, auto_commit: bool) -> Result<(), DbWrapperError>;
    fn get_auto_commit(&self) -> Result<bool, DbWrapperError>;
    fn commit(&self) -> Result<(), DbWrapperError>;
    fn rollback(&self) -> Result<(), DbWrapperError>;
    fn set_read_only(&self, read_only: bool) -> Result<(), DbWrapperError>;
    fn is_read_only(&self) -> Result<bool, DbWrapperError>;
    fn set_catalog(&self, catalog: &str) -> Result<(), DbWrapperError>;
    fn get_catalog(&self) -> Result<String, DbWrapperError>;
    fn set_transaction_isolation(&self, level: i32) -> Result<(), DbWrapperError>;
    fn get_transaction_isolation(&self) -> Result<i32, DbWrapperError>;
    fn clear_warnings(&self) -> Result<(), DbWrapperError>;
    fn is_valid(&self, timeout_seconds: i32) -> bool;
    fn close(&self) -> Result<(), DbWrapperError>;
}

/// 抽象 DataSource，对齐 `cn.hutool.db.ds.simple.AbstractDataSource`。
///
/// Java 实现 `javax.sql.DataSource`；Rust 用 trait 提供形状。
pub trait AbstractDataSource: Send + Sync {
    fn set_login_timeout(&self, seconds: i32) -> Result<(), DbWrapperError>;
    fn get_login_timeout(&self) -> Result<i32, DbWrapperError>;
    fn unwrap(&self) -> Option<Box<dyn Any>>;
    fn is_wrapper_for(&self) -> bool;
}

/// JDBC Wrapper 错误类型
#[derive(Debug, thiserror::Error)]
pub enum DbWrapperError {
    #[error("JDBC operation requires Java JDBC SPI; use SQLx in Rust instead")]
    JdbcSpiNotAvailable,
    #[error("JDBC operation failed: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 桩实现，验证 trait 可被实现
    struct StubStatement;
    impl StatementWrapper for StubStatement {
        fn execute_query_sql(&self, _sql: &str) -> Result<Box<dyn Any>, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_update_sql(&self, _sql: &str) -> Result<i64, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn close(&self) -> Result<(), DbWrapperError> {
            Ok(())
        }
        fn get_max_field_size(&self) -> Result<i32, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_max_field_size(&self, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn get_max_rows(&self) -> Result<i32, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_max_rows(&self, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_escape_processing(&self, _: bool) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn get_query_timeout(&self) -> Result<i32, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_query_timeout(&self, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn cancel(&self) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_cursor_name(&self, _: &str) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_sql(&self, _: &str) -> Result<bool, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn get_update_count(&self) -> Result<i64, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_fetch_direction(&self, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn get_fetch_direction(&self) -> Result<i32, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_fetch_size(&self, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn get_fetch_size(&self) -> Result<i32, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn add_batch(&self, _: &str) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn clear_batch(&self) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_batch(&self) -> Result<Vec<i64>, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn is_closed(&self) -> bool {
            false
        }
        fn set_poolable(&self, _: bool) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn is_poolable(&self) -> bool {
            false
        }
        fn set_null(&self, _: i32, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_boolean(&self, _: i32, _: bool) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_byte(&self, _: i32, _: i8) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_short(&self, _: i32, _: i16) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_int(&self, _: i32, _: i32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_long(&self, _: i32, _: i64) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_float(&self, _: i32, _: f32) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_double(&self, _: i32, _: f64) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_string(&self, _: i32, _: &str) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_bytes(&self, _: i32, _: &[u8]) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn clear_parameters(&self) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn set_object(&self, _: i32, _: &dyn Any) -> Result<(), DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_prepared(&self) -> Result<bool, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_query_prepared(&self) -> Result<Box<dyn Any>, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
        fn execute_update_prepared(&self) -> Result<i64, DbWrapperError> {
            Err(DbWrapperError::JdbcSpiNotAvailable)
        }
    }

    #[test]
    fn test_stub_statement_returns_jdbc_spi_error() {
        let s = StubStatement;
        let r = s.execute_query_sql("SELECT 1");
        assert!(matches!(r, Err(DbWrapperError::JdbcSpiNotAvailable)));
        assert!(s.close().is_ok());
        assert!(!s.is_closed());
    }

    #[test]
    fn test_stub_statement_set_get() {
        let s = StubStatement;
        let r = s.set_int(1, 42);
        assert!(r.is_err());
    }

    #[test]
    fn test_db_wrapper_error_display() {
        let e = DbWrapperError::JdbcSpiNotAvailable;
        assert!(e.to_string().contains("SQLx"));
    }
}