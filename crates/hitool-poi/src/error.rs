//! Tabular document errors aligned with Hutool's POI module.
//!
//! 对齐: `cn.hutool.poi.exceptions.POIException`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/exceptions/POIException.java
//!
//! Hutool 通过单一 `POIException` 包装所有 POI 操纵失败。Rust 版本将其拆分为
//! 非穷举 `PoiError` 枚举,以便调用者按错误类别精确处理。

use thiserror::Error;

/// Tabular document errors.
///
/// 对齐 Java: `cn.hutool.poi.exceptions.POIException`
/// 非穷举枚举(non-exhaustive)允许未来按需新增变体而不破坏调用方。
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PoiError {
    /// XLSX creation failed.
    #[cfg(feature = "xlsx")]
    #[error(transparent)]
    XlsxWrite(#[from] rust_xlsxwriter::XlsxError),
    /// CSV serialization or parsing failed.
    #[cfg(feature = "csv")]
    #[error(transparent)]
    Csv(#[from] csv::Error),
    /// In-memory writer finalization failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// XLSX ZIP container processing failed.
    #[cfg(any(feature = "docx", feature = "xlsx"))]
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    /// XLSX XML content is malformed or unsupported.
    #[cfg(feature = "xlsx")]
    #[error("malformed XLSX: {0}")]
    MalformedXlsx(String),
    /// A document exceeded configured resource limits.
    #[error("document resource limit exceeded: {0}")]
    ResourceLimit(&'static str),
    /// A row or column index cannot be represented by XLSX.
    #[cfg(feature = "xlsx")]
    #[error("table dimensions exceed XLSX limits")]
    TableTooLarge,
    /// CSV output was not valid UTF-8.
    #[cfg(feature = "csv")]
    #[error("CSV output was not valid UTF-8")]
    InvalidUtf8,
    /// The requested engine capability is not yet wired up.
    ///
    /// 对齐 Java: 等待 easyexcel-rs / easydoc-rs / easyofd-rs / easypdf-rs 完成
    /// 时抛出的标记性错误。每个尚未实现的方法体均通过该错误表达"已对齐签名,
    /// 等待引擎"的状态,避免与运行期故障混淆。
    #[error("POI engine not implemented yet, waiting for {0}")]
    PendingEngine(&'static str),
}

/// Result type for tabular document operations.
///
/// 对齐 Java: Hutool 各 POI 方法在签名中抛出 `POIException`,
/// Rust 版本统一通过 `Result<T, PoiError>` 表达可失败的计算。
pub type Result<T> = std::result::Result<T, PoiError>;
