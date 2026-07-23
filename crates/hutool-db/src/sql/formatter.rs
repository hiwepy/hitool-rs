//! SQL 格式化 —— 对齐 Hutool `cn.hutool.db.sql.SqlFormatter`（简化实现）。

/// 对齐 Java: `SqlFormatter.format(String)` — 保证复杂 SQL 不 panic。
#[must_use]
pub fn format(source: &str) -> String {
    if source.trim().is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut depth = 0usize;
    for ch in source.chars() {
        match ch {
            '(' => {
                depth += 1;
                out.push(ch);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}
