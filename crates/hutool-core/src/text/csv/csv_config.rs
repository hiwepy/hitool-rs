//! 对齐: `cn.hutool.core.text.csv.CsvConfig`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvConfig.java

use indexmap::IndexMap;

/// 对齐 Java: `CsvConfig#`
#[derive(Debug, Clone)]
pub struct CsvConfig {
    /// 字段分隔符
    pub field_separator: char,
    /// 文本包装符
    pub text_delimiter: char,
    /// 注释符
    pub comment_character: Option<char>,
    /// 表头别名
    pub header_alias: IndexMap<String, String>,
}

impl Default for CsvConfig {
    /// 对齐 Java: 默认 `,` / `"` / `#`。
    fn default() -> Self {
        Self {
            field_separator: ',',
            text_delimiter: '"',
            comment_character: Some('#'),
            header_alias: IndexMap::new(),
        }
    }
}

impl CsvConfig {
    /// 对齐 Java: `setFieldSeparator`
    pub fn set_field_separator(&mut self, c: char) -> &mut Self {
        self.field_separator = c;
        self
    }

    /// 对齐 Java: `setTextDelimiter`
    pub fn set_text_delimiter(&mut self, c: char) -> &mut Self {
        self.text_delimiter = c;
        self
    }

    /// 对齐 Java: `setCommentCharacter`
    pub fn set_comment_character(&mut self, c: Option<char>) -> &mut Self {
        self.comment_character = c;
        self
    }

    /// 对齐 Java: `disableComment`
    pub fn disable_comment(&mut self) -> &mut Self {
        self.comment_character = None;
        self
    }

    /// 对齐 Java: `setHeaderAlias`
    pub fn set_header_alias(&mut self, alias: IndexMap<String, String>) -> &mut Self {
        self.header_alias = alias;
        self
    }

    /// 对齐 Java: `addHeaderAlias`
    pub fn add_header_alias(&mut self, header: &str, alias: &str) -> &mut Self {
        self.header_alias
            .insert(header.to_string(), alias.to_string());
        self
    }

    /// 对齐 Java: `removeHeaderAlias`
    pub fn remove_header_alias(&mut self, header: &str) -> &mut Self {
        self.header_alias.shift_remove(header);
        self
    }
}
