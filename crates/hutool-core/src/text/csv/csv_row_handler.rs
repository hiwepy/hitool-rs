//! 对齐: `cn.hutool.core.text.csv.CsvRowHandler`

/// 对齐 Java: `CsvRowHandler#`（Rust 用闭包替代）
pub trait CsvRowHandler {
    /// 处理一行
    fn handle(&mut self, row: &super::csv_row::CsvRow);
}
