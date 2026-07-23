//! 方言名枚举 —— 对齐 Hutool `cn.hutool.db.dialect.DialectName`。

/// 对齐 Hutool `DialectName`：列出 hutool-db 识别的数据库方言。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialectName {
    /// ANSI SQL 通用方言。
    Ansi,
    /// MySQL。
    Mysql,
    /// Oracle。
    Oracle,
    /// PostgreSQL。
    Postgresql,
    /// SQLite3。
    Sqlite3,
    /// H2。
    H2,
    /// SQL Server。
    Sqlserver,
    /// SQL Server 2012+。
    Sqlserver2012,
    /// Apache Phoenix。
    Phoenix,
    /// 达梦 DM。
    Dm,
    /// SAP HANA。
    Hana,
}

impl DialectName {
    /// 对齐 Java: `DialectName.match(String)` —— 忽略大小写比较方言名。
    #[must_use]
    pub fn match_name(self, dialect_name: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(dialect_name)
    }

    /// 返回与 Hutool `name()` 一致的大写方言标识。
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ansi => "ANSI",
            Self::Mysql => "MYSQL",
            Self::Oracle => "ORACLE",
            Self::Postgresql => "POSTGRESQL",
            Self::Sqlite3 => "SQLITE3",
            Self::H2 => "H2",
            Self::Sqlserver => "SQLSERVER",
            Self::Sqlserver2012 => "SQLSERVER2012",
            Self::Phoenix => "PHOENIX",
            Self::Dm => "DM",
            Self::Hana => "HANA",
        }
    }

    /// 从产品/URL 文本解析方言名（委托 `identify_driver` 文本分类）。
    #[must_use]
    pub fn from_product_info(info: &str) -> Option<Self> {
        let lower = info.to_ascii_lowercase();
        if lower.contains("mysql") || lower.contains("mariadb") || lower.contains("goldendb") {
            Some(Self::Mysql)
        } else if lower.contains("oracle") {
            Some(Self::Oracle)
        } else if lower.contains("postgresql") || lower.contains("postgres") {
            Some(Self::Postgresql)
        } else if lower.contains("sqlite") {
            Some(Self::Sqlite3)
        } else if lower.contains("h2") {
            Some(Self::H2)
        } else if lower.contains("sqlserver2012") {
            Some(Self::Sqlserver2012)
        } else if lower.contains("sqlserver") || lower.contains("microsoft") {
            Some(Self::Sqlserver)
        } else if lower.contains("phoenix") {
            Some(Self::Phoenix)
        } else if lower.contains("dm") {
            Some(Self::Dm)
        } else if lower.contains("hana") {
            Some(Self::Hana)
        } else {
            Some(Self::Ansi)
        }
    }
}

impl std::fmt::Display for DialectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
