//! 对齐: `cn.hutool.core.exceptions.DependencyException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/DependencyException.java

use crate::string::format_template;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.exceptions.DependencyException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct DependencyException {
    /// 错误消息。
    pub message: String,
    /// 可选原因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl DependencyException {
    /// 对齐 Java: `DependencyException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `DependencyException(String, Object...)`
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `DependencyException(Throwable)`
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `DependencyException(String, Throwable)`
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `DependencyException(Throwable, String, Object...)`
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
