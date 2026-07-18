//! 对齐: `cn.hutool.core.text.CharSequenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/CharSequenceUtil.java
//!
//! 字符序列工具类(Java 中为 `StrUtil` 的实现基类),含判空、切片、替换、格式化、Unicode 等。
//! 注意: hitool-rs 已存在 `string.rs`(`is_blank`、`is_empty`、`trim`、`split` 等),
//! 本文件按 Java 类粒度提供对齐桩,具体实现在引擎层完成。

use crate::{CoreError, Result};

/// 对齐 Java: `CharSequenceUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct CharSequenceUtil;

impl CharSequenceUtil {
    // ---- 常量对齐 ----

    /// 对齐 Java: `CharSequenceUtil::INDEX_NOT_FOUND`
    pub const INDEX_NOT_FOUND: i32 = -1;
    /// 对齐 Java: `CharSequenceUtil::NULL`
    pub const NULL: &'static str = "null";
    /// 对齐 Java: `CharSequenceUtil::EMPTY`
    pub const EMPTY: &'static str = "";
    /// 对齐 Java: `CharSequenceUtil::SPACE`
    pub const SPACE: &'static str = " ";

    // ---- 判空 ----

    /// 对齐 Java: `CharSequenceUtil::isBlank#boolean (CharSequence str)`
    pub fn is_blank(_str: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::isNotBlank#boolean (CharSequence str)`
    pub fn is_not_blank(_str: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_not_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::hasBlank#boolean (CharSequence... strs)`
    pub fn has_blank(_strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::has_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllBlank#boolean (CharSequence... strs)`
    pub fn is_all_blank(_strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_all_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::isEmpty#boolean (CharSequence str)`
    pub fn is_empty(_str: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::isNotEmpty#boolean (CharSequence str)`
    pub fn is_not_empty(_str: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_not_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::emptyIfNull#String (CharSequence str)`
    pub fn empty_if_null(_str: Option<&str>) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::empty_if_null"))
    }

    /// 对齐 Java: `CharSequenceUtil::nullToEmpty#String (CharSequence str)`
    pub fn null_to_empty(_str: Option<&str>) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::null_to_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::nullToDefault#String (CharSequence str, String defaultStr)`
    pub fn null_to_default(_str: Option<&str>, _default: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::null_to_default"))
    }

    /// 对齐 Java: `CharSequenceUtil::emptyToDefault#String (CharSequence str, String defaultStr)`
    pub fn empty_to_default(_str: &str, _default: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::empty_to_default"))
    }

    /// 对齐 Java: `CharSequenceUtil::blankToDefault#String (CharSequence str, String defaultStr)`
    pub fn blank_to_default(_str: &str, _default: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::blank_to_default"))
    }

    /// 对齐 Java: `CharSequenceUtil::emptyToNull#String (CharSequence str)`
    pub fn empty_to_null(_str: &str) -> Result<Option<String>> {
        Err(CoreError::PendingEngine("CharSequenceUtil::empty_to_null"))
    }

    /// 对齐 Java: `CharSequenceUtil::hasEmpty#boolean (CharSequence... strs)`
    pub fn has_empty(_strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::has_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllEmpty#boolean (CharSequence... strs)`
    pub fn is_all_empty(_strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_all_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllNotEmpty#boolean (CharSequence... args)`
    pub fn is_all_not_empty(_args: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_all_not_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllNotBlank#boolean (CharSequence... args)`
    pub fn is_all_not_blank(_args: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_all_not_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::isNullOrUndefined#boolean (CharSequence str)`
    pub fn is_null_or_undefined(_str: Option<&str>) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_null_or_undefined"))
    }

    /// 对齐 Java: `CharSequenceUtil::isEmptyOrUndefined#boolean (CharSequence str)`
    pub fn is_empty_or_undefined(_str: Option<&str>) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_empty_or_undefined"))
    }

    /// 对齐 Java: `CharSequenceUtil::isBlankOrUndefined#boolean (CharSequence str)`
    pub fn is_blank_or_undefined(_str: Option<&str>) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_blank_or_undefined"))
    }

    // ---- 修剪 ----

    /// 对齐 Java: `CharSequenceUtil::trim#String (CharSequence str)`
    pub fn trim(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::trim"))
    }

    /// 对齐 Java: `CharSequenceUtil::trimToEmpty#String (CharSequence str)`
    pub fn trim_to_empty(_str: Option<&str>) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::trim_to_empty"))
    }

    /// 对齐 Java: `CharSequenceUtil::trimToNull#String (CharSequence str)`
    pub fn trim_to_null(_str: Option<&str>) -> Result<Option<String>> {
        Err(CoreError::PendingEngine("CharSequenceUtil::trim_to_null"))
    }

    /// 对齐 Java: `CharSequenceUtil::trimStart#String (CharSequence str)`
    pub fn trim_start(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::trim_start"))
    }

    /// 对齐 Java: `CharSequenceUtil::trimEnd#String (CharSequence str)`
    pub fn trim_end(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::trim_end"))
    }

    // ---- 前缀/后缀判断 ----

    /// 对齐 Java: `CharSequenceUtil::startWith#boolean (CharSequence str, char c)`
    pub fn start_with_char(_str: &str, _c: char) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::start_with_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::startWith#boolean (CharSequence str, CharSequence prefix, boolean ignoreCase)`
    pub fn start_with(_str: &str, _prefix: &str, _ignore_case: bool) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::start_with"))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithIgnoreEquals#boolean (CharSequence str, CharSequence prefix)`
    pub fn start_with_ignore_equals(_str: &str, _prefix: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::start_with_ignore_equals"))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithIgnoreCase#boolean (CharSequence str, CharSequence prefix)`
    pub fn start_with_ignore_case(_str: &str, _prefix: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::start_with_ignore_case"))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithAny#boolean (CharSequence str, CharSequence... prefixes)`
    pub fn start_with_any(_str: &str, _prefixes: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::start_with_any"))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithAnyIgnoreCase#boolean (CharSequence str, CharSequence... prefixes)`
    pub fn start_with_any_ignore_case(_str: &str, _prefixes: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::start_with_any_ignore_case",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::endWith#boolean (CharSequence str, char c)`
    pub fn end_with_char(_str: &str, _c: char) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::end_with_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::endWith#boolean (CharSequence str, CharSequence suffix, boolean ignoreCase)`
    pub fn end_with(_str: &str, _suffix: &str, _ignore_case: bool) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::end_with"))
    }

    /// 对齐 Java: `CharSequenceUtil::endWithIgnoreCase#boolean (CharSequence str, CharSequence suffix)`
    pub fn end_with_ignore_case(_str: &str, _suffix: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::end_with_ignore_case"))
    }

    /// 对齐 Java: `CharSequenceUtil::endWithAny#boolean (CharSequence str, CharSequence... suffixes)`
    pub fn end_with_any(_str: &str, _suffixes: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::end_with_any"))
    }

    /// 对齐 Java: `CharSequenceUtil::endWithAnyIgnoreCase#boolean (CharSequence str, CharSequence... suffixes)`
    pub fn end_with_any_ignore_case(_str: &str, _suffixes: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::end_with_any_ignore_case",
        ))
    }

    // ---- 包含/位置 ----

    /// 对齐 Java: `CharSequenceUtil::contains#boolean (CharSequence str, char searchChar)`
    pub fn contains_char(_str: &str, _c: char) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::contains#boolean (CharSequence str, CharSequence searchStr)`
    pub fn contains(_str: &str, _search: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains"))
    }

    /// 对齐 Java: `CharSequenceUtil::containsAny#boolean (CharSequence str, CharSequence... testStrs)`
    pub fn contains_any(_str: &str, _test_strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains_any"))
    }

    /// 对齐 Java: `CharSequenceUtil::containsOnly#boolean (CharSequence str, char... testChars)`
    pub fn contains_only(_str: &str, _test_chars: &[char]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains_only"))
    }

    /// 对齐 Java: `CharSequenceUtil::containsAll#boolean (CharSequence str, CharSequence... testChars)`
    pub fn contains_all(_str: &str, _test_chars: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains_all"))
    }

    /// 对齐 Java: `CharSequenceUtil::containsBlank#boolean (CharSequence str)`
    pub fn contains_blank(_str: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::contains_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::containsIgnoreCase#boolean (CharSequence str, CharSequence testStr)`
    pub fn contains_ignore_case(_str: &str, _test: &str) -> Result<bool> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::contains_ignore_case",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOf#int (CharSequence str, char searchChar)`
    pub fn index_of_char(_str: &str, _c: char) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::index_of_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOf#int (CharSequence str, char searchChar, int start)`
    pub fn index_of_char_start(_str: &str, _c: char, _start: i32) -> Result<i32> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::index_of_char_start",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOfIgnoreCase#int (CharSequence str, CharSequence searchStr)`
    pub fn index_of_ignore_case(_str: &str, _search: &str) -> Result<i32> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::index_of_ignore_case",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::ordinalIndexOf#int (CharSequence str, CharSequence searchStr, int ordinal)`
    pub fn ordinal_index_of(_str: &str, _search: &str, _ordinal: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::ordinal_index_of"))
    }

    // ---- 删除/剥离 ----

    /// 对齐 Java: `CharSequenceUtil::removeAll#String (CharSequence str, CharSequence strToRemove)`
    pub fn remove_all(_str: &str, _to_remove: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::remove_all"))
    }

    /// 对齐 Java: `CharSequenceUtil::removeAllLineBreaks#String (CharSequence str)`
    pub fn remove_all_line_breaks(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::remove_all_line_breaks",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::removePrefix#String (CharSequence str, CharSequence prefix)`
    pub fn remove_prefix(_str: &str, _prefix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::remove_prefix"))
    }

    /// 对齐 Java: `CharSequenceUtil::removeSuffix#String (CharSequence str, CharSequence suffix)`
    pub fn remove_suffix(_str: &str, _suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::remove_suffix"))
    }

    /// 对齐 Java: `CharSequenceUtil::cleanBlank#String (CharSequence str)`
    pub fn clean_blank(_str: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::clean_blank"))
    }

    /// 对齐 Java: `CharSequenceUtil::strip#String (CharSequence str, CharSequence prefixOrSuffix)`
    pub fn strip(_str: &str, _prefix_or_suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::strip"))
    }

    /// 对齐 Java: `CharSequenceUtil::strip#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn strip_full(
        _str: &str,
        _prefix: &str,
        _suffix: &str,
    ) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::strip_full"))
    }

    /// 对齐 Java: `CharSequenceUtil::addPrefixIfNot#String (CharSequence str, CharSequence prefix)`
    pub fn add_prefix_if_not(_str: &str, _prefix: &str) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::add_prefix_if_not",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::addSuffixIfNot#String (CharSequence str, CharSequence suffix)`
    pub fn add_suffix_if_not(_str: &str, _suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::add_suffix_if_not",
        ))
    }

    // ---- 切割/切片 ----

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, char separator)`
    pub fn split_char(_str: &str, _sep: char) -> Result<Vec<String>> {
        Err(CoreError::PendingEngine("CharSequenceUtil::split_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, char separator, int limit)`
    pub fn split_char_limit(_str: &str, _sep: char, _limit: i32) -> Result<Vec<String>> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::split_char_limit",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, CharSequence separator)`
    pub fn split_str(_str: &str, _sep: &str) -> Result<Vec<String>> {
        Err(CoreError::PendingEngine("CharSequenceUtil::split_str"))
    }

    /// 对齐 Java: `CharSequenceUtil::cut#String[] (CharSequence str, int partLength)`
    pub fn cut(_str: &str, _part_length: i32) -> Result<Vec<String>> {
        Err(CoreError::PendingEngine("CharSequenceUtil::cut"))
    }

    /// 对齐 Java: `CharSequenceUtil::sub#String (CharSequence str, int fromIndexInclude, int toIndexExclude)`
    pub fn sub(_str: &str, _from: i32, _to: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub"))
    }

    /// 对齐 Java: `CharSequenceUtil::subPre#String (CharSequence string, int toIndexExclude)`
    pub fn sub_pre(_str: &str, _to_exclude: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub_pre"))
    }

    /// 对齐 Java: `CharSequenceUtil::subSuf#String (CharSequence string, int fromIndex)`
    pub fn sub_suf(_str: &str, _from: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub_suf"))
    }

    /// 对齐 Java: `CharSequenceUtil::subBefore#String (CharSequence string, CharSequence separator, boolean isLastSeparator)`
    pub fn sub_before(_str: &str, _sep: &str, _last: bool) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub_before"))
    }

    /// 对齐 Java: `CharSequenceUtil::subAfter#String (CharSequence string, CharSequence separator, boolean isLastSeparator)`
    pub fn sub_after(_str: &str, _sep: &str, _last: bool) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub_after"))
    }

    /// 对齐 Java: `CharSequenceUtil::subBetween#String (CharSequence str, CharSequence before, CharSequence after)`
    pub fn sub_between(_str: &str, _before: &str, _after: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::sub_between"))
    }

    // ---- 重复/拼接/填充 ----

    /// 对齐 Java: `CharSequenceUtil::repeat#String (char c, int count)`
    pub fn repeat_char(_c: char, _count: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::repeat_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::repeat#String (CharSequence str, int count)`
    pub fn repeat(_str: &str, _count: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::repeat"))
    }

    /// 对齐 Java: `CharSequenceUtil::repeatAndJoin#String (CharSequence str, int count, CharSequence delimiter)`
    pub fn repeat_and_join(_str: &str, _count: i32, _delim: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::repeat_and_join"))
    }

    /// 对齐 Java: `CharSequenceUtil::wrap#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn wrap(_str: &str, _prefix: &str, _suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::wrap"))
    }

    /// 对齐 Java: `CharSequenceUtil::wrapIfMissing#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn wrap_if_missing(_str: &str, _prefix: &str, _suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::wrap_if_missing"))
    }

