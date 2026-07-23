//! 对齐: format::fast_date_parser
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct FastDateParser;
impl FastDateParser {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
