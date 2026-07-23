//! 逻辑运算符 —— 对齐 Hutool `cn.hutool.db.sql.LogicalOperator`。

/// 条件之间的逻辑连接符。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogicalOperator {
    /// AND
    #[default]
    And,
    /// OR
    Or,
}

impl LogicalOperator {
    /// 对齐 Java: `LogicalOperator.toString()` 在 SQL 构建中的输出。
    #[must_use]
    pub fn as_sql(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }

    /// 对齐 Java: `LogicalOperator.isSame(String)`.
    #[must_use]
    pub fn is_same(&self, value: &str) -> bool {
        value.trim().eq_ignore_ascii_case(self.as_sql())
    }
}
