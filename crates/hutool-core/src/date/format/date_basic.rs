//! 对齐: format::date_basic
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct DateBasic;
impl DateBasic {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
