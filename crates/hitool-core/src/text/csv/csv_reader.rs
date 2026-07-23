//! 对齐: `cn.hutool.core.text.csv.CsvReader`

use super::csv_data::CsvData;
use super::csv_parser::CsvParser;
use super::csv_read_config::CsvReadConfig;
use super::csv_row::CsvRow;
use crate::Result;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

/// 对齐 Java: `CsvReader#`
#[derive(Debug, Clone)]
pub struct CsvReader {
    config: CsvReadConfig,
}

impl Default for CsvReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvReader {
    /// 对齐 Java: `CsvReader()`
    pub fn new() -> Self {
        Self {
            config: CsvReadConfig::default_config(),
        }
    }

    /// 对齐 Java: `CsvReader(CsvReadConfig)`
    pub fn with_config(config: CsvReadConfig) -> Self {
        Self { config }
    }

    /// 对齐 Java: `read(Reader)`
    pub fn read_str(&self, text: &str) -> Result<CsvData> {
        let mut parser = CsvParser::new(text, Some(self.config.clone()));
        let rows = parser.read_all()?;
        Ok(CsvData {
            header: parser.header_fields().map(|h| h.to_vec()),
            rows,
        })
    }

    /// 对齐 Java: `read(File)`
    pub fn read_file(&self, path: impl AsRef<Path>) -> Result<CsvData> {
        let text = fs::read_to_string(path)?;
        self.read_str(&text)
    }

    /// 对齐 Java: `readFromStr`
    pub fn read_from_str(&self, text: &str) -> Result<CsvData> {
        self.read_str(text)
    }

    /// 对齐 Java: `read` with row handler
    pub fn read_with_handler<F>(&self, text: &str, mut handler: F) -> Result<()>
    where
        F: FnMut(&CsvRow),
    {
        let mut parser = CsvParser::new(text, Some(self.config.clone()));
        while let Some(row) = parser.next_row()? {
            handler(&row);
        }
        Ok(())
    }

    /// 对齐 Java: `readMapList`
    pub fn read_map_list(&self, text: &str) -> Result<Vec<IndexMap<String, String>>> {
        let mut cfg = self.config.clone();
        if cfg.header_line_no < 0 {
            cfg.header_line_no = 0;
        }
        let mut parser = CsvParser::new(text, Some(cfg));
        let mut out = Vec::new();
        while let Some(row) = parser.next_row()? {
            out.push(row.to_map());
        }
        Ok(out)
    }

    /// 流式读取所有行
    pub fn stream_rows(&self, text: &str) -> Result<Vec<CsvRow>> {
        let mut parser = CsvParser::new(text, Some(self.config.clone()));
        parser.read_all()
    }
}
