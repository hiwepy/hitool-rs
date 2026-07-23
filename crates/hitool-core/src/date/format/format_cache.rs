//! 对齐: format::format_cache
use crate::Result;
#[derive(Debug, Clone, Copy, Default)]
pub struct FormatCache;
impl FormatCache {
    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> { Ok(()) }
}
