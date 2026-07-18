//! 对齐: `cn.hutool.core.text.csv.CsvRowHandler`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvRowHandler.java

/// 对齐 Java: `CsvRowHandler#` 接口
pub trait CsvRowHandler {
    /// 对齐 Java: `CsvRowHandler::handle#void (CsvRow)`
    fn handle(&mut self, _row: ());
}