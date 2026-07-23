//! 对齐: `cn.hutool.core.util.StrUtil`（parity 子集）
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/StrUtil.java

use crate::text::naming_case::NamingCase;
use crate::text::str_splitter::StrSplitter;
use crate::Result;

/// 对齐 Java: `cn.hutool.core.util.StrUtil`（Issue/parity 所需方法）
#[derive(Debug, Clone, Copy, Default)]
pub struct StrUtil;

impl StrUtil {
    /// 对齐 Java: `CharSequenceUtil.split(CharSequence, char)`
    pub fn split(str: &str, separator: char) -> Result<Vec<String>> {
        if str.is_empty() {
            return Ok(vec![String::new()]);
        }
        StrSplitter::split_char(str, separator, false, false)
    }

    /// 对齐 Java: `CharSequenceUtil.splitTrim(CharSequence, char)`
    pub fn split_trim(str: &str, separator: char) -> Result<Vec<String>> {
        if str.is_empty() {
            return Ok(Vec::new());
        }
        StrSplitter::split_char(str, separator, true, true)
    }

    /// 对齐 Java: `StrUtil.toUnderlineCase` → `NamingCase.toUnderlineCase`
    pub fn to_underline_case(str: &str) -> Result<String> {
        NamingCase::to_underline_case(str)
    }
}
