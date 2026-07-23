//! 对齐: `cn.hutool.core.convert.ConvertException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/ConvertException.java

#![allow(dead_code)]

/// 对齐 Java 异常类: `cn.hutool.core.convert.ConvertException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ConvertException {
    /// 错误消息。
    pub message: String,
}

impl ConvertException {
    /// 创建新的错误实例。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
