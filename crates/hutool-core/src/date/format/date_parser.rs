//! 对齐: format::date_parser
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct DateParser;
impl DateParser {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
