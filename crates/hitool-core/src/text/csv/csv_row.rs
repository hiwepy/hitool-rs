//! 对齐: `cn.hutool.core.text.csv.CsvRow`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvRow.java
//!
//! 实现 List 语义的常用读写方法（反射 `toBean` 标为 planned）。

use indexmap::IndexMap;

/// 对齐 Java: `CsvRow#`
#[derive(Debug, Clone)]
pub struct CsvRow {
    /// 原始行号（1-based，对齐 Hutool）。
    pub original_line_number: i64,
    /// 表头名 → 列下标。
    pub header_map: Option<IndexMap<String, usize>>,
    /// 字段值。
    pub fields: Vec<String>,
}

impl CsvRow {
    /// 对齐 Java: `CsvRow(long, Map, List)`
    pub fn new(line_no: i64, header: Option<IndexMap<String, usize>>, fields: Vec<String>) -> Self {
        Self {
            original_line_number: line_no,
            header_map: header,
            fields,
        }
    }

    /// 对齐 Java: `get(int)`
    pub fn get(&self, index: usize) -> Option<&str> {
        self.fields.get(index).map(|s| s.as_str())
    }

    /// 对齐 Java: `getByName`
    pub fn get_by_name(&self, name: &str) -> Option<&str> {
        let idx = self.header_map.as_ref()?.get(name)?;
        self.get(*idx)
    }

    /// 对齐 Java: `getRawList`
    pub fn get_raw_list(&self) -> &[String] {
        &self.fields
    }

    /// 对齐 Java: `getFieldMap`
    pub fn get_field_map(&self) -> IndexMap<String, String> {
        self.to_map()
    }

    /// 对齐 Java: `getFieldCount` / `size`
    pub fn get_field_count(&self) -> usize {
        self.fields.len()
    }

    /// 对齐 Java: `size`
    pub fn size(&self) -> usize {
        self.fields.len()
    }

    /// 对齐 Java: `isEmpty`
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// 对齐 Java: `getOriginalLineNumber`
    pub fn get_original_line_number(&self) -> i64 {
        self.original_line_number
    }

    /// 对齐 Java: `contains`
    pub fn contains(&self, value: &str) -> bool {
        self.fields.iter().any(|f| f == value)
    }

    /// 对齐 Java: `containsAll`
    pub fn contains_all(&self, values: &[&str]) -> bool {
        values.iter().all(|v| self.contains(v))
    }

    /// 对齐 Java: `indexOf`
    pub fn index_of(&self, value: &str) -> i32 {
        self.fields
            .iter()
            .position(|f| f == value)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }

    /// 对齐 Java: `lastIndexOf`
    pub fn last_index_of(&self, value: &str) -> i32 {
        self.fields
            .iter()
            .rposition(|f| f == value)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }

    /// 对齐 Java: `add`
    pub fn add(&mut self, value: String) -> bool {
        self.fields.push(value);
        true
    }

    /// 对齐 Java: `addAll`
    pub fn add_all(&mut self, values: &[String]) -> bool {
        self.fields.extend(values.iter().cloned());
        true
    }

    /// 对齐 Java: `set`
    pub fn set(&mut self, index: usize, value: String) -> Option<String> {
        if index < self.fields.len() {
            Some(std::mem::replace(&mut self.fields[index], value))
        } else {
            None
        }
    }

    /// 对齐 Java: `remove(int)`
    pub fn remove_at(&mut self, index: usize) -> Option<String> {
        if index < self.fields.len() {
            Some(self.fields.remove(index))
        } else {
            None
        }
    }

    /// 对齐 Java: `remove(Object)`
    pub fn remove_value(&mut self, value: &str) -> bool {
        if let Some(i) = self.fields.iter().position(|f| f == value) {
            self.fields.remove(i);
            true
        } else {
            false
        }
    }

    /// 对齐 Java: `removeAll`
    pub fn remove_all(&mut self, values: &[&str]) -> bool {
        let before = self.fields.len();
        self.fields.retain(|f| !values.contains(&f.as_str()));
        before != self.fields.len()
    }

    /// 对齐 Java: `retainAll`
    pub fn retain_all(&mut self, values: &[&str]) -> bool {
        let before = self.fields.len();
        self.fields.retain(|f| values.contains(&f.as_str()));
        before != self.fields.len()
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.fields.clear();
    }

    /// 对齐 Java: `subList`
    pub fn sub_list(&self, from: usize, to: usize) -> Vec<String> {
        let end = to.min(self.fields.len());
        let start = from.min(end);
        self.fields[start..end].to_vec()
    }

    /// 对齐 Java: `toArray`
    pub fn to_array(&self) -> Vec<String> {
        self.fields.clone()
    }

    /// 对齐 Java: `iterator`
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.fields.iter()
    }

    /// 转为 Map（按 header）
    pub fn to_map(&self) -> IndexMap<String, String> {
        let mut m = IndexMap::new();
        if let Some(ref header) = self.header_map {
            for (k, &i) in header {
                if let Some(v) = self.fields.get(i) {
                    m.insert(k.clone(), v.clone());
                }
            }
        } else {
            for (i, v) in self.fields.iter().enumerate() {
                m.insert(i.to_string(), v.clone());
            }
        }
        m
    }

    /// 对齐 Java: `toString`
    pub fn to_string_repr(&self) -> String {
        self.fields.join(",")
    }
}

impl std::fmt::Display for CsvRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}
