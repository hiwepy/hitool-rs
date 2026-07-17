//! POI exception placeholder aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.exceptions.POIException`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/exceptions/POIException.java
//!
//! Hutool 的 `POIException` 继承自 `cn.hutool.core.exceptions.ExceptionUtil`,
//! 用于包裹 Apache POI 抛出的 checked exception。Rust 版本通过 `PoiError`
//! 枚举(`error.rs`)表达所有 POI 故障,本文件仅保留运行时异常类型的命名对齐,
//! 以便调用方在错误消息中匹配原文。

use std::fmt;

/// Runtime POI exception placeholder.
///
/// 对齐 Java: `cn.hutool.poi.exceptions.POIException`
/// 类型: `RuntimeException` 子类
#[derive(Debug, Clone)]
pub struct PoiException {
    /// 原始 Java 异常类名,用于日志与跨语言诊断。
    pub origin: &'static str,
    /// 异常消息。
    pub message: String,
}

impl PoiException {
    /// Creates a new exception with a Java-origin tag and message.
    pub fn new(origin: &'static str, message: impl Into<String>) -> Self {
        Self {
            origin,
            message: message.into(),
        }
    }
}

impl fmt::Display for PoiException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.origin, self.message)
    }
}

impl std::error::Error for PoiException {}

/// Stop reading exception used by SAX readers.
///
/// 对齐 Java: `cn.hutool.poi.excel.sax.StopReadException`
/// 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/sax/StopReadException.java
/// Hutool 用它来提前终止 SAX 读取流程;Rust 版本通常改用 `Result` 短路,
/// 本桩保留类型命名以便未来 easyexcel-rs 接入时直接复用。
#[derive(Debug, Clone, Copy)]
pub struct StopReadException;

impl fmt::Display for StopReadException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("StopReadException")
    }
}

impl std::error::Error for StopReadException {}