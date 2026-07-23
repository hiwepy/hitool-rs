//! ORDER BY —— 对齐 Hutool `cn.hutool.db.sql.Order`。

/// 排序方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Asc,
    Desc,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
        }
    }
}

/// 排序字段。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    field: String,
    direction: Option<Direction>,
}

impl Order {
    /// 对齐 Java: `new Order(String field)`.
    #[must_use]
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: None,
        }
    }

    /// 返回字段名。
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// 返回排序方向。
    #[must_use]
    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }
}