    /// 对齐 Java: `CharSequenceUtil::unWrap#String (CharSequence str, String prefix, String suffix)`
    pub fn unwrap(_str: &str, _prefix: &str, _suffix: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::unwrap"))
    }

    /// 对齐 Java: `CharSequenceUtil::isWrap#boolean (CharSequence str, String prefix, String suffix)`
    pub fn is_wrap(_str: &str, _prefix: &str, _suffix: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::is_wrap"))
    }

    /// 对齐 Java: `CharSequenceUtil::padPre#String (CharSequence str, int length, char padChar)`
    pub fn pad_pre_char(_str: &str, _length: i32, _pad: char) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::pad_pre_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::padAfter#String (CharSequence str, int length, char padChar)`
    pub fn pad_after_char(_str: &str, _length: i32, _pad: char) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::pad_after_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::center#String (CharSequence str, final int size)`
    pub fn center(_str: &str, _size: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::center"))
    }

    // ---- 比较 ----

    /// 对齐 Java: `CharSequenceUtil::equals#boolean (CharSequence str1, CharSequence str2)`
    pub fn equals(_a: &str, _b: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::equals"))
    }

    /// 对齐 Java: `CharSequenceUtil::equalsIgnoreCase#boolean (CharSequence str1, CharSequence str2)`
    pub fn equals_ignore_case(_a: &str, _b: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::equals_ignore_case"))
    }

