//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

/// 对齐 Hutool `TableType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    /// 普通表。
    Table,
    /// 视图。
    View,
    /// 其他。
    Other,
}

impl TableType {
    /// 从 JDBC TABLE_TYPE 字符串解析。
    #[must_use]
    pub fn from_jdbc(name: &str) -> Self {
        match name.to_ascii_uppercase().as_str() {
            "TABLE" => Self::Table,
            "VIEW" => Self::View,
            _ => Self::Other,
        }
    }

    /// JDBC 名称。
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Table => "TABLE",
            Self::View => "VIEW",
            Self::Other => "OTHER",
        }
    }
}

/// 对齐 Hutool `JdbcType` 的常用子集。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JdbcType {
    /// VARCHAR。
    Varchar,
    /// INTEGER。
    Integer,
    /// BIGINT。
    BigInt,
    /// DECIMAL。
    Decimal,
    /// TIMESTAMP。
    Timestamp,
    /// BOOLEAN。
    Boolean,
    /// OTHER / 未知。
    Other,
}

impl JdbcType {
    /// 从类型名解析。
    #[must_use]
    pub fn from_type_name(name: &str) -> Self {
        match name.to_ascii_uppercase().as_str() {
            "VARCHAR" | "TEXT" | "CHAR" | "NVARCHAR" => Self::Varchar,
            "INT" | "INTEGER" | "SMALLINT" | "TINYINT" => Self::Integer,
            "BIGINT" => Self::BigInt,
            "DECIMAL" | "NUMERIC" | "REAL" | "FLOAT" | "DOUBLE" => Self::Decimal,
            "TIMESTAMP" | "DATETIME" | "DATE" | "TIME" => Self::Timestamp,
            "BOOLEAN" | "BOOL" | "BIT" => Self::Boolean,
            _ => Self::Other,
        }
    }
}

/// 对齐 Hutool `Column`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    table_name: String,
    name: String,
    type_code: i32,
    type_name: String,
    size: i32,
    digit: i32,
    nullable: bool,
    comment: String,
    auto_increment: bool,
    pk: bool,
    column_def: String,
}

impl Column {
    /// 对齐 Java: `Column.create(String tableName, String columnName)`。
    #[must_use]
    pub fn create(table_name: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            name: name.into(),
            type_code: 0,
            type_name: String::new(),
            size: 0,
            digit: 0,
            nullable: true,
            comment: String::new(),
            auto_increment: false,
            pk: false,
            column_def: String::new(),
        }
    }

    /// 对齐 Java: `init` 风格批量设置。
    pub fn init(
        &mut self,
        type_name: impl Into<String>,
        size: i32,
        nullable: bool,
        pk: bool,
    ) -> &mut Self {
        self.type_name = type_name.into();
        self.type_code = match JdbcType::from_type_name(&self.type_name) {
            JdbcType::Varchar => 12,
            JdbcType::Integer => 4,
            JdbcType::BigInt => -5,
            JdbcType::Decimal => 3,
            JdbcType::Timestamp => 93,
            JdbcType::Boolean => 16,
            JdbcType::Other => 1111,
        };
        self.size = size;
        self.nullable = nullable;
        self.pk = pk;
        self
    }

    /// 对齐 Java: `getTableName`。
    #[must_use]
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// 设置表名。
    pub fn set_table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
        self.table_name = table_name.into();
        self
    }

    /// 对齐 Java: `getName`。
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 设置列名。
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// 对齐 Java: `getTypeEnum`。
    #[must_use]
    pub fn type_enum(&self) -> JdbcType {
        JdbcType::from_type_name(&self.type_name)
    }

    /// 对齐 Java: `getType`。
    #[must_use]
    pub fn type_code(&self) -> i32 {
        self.type_code
    }

    /// 设置 JDBC 类型码。
    pub fn set_type(&mut self, type_code: i32) -> &mut Self {
        self.type_code = type_code;
        self
    }

    /// 对齐 Java: `getTypeName`。
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// 设置类型名。
    pub fn set_type_name(&mut self, type_name: impl Into<String>) -> &mut Self {
        self.type_name = type_name.into();
        self
    }

    /// 对齐 Java: `getSize`。
    #[must_use]
    pub fn size(&self) -> i32 {
        self.size
    }

    /// 设置大小。
    pub fn set_size(&mut self, size: i32) -> &mut Self {
        self.size = size;
        self
    }

    /// 对齐 Java: `getDigit`。
    #[must_use]
    pub fn digit(&self) -> i32 {
        self.digit
    }

    /// 设置小数位。
    pub fn set_digit(&mut self, digit: i32) -> &mut Self {
        self.digit = digit;
        self
    }

    /// 对齐 Java: `isNullable`。
    #[must_use]
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    /// 设置可空。
    pub fn set_nullable(&mut self, nullable: bool) -> &mut Self {
        self.nullable = nullable;
        self
    }

    /// 对齐 Java: `getComment`。
    #[must_use]
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// 设置注释。
    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.comment = comment.into();
        self
    }

    /// 对齐 Java: `isAutoIncrement`。
    #[must_use]
    pub fn is_auto_increment(&self) -> bool {
        self.auto_increment
    }

    /// 设置自增。
    pub fn set_auto_increment(&mut self, auto_increment: bool) -> &mut Self {
        self.auto_increment = auto_increment;
        self
    }

    /// 对齐 Java: `isPk`。
    #[must_use]
    pub fn is_pk(&self) -> bool {
        self.pk
    }

    /// 设置主键。
    pub fn set_pk(&mut self, pk: bool) -> &mut Self {
        self.pk = pk;
        self
    }

    /// 对齐 Java: `getColumnDef`。
    #[must_use]
    pub fn column_def(&self) -> &str {
        &self.column_def
    }

    /// 设置默认值定义。
    pub fn set_column_def(&mut self, column_def: impl Into<String>) -> &mut Self {
        self.column_def = column_def.into();
        self
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.table_name, self.name, self.type_name)
    }
}

