//! NoSQL 配置桩 —— 对齐 Hutool `cn.hutool.db.nosql.*` 测试期望。

/// MongoDB 数据源配置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MongoDs {
    database: String,
}

impl MongoDs {
    /// 构造 Mongo 数据源。
    #[must_use]
    pub fn new(database: impl Into<String>) -> Self {
        Self {
            database: database.into(),
        }
    }

    /// 对齐 Java: `MongoDatabase.getName()`.
    #[must_use]
    pub fn db_name(&self) -> &str {
        &self.database
    }
}

/// Redis 数据源配置。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisDs {
    host: String,
}

impl RedisDs {
    /// 对齐 Java: `RedisDS.create()`.
    #[must_use]
    pub fn create() -> Self {
        Self {
            host: "localhost".to_string(),
        }
    }

    /// 返回主机名（桩实现）。
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }
}
