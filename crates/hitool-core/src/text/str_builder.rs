//! 对齐: `cn.hutool.core.text.StrBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrBuilder.java
//!
//! 可变字符串构建器,等价于 Java `StringBuilder`。

use crate::{CoreError, Result};

/// 对齐 Java: `StrBuilder#DEFAULT_CAPACITY`
pub const DEFAULT_CAPACITY: i32 = 16;

/// 对齐 Java: `StrBuilder#`
#[derive(Debug, Clone)]
pub struct StrBuilder;

impl Default for StrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StrBuilder {
    /// 对齐 Java: `StrBuilder::create()`
    pub fn create() -> Self {
        Self::new()
    }

    /// 对齐 Java: `StrBuilder::create(int initialCapacity)`
    pub fn with_capacity(_cap: i32) -> Self {
        Self::new()
    }

    /// 对齐 Java: `StrBuilder::create(CharSequence... strs)`
    pub fn from_strs(_strs: &[&str]) -> Self {
        Self::new()
    }

    /// 对齐 Java: `StrBuilder()`
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `StrBuilder(int initialCapacity)`
    pub fn with_capacity_ctor(_cap: i32) -> Self {
        Self::new()
    }

    /// 对齐 Java: `StrBuilder(CharSequence... strs)`
    pub fn from_strs_ctor(_strs: &[&str]) -> Self {
        Self::new()
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (Object obj)`
    pub fn append_object(&mut self, _obj: &dyn std::fmt::Display) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_object"))
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char c)`
    pub fn append_char(&mut self, _c: char) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_char"))
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char[] src)`
    pub fn append_chars(&mut self, _src: &[char]) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_chars"))
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char[] src, int srcPos, int length)`
    pub fn append_chars_range(
        &mut self,
        _src: &[char],
        _pos: i32,
        _len: i32,
    ) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_chars_range"))
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (CharSequence csq)`
    pub fn append_str(&mut self, _cs: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_str"))
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (CharSequence csq, int start, int end)`
    pub fn append_str_range(
        &mut self,
        _cs: &str,
        _start: i32,
        _end: i32,
    ) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::append_str_range"))
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, Object obj)`
    pub fn insert_object(
        &mut self,
        _index: i32,
        _obj: &dyn std::fmt::Display,
    ) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::insert_object"))
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, char c)`
    pub fn insert_char(&mut self, _index: i32, _c: char) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::insert_char"))
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, CharSequence csq)`
    pub fn insert_str(&mut self, _index: i32, _cs: &str) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::insert_str"))
    }

    /// 对齐 Java: `StrBuilder::hasContent#boolean ()`
    pub fn has_content(&self) -> Result<bool> {
        Err(CoreError::PendingEngine("StrBuilder::has_content"))
    }

    /// 对齐 Java: `StrBuilder::isEmpty#boolean ()`
    pub fn is_empty(&self) -> Result<bool> {
        Err(CoreError::PendingEngine("StrBuilder::is_empty"))
    }

    /// 对齐 Java: `StrBuilder::clear#StrBuilder ()`
    pub fn clear(&mut self) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::clear"))
    }

    /// 对齐 Java: `StrBuilder::reset#StrBuilder ()`
    pub fn reset(&mut self) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::reset"))
    }

    /// 对齐 Java: `StrBuilder::delTo#StrBuilder (int newPosition)`
    pub fn del_to(&mut self, _pos: i32) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::del_to"))
    }

    /// 对齐 Java: `StrBuilder::del#StrBuilder (int start, int end)`
    pub fn del(&mut self, _start: i32, _end: i32) -> Result<&mut Self> {
        Err(CoreError::PendingEngine("StrBuilder::del"))
    }

    /// 对齐 Java: `StrBuilder::toString#String (boolean isReset)`
    pub fn to_string_reset(&mut self, _reset: bool) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::to_string_reset"))
    }

    /// 对齐 Java: `StrBuilder::toStringAndReset#String ()`
    pub fn to_string_and_reset(&mut self) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::to_string_and_reset"))
    }

    /// 对齐 Java: `StrBuilder::toString#String ()`
    pub fn to_string(&self) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::to_string"))
    }

    /// 对齐 Java: `StrBuilder::length#int ()`
    pub fn length(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("StrBuilder::length"))
    }

    /// 对齐 Java: `StrBuilder::charAt#char (int index)`
    pub fn char_at(&self, _index: i32) -> Result<char> {
        Err(CoreError::PendingEngine("StrBuilder::char_at"))
    }

    /// 对齐 Java: `StrBuilder::subSequence#CharSequence (int start, int end)`
    pub fn sub_sequence(&self, _start: i32, _end: i32) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::sub_sequence"))
    }

    /// 对齐 Java: `StrBuilder::subString#String (int start)`
    pub fn sub_string(&self, _start: i32) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::sub_string"))
    }

    /// 对齐 Java: `StrBuilder::subString#String (int start, int end)`
    pub fn sub_string_range(&self, _start: i32, _end: i32) -> Result<String> {
        Err(CoreError::PendingEngine("StrBuilder::sub_string_range"))
    }

    /// 对齐 Java: `StrBuilder::getChars#void (int srcBegin, int srcEnd, char[] dst, int dstBegin)`
    pub fn get_chars(
        &self,
        _begin: i32,
        _end: i32,
        _dst: &mut [char],
        _dst_begin: i32,
    ) -> Result<()> {
        Err(CoreError::PendingEngine("StrBuilder::get_chars"))
    }
}