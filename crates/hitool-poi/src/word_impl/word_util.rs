//! Word static facade aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.word.WordUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/word/WordUtil.java

use crate::word_impl::word07_writer::Word07Writer;

/// Word utility facade.
///
/// 对齐 Java: `cn.hutool.poi.word.WordUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct WordUtil;

impl WordUtil {
    /// 对齐 Java: `WordUtil.getWriter()`
    pub fn get_writer() -> Word07Writer {
        Word07Writer::new()
    }
    /// 对齐 Java: `WordUtil.getWriter(File destFile)`
    pub fn get_writer_dest(_dest_file: &str) -> Word07Writer {
        Word07Writer::new()
    }
}