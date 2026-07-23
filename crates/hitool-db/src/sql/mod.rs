//! SQL 子模块 —— 对齐 Hutool `cn.hutool.db.sql.*`。

pub mod condition;
pub mod condition_builder;
pub mod formatter;
pub mod logical_operator;
pub mod named_sql;
pub mod order;
pub mod query;
pub mod sql_builder;
pub mod sql_log;
pub mod sql_util;

pub use condition::{Condition, ConditionGroup, ConditionValue, LikeType};
pub use condition_builder::ConditionBuilder;
pub use formatter::format as format_sql;
pub use logical_operator::LogicalOperator;
pub use named_sql::NamedSql;
pub use order::{Direction, Order};
pub use query::Query;
pub use sql_builder::{Join, SqlBuilder};
pub use sql_log::SqlLog;
pub use sql_util::{build_conditions, build_like_value, is_in_clause, remove_outer_order_by};