/// 对齐 Hutool `ColumnIndexInfo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnIndexInfo {
    column_name: String,
    asc_or_desc: String,
}

impl ColumnIndexInfo {
    /// 对齐 Java: `create`。
    #[must_use]
    pub fn create(column_name: impl Into<String>, asc_or_desc: impl Into<String>) -> Self {
        Self {
            column_name: column_name.into(),
            asc_or_desc: asc_or_desc.into(),
        }
    }

    /// 对齐 Java: `getColumnName`。
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// 设置列名。
    pub fn set_column_name(&mut self, column_name: impl Into<String>) -> &mut Self {
        self.column_name = column_name.into();
        self
    }

    /// 对齐 Java: `getAscOrDesc`。
    #[must_use]
    pub fn asc_or_desc(&self) -> &str {
        &self.asc_or_desc
    }

    /// 设置升降序。
    pub fn set_asc_or_desc(&mut self, asc_or_desc: impl Into<String>) -> &mut Self {
        self.asc_or_desc = asc_or_desc.into();
        self
    }
}

impl fmt::Display for ColumnIndexInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.column_name, self.asc_or_desc)
    }
}

/// 对齐 Hutool `IndexInfo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexInfo {
    non_unique: bool,
    index_name: String,
    table_name: String,
    schema: String,
    catalog: String,
    column_index_info_list: Vec<ColumnIndexInfo>,
}

impl IndexInfo {
    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(index_name: impl Into<String>, table_name: impl Into<String>) -> Self {
        Self {
            non_unique: true,
            index_name: index_name.into(),
            table_name: table_name.into(),
            schema: String::new(),
            catalog: String::new(),
            column_index_info_list: Vec::new(),
        }
    }

    /// 对齐 Java: `isNonUnique`。
    #[must_use]
    pub fn is_non_unique(&self) -> bool {
        self.non_unique
    }

    /// 设置非唯一。
    pub fn set_non_unique(&mut self, non_unique: bool) -> &mut Self {
        self.non_unique = non_unique;
        self
    }

    /// 对齐 Java: `getIndexName`。
    #[must_use]
    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    /// 设置索引名。
    pub fn set_index_name(&mut self, index_name: impl Into<String>) -> &mut Self {
        self.index_name = index_name.into();
        self
    }

    /// 对齐 Java: `getTableName`。
    #[must_use]
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// 设置表名。
    pub fn set_table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
        self.table_name = table_name.into();
        self
    }

    /// 对齐 Java: `getSchema`。
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// 设置 schema。
    pub fn set_schema(&mut self, schema: impl Into<String>) -> &mut Self {
        self.schema = schema.into();
        self
    }

    /// 对齐 Java: `getCatalog`。
    #[must_use]
    pub fn catalog(&self) -> &str {
        &self.catalog
    }

    /// 设置 catalog。
    pub fn set_catalog(&mut self, catalog: impl Into<String>) -> &mut Self {
        self.catalog = catalog.into();
        self
    }

    /// 对齐 Java: `getColumnIndexInfoList`。
    #[must_use]
    pub fn column_index_info_list(&self) -> &[ColumnIndexInfo] {
        &self.column_index_info_list
    }

    /// 设置列索引信息。
    pub fn set_column_index_info_list(&mut self, list: Vec<ColumnIndexInfo>) -> &mut Self {
        self.column_index_info_list = list;
        self
    }
}

impl fmt::Display for IndexInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.index_name, self.table_name)
    }
}
