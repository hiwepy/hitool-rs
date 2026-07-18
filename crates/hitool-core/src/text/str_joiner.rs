//! 对齐: `cn.hutool.core.text.StrJoiner`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrJoiner.java
//!
//! 字符串拼接器(类似 Java `StringJoiner`,支持前缀/后缀/`null` 模式)。

use crate::{CoreError, Result};

/// 对齐 Java: `StrJoiner#NullMode` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullMode {
    /// 忽略 null
    Ignore,
    /// 视为空字符串
    ToEmpty,
    /// 抛出异常
    Throw,
}

/// 对齐 Java: `StrJoiner#`
#[derive(Debug, Clone)]
pub struct StrJoiner;

impl Default for StrJoiner {
    fn default() -> Self {
        Self::of("")
    }
}

impl StrJoiner {
    /// 对齐 Java: `StrJoiner::of(StrJoiner joiner)`
    pub fn of_joiner(_other: &StrJoiner) -> Self {
        Self::of("")
    }

    /// 对齐 Java: `StrJoiner::of(CharSequence delimiter)`
    pub fn of(_delim: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrJoiner::of(CharSequence delimiter, CharSequence prefix, CharSequence suffix)`
    pub fn of_wrapped(_delim: &str, _prefix: &str, _suffix: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrJoiner(CharSequence delimiter)`
    pub fn new_join(_delim: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrJoiner(Appendable appendable, CharSequence delimiter)`
    pub fn with_appendable(_delim: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrJoiner(CharSequence delimiter, CharSequence prefix, CharSequence suffix)`
    pub fn new_join_wrapped(_delim: &str, _prefix: &str, _suffix: &str) -> Self {
        Self
    }

    /// 对齐 Java: `StrJoiner::setDelimiter#StrJoiner (CharSequence)`
    pub fn set_delimiter(&mut self, _delim: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_delimiter"))
    }

    /// 对齐 Java: `StrJoiner::setPrefix#StrJoiner (CharSequence)`
    pub fn set_prefix(&mut self, _prefix: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_prefix"))
    }

    /// 对齐 Java: `StrJoiner::setSuffix#StrJoiner (CharSequence)`
    pub fn set_suffix(&mut self, _suffix: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_suffix"))
    }

    /// 对齐 Java: `StrJoiner::setWrapElement#StrJoiner (boolean)`
    pub fn set_wrap_element(&mut self, _wrap: bool) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_wrap_element"))
    }

    /// 对齐 Java: `StrJoiner::setNullMode#StrJoiner (NullMode)`
    pub fn set_null_mode(&mut self, _mode: NullMode) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_null_mode"))
    }

    /// 对齐 Java: `StrJoiner::setEmptyResult#StrJoiner (String)`
    pub fn set_empty_result(&mut self, _empty: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::set_empty_result"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (Object)`
    pub fn append_object(&mut self, _obj: &dyn std::fmt::Display) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_object"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (T[] array)`
    pub fn append_array<T: std::fmt::Display>(
        &mut self,
        _arr: &[T],
    ) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_array"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (Iterator<T>)`
    pub fn append_iter(&mut self) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_iter"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (CharSequence csq)`
    pub fn append_str(&mut self, _cs: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_str"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (CharSequence csq, int startInclude, int endExclude)`
    pub fn append_str_range(
        &mut self,
        _cs: &str,
        _start: i32,
        _end: i32,
    ) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_str_range"))
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (char c)`
    pub fn append_char(&mut self, _c: char) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::append_char"))
    }

    /// 对齐 Java: `StrJoiner::merge#StrJoiner (StrJoiner)`
    pub fn merge(&mut self, _other: &StrJoiner) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrJoiner::merge"))
    }

    /// 对齐 Java: `StrJoiner::length#int ()`
    pub fn length(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("StrJoiner::length"))
    }

    /// 对齐 Java: `StrJoiner::toString#String ()`
    pub fn to_string(&self) -> Result<String> {
        Err(CoreError::PendingEngine("StrJoiner::to_string"))
    }
}