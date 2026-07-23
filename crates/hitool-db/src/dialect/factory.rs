//! 方言工厂 —— 对齐 Hutool `cn.hutool.db.dialect.DialectFactory`。

use crate::dialect::driver_name_pool as pool;
use regex::Regex;
use std::sync::LazyLock;

static JDBC_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)jdbc:(.*?):").expect("jdbc prefix"));

/// 对齐 Java: `DialectFactory.identifyDriver(String, ClassLoader)`.
#[must_use]
pub fn identify_driver(name_contains_product_info: &str) -> Option<String> {
    if name_contains_product_info.trim().is_empty() {
        return None;
    }
    let mut info = name_contains_product_info
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase();

    if let Some(caps) = JDBC_PREFIX.captures(&info) {
        info = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or(info);
    }

    if info.contains("mysql") || info.contains("cobar") {
        return Some(pool::DRIVER_MYSQL_V6.to_string());
    }
    if info.contains("oracle") {
        return Some(pool::DRIVER_ORACLE.to_string());
    }
    if info.contains("postgresql") {
        return Some(pool::DRIVER_POSTGRESQL.to_string());
    }
    if info.contains("sqlite") {
        return Some(pool::DRIVER_SQLLITE3.to_string());
    }
    if info.contains("sqlserver") || info.contains("microsoft") {
        return Some(pool::DRIVER_SQLSERVER.to_string());
    }
    if info.contains("h2") {
        return Some(pool::DRIVER_H2.to_string());
    }
    if info.contains("derby") {
        return Some(pool::DRIVER_DERBY.to_string());
    }
    if info.contains("hsqldb") {
        return Some(pool::DRIVER_HSQLDB.to_string());
    }
    if info.contains("dm") {
        return Some(pool::DRIVER_DM7.to_string());
    }
    if info.contains("kingbase8") {
        return Some(pool::DRIVER_KINGBASE8.to_string());
    }
    if info.contains("ignite") {
        return Some(pool::DRIVER_IGNITE_THIN.to_string());
    }
    if info.contains("clickhouse") {
        return Some(pool::DRIVER_CLICK_HOUSE.to_string());
    }
    if info.contains("highgo") {
        return Some(pool::DRIVER_HIGHGO.to_string());
    }
    if info.contains("db2") {
        return Some(pool::DRIVER_DB2.to_string());
    }
    if info.contains("xugu") {
        return Some(pool::DRIVER_XUGU.to_string());
    }
    if info.contains("phoenix") {
        return Some(pool::DRIVER_PHOENIX.to_string());
    }
    if info.contains("zenith") {
        return Some(pool::DRIVER_GAUSS.to_string());
    }
    if info.contains("gbase") {
        return Some(pool::DRIVER_GBASE.to_string());
    }
    if info.contains("oscar") {
        return Some(pool::DRIVER_OSCAR.to_string());
    }
    if info.contains("sybase") {
        return Some(pool::DRIVER_SYBASE.to_string());
    }
    if info.contains("mariadb") {
        return Some(pool::DRIVER_MARIADB.to_string());
    }
    if info.contains("goldendb") {
        return Some(pool::DRIVER_GOLDENDB.to_string());
    }
    None
}
