//! 对齐: `cn.hutool.core.exceptions.InvocationTargetRuntimeException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/InvocationTargetRuntimeException.java

use crate::string::format_template;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.exceptions.InvocationTargetRuntimeException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct InvocationTargetRuntimeException {
    /// 错误消息。
    pub message: String,
    /// 可选原因（目标调用异常）。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl InvocationTargetRuntimeException {
    /// 对齐 Java: `InvocationTargetRuntimeException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `InvocationTargetRuntimeException(String, Object...)`
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `InvocationTargetRuntimeException(Throwable)`
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `InvocationTargetRuntimeException(String, Throwable)`
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `InvocationTargetRuntimeException(Throwable, String, Object...)`
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
