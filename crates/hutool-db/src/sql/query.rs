//! 查询对象 —— 对齐 Hutool `cn.hutool.db.sql.Query`。

use crate::entity::Entity;
use crate::hutool_page::HutoolPage;
use crate::sql::condition::Condition;
use crate::sql::sql_util::build_conditions;

/// 查询描述对象：表名、字段、条件与分页。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Query {
    fields: Vec<String>,
    table_names: Vec<String>,
    where_conditions: Vec<Condition>,
    page: Option<HutoolPage>,
}

impl Query {
    /// 对齐 Java: `Query.of(Entity)`。
    #[must_use]
    pub fn of(where_entity: &Entity) -> Self {
        let mut query = Self {
            fields: where_entity.field_names().to_vec(),
            table_names: where_entity
                .table_name()
                .map(|t| vec![t.to_string()])
                .unwrap_or_default(),
            where_conditions: build_conditions(where_entity),
            page: None,
        };
        if query.fields.is_empty() {
            query.fields = where_entity.iter().map(|(k, _)| k.clone()).collect();
        }
        query
    }

    /// 对齐 Java: `Query(String... tableNames)`。
    #[must_use]
    pub fn from_tables(table_names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            table_names: table_names.into_iter().map(|t| t.into()).collect(),
            ..Self::default()
        }
    }

    /// 对齐 Java: `Query(Condition[], String...)`。
    #[must_use]
    pub fn with_where(
        where_conditions: Vec<Condition>,
        table_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            where_conditions,
            table_names: table_names.into_iter().map(|t| t.into()).collect(),
            ..Self::default()
        }
    }

    /// 对齐 Java: `getFields()`。
    #[must_use]
    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    /// 对齐 Java: `setFields(Collection)` / `setFields(String...)`。
    pub fn set_fields(&mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.fields = fields.into_iter().map(|f| f.into()).collect();
        self
    }

    /// 对齐 Java: `getTableNames()`。
    #[must_use]
    pub fn table_names(&self) -> &[String] {
        &self.table_names
    }

    /// 对齐 Java: `setTableNames(String...)`。
    pub fn set_table_names(
        &mut self,
        table_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.table_names = table_names.into_iter().map(|t| t.into()).collect();
        self
    }

    /// 对齐 Java: `getWhere()`。
    #[must_use]
    pub fn where_conditions(&self) -> &[Condition] {
        &self.where_conditions
    }

    /// 对齐 Java: `setWhere(Condition...)`。
    pub fn set_where(&mut self, where_conditions: Vec<Condition>) -> &mut Self {
        self.where_conditions = where_conditions;
        self
    }

    /// 对齐 Java: `getPage()`。
    #[must_use]
    pub fn page(&self) -> Option<&HutoolPage> {
        self.page.as_ref()
    }

    /// 对齐 Java: `setPage(Page)`。
    pub fn set_page(&mut self, page: HutoolPage) -> &mut Self {
        self.page = Some(page);
        self
    }

    /// 对齐 Java: `getFirstTableName()`。
    #[must_use]
    pub fn first_table_name(&self) -> Option<&str> {
        self.table_names.first().map(String::as_str)
    }
}
