//! 命名参数 SQL —— 对齐 Hutool `cn.hutool.db.sql.NamedSql`。

use crate::sql::sql_util::is_in_clause;
use indexmap::IndexMap;
use serde_json::Value;
use std::collections::HashMap;

const NAME_START_CHARS: [char; 3] = [':', '@', '?'];

/// 命名占位符 SQL 解析结果。
#[derive(Debug, Clone, PartialEq)]
pub struct NamedSql {
    sql: String,
    params: Vec<Value>,
}

impl NamedSql {
    /// 对齐 Java: `NamedSql(String namedSql, Map<String, Object> paramMap)`.
    #[must_use]
    pub fn new(named_sql: impl Into<String>, param_map: &HashMap<String, Value>) -> Self {
        let mut params = Vec::new();
        let sql = parse_named_sql(named_sql.into(), param_map, &mut params);
        Self { sql, params }
    }

    /// 对齐 Java: `NamedSql.getSql()`.
    #[must_use]
    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// 对齐 Java: `NamedSql.getParams()`.
    #[must_use]
    pub fn params(&self) -> &[Value] {
        &self.params
    }
}

fn parse_named_sql(
    named_sql: String,
    param_map: &HashMap<String, Value>,
    params: &mut Vec<Value>,
) -> String {
    if param_map.is_empty() {
        return named_sql;
    }

    let mut sql = String::new();
    let mut name = String::new();
    let mut name_start: Option<char> = None;
    let chars: Vec<char> = named_sql.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];
        if NAME_START_CHARS.contains(&c) {
            replace_var(name_start, &mut name, &mut sql, param_map, params);
            name_start = Some(c);
        } else if name_start.is_some() {
            if is_generate_char(c) {
                name.push(c);
            } else {
                replace_var(name_start, &mut name, &mut sql, param_map, params);
                name_start = None;
                sql.push(c);
            }
        } else {
            sql.push(c);
        }
        i += 1;
    }

    if !name.is_empty() {
        replace_var(name_start, &mut name, &mut sql, param_map, params);
    }
    sql
}

fn replace_var(
    name_start: Option<char>,
    name: &mut String,
    sql: &mut String,
    param_map: &HashMap<String, Value>,
    params: &mut Vec<Value>,
) {
    if name.is_empty() {
        if let Some(start) = name_start {
            sql.push(start);
        }
        return;
    }

    let name_str = name.clone();
    if let Some(param_value) = param_map.get(&name_str) {
        if param_value.is_array() && is_in_clause(sql) {
            if let Some(array) = param_value.as_array() {
                for (idx, item) in array.iter().enumerate() {
                    if idx > 0 {
                        sql.push(',');
                    }
                    sql.push('?');
                    params.push(item.clone());
                }
            }
        } else {
            sql.push('?');
            params.push(param_value.clone());
        }
    } else if let Some(start) = name_start {
        sql.push(start);
        sql.push_str(&name_str);
    }
    name.clear();
}

fn is_generate_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// 便捷构造：IndexMap 参数。
#[must_use]
pub fn from_map(named_sql: impl Into<String>, param_map: &IndexMap<String, Value>) -> NamedSql {
    let map = param_map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    NamedSql::new(named_sql, &map)
}
