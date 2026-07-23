//! 对齐: `cn.hutool.core.comparator.ComparatorException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/ComparatorException.java

use crate::string::format_template;
use std::fmt::Display;

/// 对齐 Java 异常类: `cn.hutool.core.comparator.ComparatorException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ComparatorException {
    /// 错误消息。
    pub message: String,
    /// 可选原因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ComparatorException {
    /// 对齐 Java: `ComparatorException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `ComparatorException(Throwable)`
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `ComparatorException(String, Object...)`
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `ComparatorException(String, Throwable)`
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `ComparatorException(Throwable, String, Object...)`
    pub fn with_cause_template(
        cause: impl std::error::Error + Send + Sync + 'static,
        template: &str,
        params: &[&dyn Display],
    ) -> Self {
        Self {
            message: format_template(template, params),
            source: Some(Box::new(cause)),
        }
    }
}
