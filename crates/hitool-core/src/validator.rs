//! 对齐: `cn.hutool.core.lang.Validator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Validator.java
//!
//! Rust 版本提供数据验证的 idiomatic 实现。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.Validator`
#[derive(Debug, Clone, Copy, Default)]
pub struct Validator;

impl Validator {
    // ── 空值验证 ──

    /// 对齐 Java: `Validator.isNotEmpty(CharSequence)`
    pub fn is_not_empty(value: Option<&str>) -> bool {
        value.map_or(false, |s| !s.is_empty())
    }

    /// 对齐 Java: `Validator.isEmpty(CharSequence)`
    pub fn is_empty(value: Option<&str>) -> bool {
        value.map_or(true, |s| s.is_empty())
    }

    // ── 长度验证 ──

    /// 对齐 Java: `Validator.isBetween(CharSequence, int, int)`
    pub fn is_between(value: &str, min: usize, max: usize) -> bool {
        let len = value.len();
        len >= min && len <= max
    }

    /// 对齐 Java: `Validator.minLength(CharSequence, int)`
    pub fn min_length(value: &str, min: usize) -> bool {
        value.len() >= min
    }

    /// 对齐 Java: `Validator.maxLength(CharSequence, int)`
    pub fn max_length(value: &str, max: usize) -> bool {
        value.len() <= max
    }

    // ── 格式验证 ──

    /// 对齐 Java: `Validator.isEmail(CharSequence)`
    pub fn is_email(value: &str) -> bool {
        crate::re_util::ReUtil::is_email(value)
    }

    /// 对齐 Java: `Validator.isMobile(CharSequence)`
    pub fn is_mobile(value: &str) -> bool {
        crate::re_util::ReUtil::is_mobile(value)
    }

    /// 对齐 Java: `Validator.isIpv4(CharSequence)`
    pub fn is_ipv4(value: &str) -> bool {
        crate::re_util::ReUtil::is_ipv4(value)
    }

    /// 对齐 Java: `Validator.isIpv6(CharSequence)`
    pub fn is_ipv6(value: &str) -> bool {
        crate::re_util::ReUtil::is_ipv6(value)
    }

    /// 对齐 Java: `Validator.isUrl(CharSequence)`
    pub fn is_url(value: &str) -> bool {
        crate::re_util::ReUtil::is_url(value)
    }

    /// 对齐 Java: `Validator.isChinese(CharSequence)`
    pub fn is_chinese(value: &str) -> bool {
        crate::re_util::ReUtil::is_chinese(value)
    }

    /// 对齐 Java: `Validator.isIdCard(CharSequence)`
    pub fn is_id_card(value: &str) -> bool {
        crate::idcard_util::IdcardUtil::is_valid_card(value)
    }

    // ── 数字验证 ──

    /// 对齐 Java: `Validator.isNumber(CharSequence)`
    pub fn is_number(value: &str) -> bool {
        crate::number_util::NumberUtil::is_number(value)
    }

    /// 对齐 Java: `Validator.isInteger(CharSequence)`
    pub fn is_integer(value: &str) -> bool {
        crate::number_util::NumberUtil::is_integer(value)
    }

    // ── 范围验证 ──

    /// 对齐 Java: `Validator.isBetween(long, long, long)`
    pub fn is_between_i64(value: i64, min: i64, max: i64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `Validator.isBetween(double, double, double)`
    pub fn is_between_f64(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    // ── 正则验证 ──

    /// 对齐 Java: `Validator.isMactchRegex(String, CharSequence)`
    pub fn is_match_regex(pattern: &str, value: &str) -> bool {
        crate::re_util::ReUtil::is_match(pattern, value)
    }

    // ── 集合验证 ──

    /// 对齐 Java: `Validator.isNotEmpty(Collection)`
    pub fn is_not_empty_collection<T>(items: &[T]) -> bool {
        !items.is_empty()
    }

    /// 对齐 Java: `Validator.isEmpty(Collection)`
    pub fn is_empty_collection<T>(items: &[T]) -> bool {
        items.is_empty()
    }

    // ── 通用验证 ──

    /// 对齐 Java: `Validator.validateNotEmpty(CharSequence, String)`
    pub fn validate_not_empty(value: Option<&str>, name: &str) -> Result<()> {
        if Self::is_empty(value) {
            return Err(CoreError::InvalidArgument {
                name: Box::leak(name.to_string().into_boxed_str()),
                reason: "must not be empty",
            });
        }
        Ok(())
    }

    /// 对齐 Java: `Validator.validateBetween(CharSequence, int, int, String)`
    pub fn validate_between(value: &str, min: usize, max: usize, name: &str) -> Result<()> {
        if !Self::is_between(value, min, max) {
            return Err(CoreError::InvalidArgument {
                name: Box::leak(name.to_string().into_boxed_str()),
                reason: "length out of range",
            });
        }
        Ok(())
    }
}
