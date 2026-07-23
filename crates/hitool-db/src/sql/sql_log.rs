//! SQL 日志 —— 对齐 Hutool `cn.hutool.db.sql.SqlLog`（显式注入，无全局单例）。

/// 对齐 Hutool `SqlLog`：控制是否打印 SQL / 参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlLog {
    show_sql: bool,
    format_sql: bool,
    show_params: bool,
}

impl SqlLog {
    /// 对齐 Java: `SqlLog.INSTANCE` 的本地等价 —— 每次显式构造。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 Java: `SqlLog.init(boolean, boolean, boolean)`。
    pub fn init(&mut self, show_sql: bool, format_sql: bool, show_params: bool) {
        self.show_sql = show_sql;
        self.format_sql = format_sql;
        self.show_params = show_params;
    }

    /// 对齐 Java: `SqlLog.log(String sql)` —— 返回格式化日志行（不写全局 logger）。
    #[must_use]
    pub fn log(&self, sql: &str) -> Option<String> {
        if !self.show_sql {
            return None;
        }
        let body = if self.format_sql {
            crate::sql::formatter::format(sql)
        } else {
            sql.to_string()
        };
        Some(format!("[SQL] {body}"))
    }

    /// 对齐 Java: `SqlLog.log(String sql, Object... params)`。
    #[must_use]
    pub fn log_with_params(&self, sql: &str, params: &[serde_json::Value]) -> Option<String> {
        let mut line = self.log(sql)?;
        if self.show_params && !params.is_empty() {
            line.push_str(" | params=");
            line.push_str(&format!("{params:?}"));
        }
        Some(line)
    }

    /// 对齐 Java: `SqlLog.logForBatch`。
    #[must_use]
    pub fn log_for_batch(&self, sql: &str, batch_size: usize) -> Option<String> {
        let mut line = self.log(sql)?;
        line.push_str(&format!(" | batchSize={batch_size}"));
        Some(line)
    }

    /// 是否展示 SQL。
    #[must_use]
    pub fn show_sql(self) -> bool {
        self.show_sql
    }
}

impl Default for SqlLog {
    fn default() -> Self {
        Self {
            show_sql: false,
            format_sql: false,
            show_params: false,
        }
    }
}
