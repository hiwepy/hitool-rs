//! 对齐: format::date_printer
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct DatePrinter;
impl DatePrinter {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
