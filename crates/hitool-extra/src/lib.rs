//! Extended utilities that do not belong in the core dependency surface.

#![forbid(unsafe_code)]

#[cfg(feature = "archive")]
pub mod archive;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "mail")]
pub mod mail;
#[cfg(feature = "qrcode")]
pub mod qrcode;

/// Errors returned by extra utilities.
#[derive(Debug, thiserror::Error)]
pub enum ExtraError {
    /// QR input could not be encoded.
    #[error(transparent)]
    #[cfg(feature = "qrcode")]
    Qr(#[from] ::qrcode::types::QrError),
    /// ZIP structure is malformed or unsupported.
    #[error(transparent)]
    #[cfg(feature = "archive")]
    Zip(#[from] zip::result::ZipError),
    /// Filesystem I/O failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An archive entry attempted to escape the destination root.
    #[error("unsafe archive path: {0}")]
    UnsafeArchivePath(String),
    /// Symbolic links are not extracted.
    #[error("symbolic link archive entry is not allowed: {0}")]
    SymlinkEntry(String),
    /// Archive resource limits were exceeded.
    #[error("archive limit exceeded: {0}")]
    ArchiveLimit(&'static str),
    /// Image decoding or encoding failed.
    #[cfg(feature = "image")]
    #[error(transparent)]
    Image(#[from] ::image::ImageError),
    /// Image input or decoded dimensions exceeded configured limits.
    #[cfg(feature = "image")]
    #[error("image limit exceeded: {0}")]
    ImageLimit(&'static str),
    /// SMTP transport construction or delivery failed.
    #[cfg(feature = "mail")]
    #[error(transparent)]
    Smtp(#[from] lettre::transport::smtp::Error),
    /// MIME message construction failed.
    #[cfg(feature = "mail")]
    #[error(transparent)]
    MailMessage(#[from] lettre::error::Error),
    /// Mail configuration or message content is invalid.
    #[cfg(feature = "mail")]
    #[error("invalid mail configuration: {0}")]
    InvalidMail(&'static str),
    /// Mail attachments exceeded configured limits.
    #[cfg(feature = "mail")]
    #[error("mail attachment limit exceeded: {0}")]
    MailAttachmentLimit(&'static str),
}

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, ExtraError>;
