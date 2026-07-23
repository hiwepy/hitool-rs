//! 对齐: `cn.hutool.core.thread.ThreadException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadException.java

/// 对齐 Java 异常类: `cn.hutool.core.thread.ThreadException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ThreadException {
    /// 错误消息。
    pub message: String,
    /// 可选根因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ThreadException {
    /// 对齐 `ThreadException(String message)`。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 `ThreadException(Throwable e)`。
    pub fn from_error(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }

    /// 对齐 `ThreadException(String message, Throwable throwable)`。
    pub fn with_cause(
        message: impl Into<String>,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(err)),
        }
    }

    /// 对齐 `ThreadException(String messageTemplate, Object... params)`。
    pub fn template(message: impl Into<String>) -> Self {
        Self::new(message)
    }
}
