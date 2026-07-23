//! 对齐: `cn.hutool.core.text.csv.CsvBaseReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvBaseReader.java
//!
//! 配置持有 + 读取门面；具体解析委托 [`super::csv_reader::CsvReader`]。

use indexmap::IndexMap;

use crate::Result;

use super::csv_data::CsvData;
use super::csv_read_config::CsvReadConfig;
use super::csv_reader::CsvReader;
use super::csv_row::CsvRow;

/// 对齐 Java: `CsvBaseReader#`
#[derive(Debug, Clone)]
pub struct CsvBaseReader {
    config: CsvReadConfig,
}

impl Default for CsvBaseReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvBaseReader {
    /// 对齐 Java: `CsvBaseReader()`
    pub fn new() -> Self {
        Self {
            config: CsvReadConfig::default_config(),
        }
    }

    /// 对齐 Java: `CsvBaseReader(CsvReadConfig)`
    pub fn with_config(config: CsvReadConfig) -> Self {
        Self { config }
    }

    /// 对齐 Java: `setFieldSeparator`
    pub fn set_field_separator(&mut self, c: char) -> &mut Self {
        self.config.set_field_separator(c);
        self
    }

    /// 对齐 Java: `setTextDelimiter`
    pub fn set_text_delimiter(&mut self, c: char) -> &mut Self {
        self.config.set_text_delimiter(c);
        self
    }

    /// 对齐 Java: `setContainsHeader`
    pub fn set_contains_header(&mut self, v: bool) -> &mut Self {
        self.config.set_contains_header(v);
        self
    }

    /// 对齐 Java: `setSkipEmptyRows`
    pub fn set_skip_empty_rows(&mut self, v: bool) -> &mut Self {
        self.config.skip_empty_rows = v;
        self
    }

    /// 对齐 Java: `setErrorOnDifferentFieldCount`
    pub fn set_error_on_different_field_count(&mut self, v: bool) -> &mut Self {
        self.config.error_on_different_field_count = v;
        self
    }

    /// 内部：构造绑定当前配置的 reader。
    fn reader(&self) -> CsvReader {
        CsvReader::with_config(self.config.clone())
    }

    /// 对齐 Java: `read(Reader)` / 字符串入口
    pub fn read(&self, text: &str) -> Result<CsvData> {
        self.reader().read_from_str(text)
    }

    /// 对齐 Java: `readFromStr`
    pub fn read_from_str(&self, text: &str) -> Result<CsvData> {
        self.read(text)
    }

    /// 对齐 Java: `read(File)` 风格 — 读路径
    pub fn read_file(&self, path: impl AsRef<std::path::Path>) -> Result<CsvData> {
        self.reader().read_file(path)
    }

    /// 对齐 Java: `readMapList`
    pub fn read_map_list(&self, text: &str) -> Result<Vec<IndexMap<String, String>>> {
        self.reader().read_map_list(text)
    }

    /// 对齐 Java: `read` + handler
    pub fn read_with_handler<F>(&self, text: &str, handler: F) -> Result<()>
    where
        F: FnMut(&CsvRow),
    {
        self.reader().read_with_handler(text, handler)
    }
}
