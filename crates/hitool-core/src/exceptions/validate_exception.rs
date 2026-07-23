//! 对齐: `cn.hutool.core.exceptions.ValidateException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/ValidateException.java

use super::stateful_exception::StatefulException;
use crate::string::format_template;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.exceptions.ValidateException`（继承 StatefulException）。
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ValidateException {
    #[from]
    inner: StatefulException,
}

impl ValidateException {
    /// 对齐 Java: `ValidateException()`
    pub fn empty() -> Self {
        Self {
            inner: StatefulException::empty(),
        }
    }

    /// 对齐 Java: `ValidateException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            inner: StatefulException::new(message),
        }
    }

    /// 对齐 Java: `ValidateException(String, Object...)`
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `ValidateException(Throwable)`
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            inner: StatefulException::from_cause(cause),
        }
    }

    /// 对齐 Java: `ValidateException(String, Throwable)`
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            inner: StatefulException::with_cause(message, cause),
        }
    }

    /// 对齐 Java: `ValidateException(int, String)`
    pub fn with_status(status: i32, message: impl Into<String>) -> Self {
        Self {
            inner: StatefulException::with_status(status, message),
        }
    }

    /// 对齐 Java: `ValidateException(int, Throwable)`
    pub fn with_status_cause(
        status: i32,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            inner: StatefulException::with_status_cause(status, cause),
        }
    }

    /// 对齐 Java: `ValidateException(int, String, Throwable)`
    pub fn with_status_message_cause(
        status: i32,
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            inner: StatefulException::with_status_message_cause(status, message, cause),
        }
    }

    /// 对齐 Java: `getStatus()`
    pub fn get_status(&self) -> i32 {
        self.inner.get_status()
    }

    /// 访问内部 StatefulException。
    pub fn inner(&self) -> &StatefulException {
        &self.inner
    }
}