    /// 对齐 Java: `CharSequenceUtil::equalsAny#boolean (CharSequence str1, CharSequence... strs)`
    pub fn equals_any(_a: &str, _strs: &[&str]) -> Result<bool> {
        Err(CoreError::PendingEngine("CharSequenceUtil::equals_any"))
    }

    /// 对齐 Java: `CharSequenceUtil::compare#int (CharSequence str1, CharSequence str2, boolean nullIsLess)`
    pub fn compare(_a: &str, _b: &str, _null_is_less: bool) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::compare"))
    }

    /// 对齐 Java: `CharSequenceUtil::count#int (CharSequence content, CharSequence strForSearch)`
    pub fn count_str(_content: &str, _search: &str) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::count_str"))
    }

    /// 对齐 Java: `CharSequenceUtil::count#int (CharSequence content, char charForSearch)`
    pub fn count_char(_content: &str, _c: char) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::count_char"))
    }

    /// 对齐 Java: `CharSequenceUtil::length#int (CharSequence cs)`
    pub fn length(_cs: &str) -> Result<i32> {
        Err(CoreError::PendingEngine("CharSequenceUtil::length"))
    }

    // ---- 替换 ----

    /// 对齐 Java: `CharSequenceUtil::replace#String (CharSequence str, CharSequence searchStr, CharSequence replacement)`
    pub fn replace(_str: &str, _search: &str, _replacement: &str) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::replace"))
    }

    /// 对齐 Java: `CharSequenceUtil::replaceIgnoreCase#String (CharSequence str, CharSequence searchStr, CharSequence replacement)`
    pub fn replace_ignore_case(
        _str: &str,
        _search: &str,
        _replacement: &str,
    ) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::replace_ignore_case",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::replaceLast#String (CharSequence str, CharSequence searchStr, CharSequence replacedStr)`
    pub fn replace_last(
        _str: &str,
        _search: &str,
        _replacement: &str,
    ) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::replace_last"))
    }

    /// 对齐 Java: `CharSequenceUtil::replaceFirst#String (CharSequence str, CharSequence searchStr, CharSequence replacedStr)`
    pub fn replace_first(
        _str: &str,
        _search: &str,
        _replacement: &str,
    ) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::replace_first"))
    }

    /// 对齐 Java: `CharSequenceUtil::hide#String (CharSequence str, int startInclude, int endExclude)`
    pub fn hide(_str: &str, _start: i32, _end: i32) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::hide"))
    }

    // ---- 格式化 ----

    /// 对齐 Java: `CharSequenceUtil::format#String (CharSequence template, Object... params)`
    pub fn format(_template: &str, _params: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::format"))
    }

    /// 对齐 Java: `CharSequenceUtil::indexedFormat#String (CharSequence pattern, Object... arguments)`
    pub fn indexed_format(_pattern: &str, _args: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine("CharSequenceUtil::indexed_format"))
    }

    /// 对齐 Java: `CharSequenceUtil::appendIfMissing#String (CharSequence str, CharSequence suffix, CharSequence... suffixes)`
    pub fn append_if_missing(_str: &str, _suffix: &str, _others: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::append_if_missing",
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::prependIfMissing#String (CharSequence str, CharSequence prefix, CharSequence... prefixes)`
    pub fn prepend_if_missing(_str: &str, _prefix: &str, _others: &[&str]) -> Result<String> {
        Err(CoreError::PendingEngine(
            "CharSequenceUtil::prepend_if_missing",
        ))
    }
}