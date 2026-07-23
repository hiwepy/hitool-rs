//! GlobalDbConfig facade，对齐 hutool 的 `cn.hutool.db.GlobalDbConfig`。
//!
//! 全局数据库配置（大小写敏感、是否返回生成键、是否显示 SQL 等）。

use std::sync::Mutex;

/// 日志级别，对齐 `cn.hutool.db.nosql.NoSQLException` 等使用的 Level（简化版）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// 全局数据库配置，对齐 `cn.hutool.db.GlobalDbConfig`。
pub struct GlobalDbConfig;

static CASE_INSENSITIVE: Mutex<bool> = Mutex::new(false);
static RETURN_GENERATED_KEY: Mutex<bool> = Mutex::new(true);
static DB_SETTING_PATH: Mutex<Option<String>> = Mutex::new(None);
static SHOW_SQL: Mutex<bool> = Mutex::new(false);
static FORMAT_SQL: Mutex<bool> = Mutex::new(false);
static SHOW_PARAMS: Mutex<bool> = Mutex::new(false);
static SQL_LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Debug);
static STATEMENT_FETCH_SIZE: Mutex<Option<i32>> = Mutex::new(None);

impl GlobalDbConfig {
    /// 对齐 `GlobalDbConfig.setCaseInsensitive(boolean)`
    pub fn set_case_insensitive(is_case_insensitive: bool) {
        if let Ok(mut g) = CASE_INSENSITIVE.lock() {
            *g = is_case_insensitive;
        }
    }

    /// 读取大小写敏感设置
    pub fn is_case_insensitive() -> bool {
        CASE_INSENSITIVE.lock().map(|g| *g).unwrap_or(false)
    }

    /// 对齐 `GlobalDbConfig.setReturnGeneratedKey(boolean)`
    pub fn set_return_generated_key(is_return: bool) {
        if let Ok(mut g) = RETURN_GENERATED_KEY.lock() {
            *g = is_return;
        }
    }

    /// 读取 return_generated_key 设置
    pub fn is_return_generated_key() -> bool {
        RETURN_GENERATED_KEY.lock().map(|g| *g).unwrap_or(true)
    }

    /// 对齐 `GlobalDbConfig.setDbSettingPath(String)`
    pub fn set_db_setting_path(path: Option<String>) {
        if let Ok(mut g) = DB_SETTING_PATH.lock() {
            *g = path;
        }
    }

    /// 读取 db_setting_path
    pub fn db_setting_path() -> Option<String> {
        DB_SETTING_PATH.lock().ok()?.clone()
    }

    /// 对齐 `GlobalDbConfig.createDbSetting()`：返回 Setting 路径
    ///
    /// Rust 版返回路径字符串（Java 版返回 `Setting` 对象）。
    pub fn create_db_setting() -> Option<String> {
        Self::db_setting_path().or_else(|| Some("config/db.setting".into()))
    }

    /// 对齐 `GlobalDbConfig.setShowSql(boolean, boolean, boolean, Level)`
    pub fn set_show_sql(is_show_sql: bool, is_format_sql: bool, is_show_params: bool, level: LogLevel) {
        if let Ok(mut g) = SHOW_SQL.lock() { *g = is_show_sql; }
        if let Ok(mut g) = FORMAT_SQL.lock() { *g = is_format_sql; }
        if let Ok(mut g) = SHOW_PARAMS.lock() { *g = is_show_params; }
        if let Ok(mut g) = SQL_LOG_LEVEL.lock() { *g = level; }
    }

    /// 读取 show_sql
    pub fn is_show_sql() -> bool {
        SHOW_SQL.lock().map(|g| *g).unwrap_or(false)
    }

    /// 读取 format_sql
    pub fn is_format_sql() -> bool {
        FORMAT_SQL.lock().map(|g| *g).unwrap_or(false)
    }

    /// 读取 show_params
    pub fn is_show_params() -> bool {
        SHOW_PARAMS.lock().map(|g| *g).unwrap_or(false)
    }

    /// 读取 sql_log_level
    pub fn sql_log_level() -> LogLevel {
        SQL_LOG_LEVEL.lock().map(|g| *g).unwrap_or(LogLevel::Debug)
    }

    /// 对齐 `GlobalDbConfig.setStatementFetchSize(Integer)`
    pub fn set_statement_fetch_size(size: Option<i32>) {
        if let Ok(mut g) = STATEMENT_FETCH_SIZE.lock() {
            *g = size;
        }
    }

    /// 读取 statement_fetch_size
    pub fn statement_fetch_size() -> Option<i32> {
        STATEMENT_FETCH_SIZE.lock().ok()?.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_default() {
        // 注意：全局状态，先重置
        GlobalDbConfig::set_case_insensitive(false);
        assert!(!GlobalDbConfig::is_case_insensitive());
        GlobalDbConfig::set_case_insensitive(true);
        assert!(GlobalDbConfig::is_case_insensitive());
        GlobalDbConfig::set_case_insensitive(false);
    }

    #[test]
    fn test_return_generated_key() {
        GlobalDbConfig::set_return_generated_key(false);
        assert!(!GlobalDbConfig::is_return_generated_key());
        GlobalDbConfig::set_return_generated_key(true);
        assert!(GlobalDbConfig::is_return_generated_key());
    }

    #[test]
    fn test_db_setting_path() {
        GlobalDbConfig::set_db_setting_path(Some("/tmp/test.setting".into()));
        assert_eq!(GlobalDbConfig::db_setting_path(), Some("/tmp/test.setting".into()));
        assert_eq!(
            GlobalDbConfig::create_db_setting(),
            Some("/tmp/test.setting".into())
        );
        GlobalDbConfig::set_db_setting_path(None);
        assert_eq!(
            GlobalDbConfig::create_db_setting(),
            Some("config/db.setting".into())
        );
    }

    #[test]
    fn test_show_sql() {
        GlobalDbConfig::set_show_sql(true, true, false, LogLevel::Info);
        assert!(GlobalDbConfig::is_show_sql());
        assert!(GlobalDbConfig::is_format_sql());
        assert!(!GlobalDbConfig::is_show_params());
        assert_eq!(GlobalDbConfig::sql_log_level(), LogLevel::Info);
        GlobalDbConfig::set_show_sql(false, false, false, LogLevel::Debug);
    }

    #[test]
    fn test_statement_fetch_size() {
        GlobalDbConfig::set_statement_fetch_size(Some(100));
        assert_eq!(GlobalDbConfig::statement_fetch_size(), Some(100));
        GlobalDbConfig::set_statement_fetch_size(None);
        assert_eq!(GlobalDbConfig::statement_fetch_size(), None);
    }
}