//! 对齐: format::abstract_date_basic
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct AbstractDateBasic;
impl AbstractDateBasic {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
