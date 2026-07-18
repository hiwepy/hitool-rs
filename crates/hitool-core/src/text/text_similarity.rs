//! 对齐: `cn.hutool.core.text.TextSimilarity`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/TextSimilarity.java
//!
//! 文本相似度工具(最长公共子串、相似率)。

use crate::{CoreError, Result};

/// 对齐 Java: `TextSimilarity#`
#[derive(Debug, Clone, Copy, Default)]
pub struct TextSimilarity;

impl TextSimilarity {
    /// 对齐 Java: `TextSimilarity::similar#double (String strA, String strB)`
    pub fn similar(_a: &str, _b: &str) -> Result<f64> {
        Err(CoreError::PendingEngine("TextSimilarity::similar"))
    }

    /// 对齐 Java: `TextSimilarity::similar#String (String strA, String strB, int scale)`
    pub fn similar_scaled(_a: &str, _b: &str, _scale: i32) -> Result<String> {
        Err(CoreError::PendingEngine("TextSimilarity::similar_scaled"))
    }

    /// 对齐 Java: `TextSimilarity::longestCommonSubstring#String (String strA, String strB)`
    pub fn longest_common_substring(_a: &str, _b: &str) -> Result<String> {
        Err(CoreError::PendingEngine(
            "TextSimilarity::longest_common_substring",
        ))
    }
}