//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

/// 对齐 Hutool `RsHandler`：将行集合折叠为自定义结果。
pub trait RsHandler<T> {
    /// 对齐 Java: `handle(ResultSet)`。
    fn handle(&self, rows: &[Entity]) -> T;
}

/// 对齐 Hutool `HandleHelper`：行/列 → Entity 转换辅助。
pub struct HandleHelper;

impl HandleHelper {
    /// 对齐 Java: `handleRow` —— 由列名与值构造 Entity。
    #[must_use]
    pub fn handle_row(columns: impl IntoIterator<Item = (impl Into<String>, Value)>) -> Entity {
        let mut entity = Entity::create();
        for (k, v) in columns {
            entity.set_value(k, v);
        }
        entity
    }

    /// 对齐 Java: `handleRowToList` —— 单行按列序输出值列表。
    #[must_use]
    pub fn handle_row_to_list(entity: &Entity) -> Vec<Value> {
        entity.iter().map(|(_, v)| v.clone()).collect()
    }

    /// 对齐 Java: `handleRs` —— 多行 Entity 列表（已是 Entity 时恒等）。
    #[must_use]
    pub fn handle_rs(rows: Vec<Entity>) -> Vec<Entity> {
        rows
    }

    /// 对齐 Java: `handleRsToBeanList`。
    pub fn handle_rs_to_bean_list<T: DeserializeOwned>(
        rows: &[Entity],
    ) -> Result<Vec<T>, serde_json::Error> {
        rows.iter().map(Entity::to_bean_ignore_case).collect()
    }
}

/// 对齐 Hutool `EntityHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityHandler {
    case_insensitive: bool,
}

impl EntityHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java: `EntityHandler(boolean caseInsensitive)`。
    #[must_use]
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }

    /// 对齐 Java: `handle` —— 取首行。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<Entity> {
        let _ = self.case_insensitive;
        rows.first().cloned()
    }
}

impl RsHandler<Option<Entity>> for EntityHandler {
    fn handle(&self, rows: &[Entity]) -> Option<Entity> {
        self.handle(rows)
    }
}

/// 对齐 Hutool `EntityListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityListHandler {
    case_insensitive: bool,
}

impl EntityListHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        let _ = self.case_insensitive;
        rows.to_vec()
    }
}

impl RsHandler<Vec<Entity>> for EntityListHandler {
    fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        self.handle(rows)
    }
}

/// 对齐 Hutool `EntitySetHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct EntitySetHandler {
    case_insensitive: bool,
}

impl EntitySetHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(case_insensitive: bool) -> Self {
        Self { case_insensitive }
    }

    /// 对齐 Java: `handle` —— 按字段签名去重。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Entity> {
        let _ = self.case_insensitive;
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for row in rows {
            let key = row
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            if seen.insert(key) {
                out.push(row.clone());
            }
        }
        out
    }
}

/// 对齐 Hutool `PageResultHandler`。
#[derive(Debug, Clone)]
pub struct PageResultHandler {
    page: u32,
    page_size: u32,
    total: u64,
}

impl PageResultHandler {
    /// 对齐 Java: `create(PageResult)` 的本地等价。
    #[must_use]
    pub fn create(page: u32, page_size: u32, total: u64) -> Self {
        Self {
            page,
            page_size,
            total,
        }
    }

    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(page: u32, page_size: u32, total: u64) -> Self {
        Self::create(page, page_size, total)
    }

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: Vec<Entity>) -> PageResult {
        PageResult::new(self.page, self.page_size, self.total, rows)
    }
}

/// 对齐 Hutool `BeanHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct BeanHandler;

impl BeanHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 首行转 bean。
    pub fn handle<T: DeserializeOwned>(&self, rows: &[Entity]) -> Result<Option<T>, serde_json::Error> {
        rows.first()
            .map(Entity::to_bean_ignore_case)
            .transpose()
    }
}

/// 对齐 Hutool `BeanListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct BeanListHandler;

impl BeanListHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle`。
    pub fn handle<T: DeserializeOwned>(&self, rows: &[Entity]) -> Result<Vec<T>, serde_json::Error> {
        HandleHelper::handle_rs_to_bean_list(rows)
    }
}

/// 对齐 Hutool `NumberHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberHandler;

impl NumberHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 取首行首列数值。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<i64> {
        rows.first()
            .and_then(|e| e.iter().next())
            .and_then(|(_, v)| match v {
                Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
                Value::String(s) => s.parse().ok(),
                Value::Bool(b) => Some(i64::from(*b)),
                _ => None,
            })
    }
}

/// 对齐 Hutool `StringHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct StringHandler;

impl StringHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle`。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Option<String> {
        rows.first()
            .and_then(|e| e.iter().next())
            .map(|(_, v)| match v {
                Value::String(s) => s.clone(),
                Value::Null => String::new(),
                other => other.to_string().trim_matches('"').to_string(),
            })
    }
}

/// 对齐 Hutool `ValueListHandler`。
#[derive(Debug, Clone, Copy, Default)]
pub struct ValueListHandler;

impl ValueListHandler {
    /// 对齐 Java: `create()`。
    #[must_use]
    pub fn create() -> Self {
        Self
    }

    /// 对齐 Java: `handle` —— 每行按列序取值。
    #[must_use]
    pub fn handle(&self, rows: &[Entity]) -> Vec<Vec<Value>> {
        rows.iter().map(HandleHelper::handle_row_to_list).collect()
    }
}
