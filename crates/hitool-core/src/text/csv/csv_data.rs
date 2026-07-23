//! 对齐: `cn.hutool.core.text.csv.CsvData`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvData.java

use super::csv_row::CsvRow;

/// 对齐 Java: `CsvData#`
#[derive(Debug, Clone, Default)]
pub struct CsvData {
    /// 表头字段（可选）。
    pub header: Option<Vec<String>>,
    /// 数据行。
    pub rows: Vec<CsvRow>,
}

impl CsvData {
    /// 对齐 Java: `CsvData(List header, List rows)`
    pub fn new(header: Option<Vec<String>>, rows: Vec<CsvRow>) -> Self {
        Self { header, rows }
    }

    /// 对齐 Java: `getHeader`
    pub fn get_header(&self) -> Option<&[String]> {
        self.header.as_deref()
    }

    /// 对齐 Java: `getRows`
    pub fn get_rows(&self) -> &[CsvRow] {
        &self.rows
    }

    /// 对齐 Java: `getRow`
    pub fn get_row(&self, index: usize) -> Option<&CsvRow> {
        self.rows.get(index)
    }

    /// 对齐 Java: `getRowCount`
    pub fn get_row_count(&self) -> usize {
        self.rows.len()
    }

    /// 对齐 Java: `iterator`
    pub fn iter(&self) -> impl Iterator<Item = &CsvRow> {
        self.rows.iter()
    }

    /// 对齐 Java: `toString`
    pub fn to_string_repr(&self) -> String {
        format!(
            "CsvData{{header={:?}, rowCount={}}}",
            self.header,
            self.rows.len()
        )
    }
}

impl std::fmt::Display for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

impl IntoIterator for CsvData {
    type Item = CsvRow;
    type IntoIter = std::vec::IntoIter<CsvRow>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}
