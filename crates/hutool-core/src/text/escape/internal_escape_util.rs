//! 对齐: `cn.hutool.core.text.escape.InternalEscapeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/InternalEscapeUtil.java

use crate::Result;

/// 对齐 Java: `InternalEscapeUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct InternalEscapeUtil;

impl InternalEscapeUtil {
    /// 对齐 Java: `InternalEscapeUtil::invert#String[][] (String[][])`
    ///
    /// 将 `(key, value)` 查找表反转为 `(value, key)`。
    pub fn invert(array: &[(&str, &str)]) -> Result<Vec<(String, String)>> {
        Ok(array
            .iter()
            .map(|(k, v)| (v.to_string(), k.to_string()))
            .collect())
    }
}
