//! 对齐: format::global_custom_format
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalCustomFormat;
impl GlobalCustomFormat {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
