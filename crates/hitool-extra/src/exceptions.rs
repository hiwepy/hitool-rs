//! 统一实现 hutool-extra 6 个 Exception 类型。
//!
//! 每个 hutool-extra Exception 都是同样的 6 个构造器模式：
//!   - `(Throwable e)` → 包装底层错误
//!   - `(String message)` → 简单消息
//!   - `(String messageTemplate, Object... params)` → 模板格式化
//!   - `(String message, Throwable throwable)` → 消息 + 错误
//!   - `(String message, Throwable throwable, boolean, boolean)` → 完整控制
//!   - `(Throwable throwable, String messageTemplate, Object... params)` → 模板 + 错误
//!
//! 使用宏 `define_hutool_exception!` 批量实现，避免重复代码。

use thiserror::Error;

/// 统一的模板格式化工具（参考 `cn.hutool.core.util.StrUtil.format`）。
/// Phase 1.4 完成后可委托到 `hitool_core::format_template`。
fn format_message(template: &str, params: &[&dyn std::fmt::Display]) -> String {
    let mut out = String::new();
    let mut idx = 0;
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'}') {
            chars.next();
            if idx < params.len() {
                use std::fmt::Write;
                let _ = write!(out, "{}", params[idx]);
                idx += 1;
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// 通用 hutool Exception 错误枚举，对齐 `cn.hutool.extra.X.XxxException` 的 6 构造器模式。
///
/// 各具体 Exception 类型（MailException、QrCodeException 等）都是这个 enum 的 re-export 别名。
#[derive(Debug, Error)]
pub enum HutoolException {
    /// 对齐 `(Throwable e)`
    #[error("{message}")]
    FromCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(String message)`
    #[error("{0}")]
    Message(String),

    /// 对齐 `(String messageTemplate, Object... params)`
    #[error("{message}")]
    Formatted { message: String },

    /// 对齐 `(String message, Throwable throwable)`
    #[error("{message}")]
    WithThrowable {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(String message, Throwable throwable, boolean, boolean)`
    /// Rust 没有 suppression/writableStackTrace 直接对应，使用 Box 包装
    #[error("{message}")]
    Full {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(Throwable throwable, String messageTemplate, Object... params)`
    #[error("{message}")]
    FormattedWithCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl HutoolException {
    /// 对齐 `XxxException(Throwable e)`
    pub fn from_cause<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::FromCause {
            message: e.to_string(),
            source: Box::new(e),
        }
    }

    /// 对齐 `XxxException(String message)`
    pub fn message<S: Into<String>>(msg: S) -> Self {
        HutoolException::Message(msg.into())
    }

    /// 对齐 `XxxException(String messageTemplate, Object... params)`
    pub fn formatted<S: AsRef<str>>(template: S, params: &[&dyn std::fmt::Display]) -> Self {
        HutoolException::Formatted {
            message: format_message(template.as_ref(), params),
        }
    }

    /// 对齐 `XxxException(String message, Throwable throwable)`
    pub fn with_throwable<S: Into<String>, E>(message: S, throwable: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::WithThrowable {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `XxxException(String message, Throwable throwable, boolean, boolean)`
    pub fn full<S: Into<String>, E>(message: S, throwable: E, _suppression: bool, _writable: bool) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::Full {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `XxxException(Throwable throwable, String messageTemplate, Object... params)`
    pub fn formatted_with_cause<E>(
        throwable: E,
        template: &str,
        params: &[&dyn std::fmt::Display],
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::FormattedWithCause {
            message: format_message(template, params),
            source: Box::new(throwable),
        }
    }

    /// 获取消息字符串（对齐 `Throwable.getMessage()`）
    pub fn get_message(&self) -> &str {
        match self {
            HutoolException::FromCause { message, .. }
            | HutoolException::Formatted { message }
            | HutoolException::WithThrowable { message, .. }
            | HutoolException::Full { message, .. }
            | HutoolException::FormattedWithCause { message, .. } => message,
            HutoolException::Message(m) => m,
        }
    }
}

// ===== 类型别名：每个 hutool-extra Exception 都是 HutoolException 的语义别名 =====
//
// 注意：PinyinException 在 pinyin.rs 中已有独立实现（保持向后兼容）。
// 其余 5 个统一指向 HutoolException。

/// 对齐 `cn.hutool.extra.mail.MailException`
pub type MailException = HutoolException;

/// 对齐 `cn.hutool.extra.qrcode.QrCodeException`
pub type QrCodeException = HutoolException;

// PinyinException 由 pinyin.rs 提供（已有独立 struct）
// pub type PinyinException = HutoolException;

/// 对齐 `cn.hutool.extra.compress.CompressException`
pub type CompressException = HutoolException;

/// 对齐 `cn.hutool.extra.expression.ExpressionException`
pub type ExpressionException = HutoolException;

/// 对齐 `cn.hutool.extra.tokenizer.TokenizerException`
pub type TokenizerException = HutoolException;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_constructor() {
        let e = HutoolException::message("test error");
        assert_eq!(e.get_message(), "test error");
    }

    #[test]
    fn test_formatted_constructor() {
        let e = HutoolException::formatted("hello {} world", &[&"rust"]);
        assert_eq!(e.get_message(), "hello rust world");
    }

    #[test]
    fn test_formatted_multiple_placeholders() {
        let e = HutoolException::formatted("{} + {} = {}", &[&1i32, &2i32, &3i32]);
        assert_eq!(e.get_message(), "1 + 2 = 3");
    }

    #[test]
    fn test_from_cause() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing.txt");
        let e = HutoolException::from_cause(io_err);
        assert!(e.get_message().contains("missing.txt"));
    }

    #[test]
    fn test_with_throwable() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let e = HutoolException::with_throwable("operation failed", io_err);
        assert_eq!(e.get_message(), "operation failed");
    }

    #[test]
    fn test_full_constructor() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "full");
        let e = HutoolException::full("msg", io_err, true, true);
        assert_eq!(e.get_message(), "msg");
    }

    #[test]
    fn test_formatted_with_cause() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io");
        let e = HutoolException::formatted_with_cause(io_err, "failed: {}", &[&"x"]);
        assert_eq!(e.get_message(), "failed: x");
    }

    #[test]
    fn test_exception_aliases() {
        // 验证类型别名可作为 Exception 使用
        let mail: MailException = HutoolException::message("mail failure");
        assert_eq!(mail.get_message(), "mail failure");

        let qr: QrCodeException = HutoolException::formatted("QR: {}", &[&"bad data"]);
        assert_eq!(qr.get_message(), "QR: bad data");

        let compress: CompressException = HutoolException::message("compress");
        let expression: ExpressionException = HutoolException::message("expression");
        let tokenizer: TokenizerException = HutoolException::message("tokenizer");

        // PinyinException 在 pinyin.rs 中有独立实现，跳过此处
        assert_eq!(compress.get_message(), "compress");
        assert_eq!(expression.get_message(), "expression");
        assert_eq!(tokenizer.get_message(), "tokenizer");
    }
}