//! 对齐: `cn.hutool.core.text.AntPathMatcher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/AntPathMatcher.java
//!
//! Ant 风格路径匹配器(`?`、`*`、`**` 通配)。

use crate::{CoreError, Result};

/// 对齐 Java: `AntPathMatcher#DEFAULT_PATH_SEPARATOR`
pub const DEFAULT_PATH_SEPARATOR: &str = "/";

/// 对齐 Java: `AntPathMatcher#`
#[derive(Debug, Clone)]
pub struct AntPathMatcher;

impl Default for AntPathMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl AntPathMatcher {
    /// 对齐 Java: `AntPathMatcher()`
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `AntPathMatcher(String pathSeparator)`
    pub fn with_separator(_separator: &str) -> Self {
        Self
    }

    /// 对齐 Java: `AntPathMatcher::setPathSeparator#AntPathMatcher (String)`
    pub fn set_path_separator(&mut self, _separator: &str) -> Result<()> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::set_path_separator",
        ))
    }

    /// 对齐 Java: `AntPathMatcher::setCaseSensitive#AntPathMatcher (boolean)`
    pub fn set_case_sensitive(&mut self, _case_sensitive: bool) -> Result<()> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::set_case_sensitive",
        ))
    }

    /// 对齐 Java: `AntPathMatcher::setTrimTokens#AntPathMatcher (boolean)`
    pub fn set_trim_tokens(&mut self, _trim: bool) -> Result<()> {
        Err(CoreError::PendingEngine("AntPathMatcher::set_trim_tokens"))
    }

    /// 对齐 Java: `AntPathMatcher::setCachePatterns#AntPathMatcher (boolean)`
    pub fn set_cache_patterns(&mut self, _cache: bool) -> Result<()> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::set_cache_patterns",
        ))
    }

    /// 对齐 Java: `AntPathMatcher::isPattern#boolean (String path)`
    pub fn is_pattern(&self, _path: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("AntPathMatcher::is_pattern"))
    }

    /// 对齐 Java: `AntPathMatcher::match#boolean (String pattern, String path)`
    pub fn match_path(&self, _pattern: &str, _path: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("AntPathMatcher::match_path"))
    }

    /// 对齐 Java: `AntPathMatcher::matchStart#boolean (String pattern, String path)`
    pub fn match_start(&self, _pattern: &str, _path: &str) -> Result<bool> {
        Err(CoreError::PendingEngine("AntPathMatcher::match_start"))
    }

    /// 对齐 Java: `AntPathMatcher::extractPathWithinPattern#String (String pattern, String path)`
    pub fn extract_path_within_pattern(
        &self,
        _pattern: &str,
        _path: &str,
    ) -> Result<String> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::extract_path_within_pattern",
        ))
    }

    /// 对齐 Java: `AntPathMatcher::extractUriTemplateVariables#Map<String,String> (String pattern, String path)`
    pub fn extract_uri_template_variables(
        &self,
        _pattern: &str,
        _path: &str,
    ) -> Result<Vec<(String, String)>> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::extract_uri_template_variables",
        ))
    }

    /// 对齐 Java: `AntPathMatcher::combine#String (String pattern1, String pattern2)`
    pub fn combine(&self, _pattern1: &str, _pattern2: &str) -> Result<String> {
        Err(CoreError::PendingEngine("AntPathMatcher::combine"))
    }

    /// 对齐 Java: `AntPathMatcher::getPatternComparator#Comparator<String> (String path)`
    pub fn get_pattern_comparator(&self, _path: &str) -> Result<()> {
        Err(CoreError::PendingEngine(
            "AntPathMatcher::get_pattern_comparator",
        ))
    }
}

/// 对齐 Java: `AntPathMatcher#AntPathStringMatcher` 内部类
#[derive(Debug, Clone)]
pub struct AntPathStringMatcher;

impl AntPathStringMatcher {
    /// 对齐 Java: `AntPathStringMatcher(String pattern, boolean caseSensitive)`
    pub fn new(_pattern: &str, _case_sensitive: bool) -> Self {
        Self
    }

    /// 对齐 Java: `AntPathStringMatcher::matchStrings#boolean`
    pub fn match_strings(
        &self,
        _str: &str,
        _uri_vars: &mut Vec<(String, String)>,
    ) -> Result<bool> {
        Err(CoreError::PendingEngine(
            "AntPathStringMatcher::match_strings",
        ))
    }
}

/// 对齐 Java: `AntPathMatcher#AntPatternComparator` 内部类
#[derive(Debug, Clone)]
pub struct AntPatternComparator;

impl AntPatternComparator {
    /// 对齐 Java: `AntPatternComparator(String path)`
    pub fn new(_path: &str) -> Self {
        Self
    }

    /// 对齐 Java: `AntPatternComparator::compare#int`
    pub fn compare(&self, _p1: &str, _p2: &str) -> Result<i32> {
        Err(CoreError::PendingEngine("AntPatternComparator::compare"))
    }
}

/// 对齐 Java: `AntPathMatcher#PatternInfo` 内部类
#[derive(Debug, Clone)]
pub struct PatternInfo;

impl PatternInfo {
    /// 对齐 Java: `PatternInfo(String pattern)`
    pub fn new(_pattern: &str) -> Self {
        Self
    }

    /// 对齐 Java: `PatternInfo::getUriVars`
    pub fn get_uri_vars(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternInfo::get_uri_vars"))
    }

    /// 对齐 Java: `PatternInfo::getSingleWildcards`
    pub fn get_single_wildcards(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternInfo::get_single_wildcards"))
    }

    /// 对齐 Java: `PatternInfo::getDoubleWildcards`
    pub fn get_double_wildcards(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternInfo::get_double_wildcards"))
    }

    /// 对齐 Java: `PatternInfo::isLeastSpecific`
    pub fn is_least_specific(&self) -> Result<bool> {
        Err(CoreError::PendingEngine("PatternInfo::is_least_specific"))
    }

    /// 对齐 Java: `PatternInfo::isPrefixPattern`
    pub fn is_prefix_pattern(&self) -> Result<bool> {
        Err(CoreError::PendingEngine("PatternInfo::is_prefix_pattern"))
    }

    /// 对齐 Java: `PatternInfo::getTotalCount`
    pub fn get_total_count(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternInfo::get_total_count"))
    }

    /// 对齐 Java: `PatternInfo::getLength`
    pub fn get_length(&self) -> Result<i32> {
        Err(CoreError::PendingEngine("PatternInfo::get_length"))
    }
}

/// 对齐 Java: `AntPathMatcher#PathSeparatorPatternCache` 内部类
#[derive(Debug, Clone)]
pub struct PathSeparatorPatternCache;

impl PathSeparatorPatternCache {
    /// 对齐 Java: `PathSeparatorPatternCache(String pathSeparator)`
    pub fn new(_separator: &str) -> Self {
        Self
    }

    /// 对齐 Java: `PathSeparatorPatternCache::getEndsOnWildCard`
    pub fn get_ends_on_wild_card(&self) -> Result<String> {
        Err(CoreError::PendingEngine(
            "PathSeparatorPatternCache::get_ends_on_wild_card",
        ))
    }

    /// 对齐 Java: `PathSeparatorPatternCache::getEndsOnDoubleWildCard`
    pub fn get_ends_on_double_wild_card(&self) -> Result<String> {
        Err(CoreError::PendingEngine(
            "PathSeparatorPatternCache::get_ends_on_double_wild_card",
        ))
    }
}