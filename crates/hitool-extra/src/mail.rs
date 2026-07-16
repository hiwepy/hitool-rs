//! Explicit, injectable SMTP and MIME mail support.

use std::{fmt, time::Duration};

pub use lettre::message::Mailbox;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Attachment, MultiPart, SinglePart, header::ContentType},
    transport::smtp::{authentication::Credentials, response::Response},
};
use secrecy::{ExposeSecret, SecretString};

use crate::{ExtraError, Result};

/// SMTP channel security mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// TLS from connection establishment, normally port 465.
    Tls,
    /// Mandatory STARTTLS upgrade, normally port 587.
    StartTls,
    /// Unencrypted SMTP. Intended only for explicitly trusted local relays.
    Plaintext,
}

/// SMTP username and password stored in redacted containers.
pub struct SmtpCredentials {
    username: SecretString,
    password: SecretString,
}

impl SmtpCredentials {
    /// Creates SMTP credentials.
    #[must_use]
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: SecretString::from(username.into()),
            password: SecretString::from(password.into()),
        }
    }
}

impl fmt::Debug for SmtpCredentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmtpCredentials")
            .field("username", &"[REDACTED]")
            .field("password", &"[REDACTED]")
            .finish()
    }
}

/// Complete SMTP connection configuration.
pub struct SmtpConfig {
    /// SMTP relay hostname.
    pub host: String,
    /// SMTP relay port.
    pub port: u16,
    /// Required channel security mode.
    pub security: SmtpSecurity,
    /// Optional authentication credentials.
    pub credentials: Option<SmtpCredentials>,
    /// Per-operation timeout.
    pub timeout: Duration,
}

impl fmt::Debug for SmtpConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SmtpConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("security", &self.security)
            .field(
                "credentials",
                &self.credentials.as_ref().map(|_| "[REDACTED]"),
            )
            .field("timeout", &self.timeout)
            .finish()
    }
}

/// Defensive MIME construction limits.
#[derive(Debug, Clone, Copy)]
pub struct MailLimits {
    /// Maximum total To/Cc/Bcc recipients.
    pub max_recipients: usize,
    /// Maximum UTF-8 body bytes.
    pub max_body_bytes: usize,
    /// Maximum attachment count.
    pub max_attachments: usize,
    /// Maximum bytes in one attachment.
    pub max_attachment_bytes: usize,
    /// Maximum bytes across all attachments.
    pub max_total_attachment_bytes: usize,
}

impl Default for MailLimits {
    fn default() -> Self {
        Self {
            max_recipients: 100,
            max_body_bytes: 2 * 1024 * 1024,
            max_attachments: 32,
            max_attachment_bytes: 25 * 1024 * 1024,
            max_total_attachment_bytes: 50 * 1024 * 1024,
        }
    }
}

/// Plain-text or HTML message body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailBody {
    /// UTF-8 plain text.
    Text(String),
    /// UTF-8 HTML. Content sanitization remains the caller's responsibility.
    Html(String),
}

/// Owned MIME attachment or inline resource.
#[derive(Debug, Clone)]
pub struct MailAttachment {
    name_or_cid: String,
    content_type: ContentType,
    bytes: Vec<u8>,
    inline: bool,
}

impl MailAttachment {
    /// Creates a downloadable attachment.
    pub fn file(
        filename: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        Self::new(filename.into(), content_type, bytes.into(), false)
    }

    /// Creates an inline resource referenced with `cid:<content_id>`.
    pub fn inline(
        content_id: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        Self::new(content_id.into(), content_type, bytes.into(), true)
    }

    fn new(name_or_cid: String, content_type: &str, bytes: Vec<u8>, inline: bool) -> Result<Self> {
        if name_or_cid.trim().is_empty() {
            return Err(ExtraError::InvalidMail(
                "attachment name/content id is empty",
            ));
        }
        let content_type = ContentType::parse(content_type)
            .map_err(|_| ExtraError::InvalidMail("attachment content type is invalid"))?;
        Ok(Self {
            name_or_cid,
            content_type,
            bytes,
            inline,
        })
    }
}

/// Builder-style mail value with no global account or hidden session.
#[derive(Debug, Clone)]
pub struct MailMessage {
    from: Mailbox,
    to: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply_to: Vec<Mailbox>,
    subject: String,
    body: MailBody,
    attachments: Vec<MailAttachment>,
}

impl MailMessage {
    /// Creates a plain-text message.
    #[must_use]
    pub fn text(from: Mailbox, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(from, subject.into(), MailBody::Text(body.into()))
    }

    /// Creates an HTML message.
    #[must_use]
    pub fn html(from: Mailbox, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(from, subject.into(), MailBody::Html(body.into()))
    }

    fn new(from: Mailbox, subject: String, body: MailBody) -> Self {
        Self {
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply_to: Vec::new(),
            subject,
            body,
            attachments: Vec::new(),
        }
    }

    /// Adds a primary recipient.
    #[must_use]
    pub fn to(mut self, recipient: Mailbox) -> Self {
        self.to.push(recipient);
        self
    }

    /// Adds a carbon-copy recipient.
    #[must_use]
    pub fn cc(mut self, recipient: Mailbox) -> Self {
        self.cc.push(recipient);
        self
    }

    /// Adds a blind-carbon-copy recipient.
    #[must_use]
    pub fn bcc(mut self, recipient: Mailbox) -> Self {
        self.bcc.push(recipient);
        self
    }

    /// Adds a reply-to address.
    #[must_use]
    pub fn reply_to(mut self, recipient: Mailbox) -> Self {
        self.reply_to.push(recipient);
        self
    }

    /// Adds an attachment or inline resource.
    #[must_use]
    pub fn attachment(mut self, attachment: MailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Builds a standards-compliant MIME message after enforcing limits.
    pub fn build(&self, limits: MailLimits) -> Result<Message> {
        let recipient_count = self
            .to
            .len()
            .saturating_add(self.cc.len())
            .saturating_add(self.bcc.len());
        if recipient_count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        if recipient_count > limits.max_recipients {
            return Err(ExtraError::InvalidMail("recipient limit exceeded"));
        }
        let body = match &self.body {
            MailBody::Text(body) | MailBody::Html(body) => body,
        };
        if body.len() > limits.max_body_bytes {
            return Err(ExtraError::InvalidMail("body byte limit exceeded"));
        }
        if self.attachments.len() > limits.max_attachments {
            return Err(ExtraError::MailAttachmentLimit("attachment count"));
        }
        let total = self
            .attachments
            .iter()
            .try_fold(0_usize, |total, attachment| {
                if attachment.bytes.len() > limits.max_attachment_bytes {
                    return Err(ExtraError::MailAttachmentLimit("single attachment bytes"));
                }
                total
                    .checked_add(attachment.bytes.len())
                    .ok_or(ExtraError::MailAttachmentLimit("total attachment bytes"))
            })?;
        if total > limits.max_total_attachment_bytes {
            return Err(ExtraError::MailAttachmentLimit("total attachment bytes"));
        }

        let mut builder = Message::builder()
            .from(self.from.clone())
            .subject(&self.subject);
        for recipient in &self.to {
            builder = builder.to(recipient.clone());
        }
        for recipient in &self.cc {
            builder = builder.cc(recipient.clone());
        }
        for recipient in &self.bcc {
            builder = builder.bcc(recipient.clone());
        }
        for recipient in &self.reply_to {
            builder = builder.reply_to(recipient.clone());
        }

        let body_part = match &self.body {
            MailBody::Text(body) => SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(body.clone()),
            MailBody::Html(body) => SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(body.clone()),
        };
        let mut multipart = MultiPart::mixed().singlepart(body_part);
        for attachment in &self.attachments {
            let part = if attachment.inline {
                Attachment::new_inline(attachment.name_or_cid.clone())
            } else {
                Attachment::new(attachment.name_or_cid.clone())
            }
            .body(attachment.bytes.clone(), attachment.content_type.clone());
            multipart = multipart.singlepart(part);
        }
        Ok(builder.multipart(multipart)?)
    }
}

/// Cloneable asynchronous SMTP client backed by Lettre and Rustls.
#[derive(Clone)]
pub struct SmtpClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl fmt::Debug for SmtpClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("SmtpClient").finish_non_exhaustive()
    }
}

impl SmtpClient {
    /// Builds a reusable SMTP connection pool.
    pub fn new(config: &SmtpConfig) -> Result<Self> {
        if config.host.trim().is_empty() || config.port == 0 || config.timeout.is_zero() {
            return Err(ExtraError::InvalidMail(
                "host, port and timeout are required",
            ));
        }
        let mut builder = match config.security {
            SmtpSecurity::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?,
            SmtpSecurity::StartTls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)?
            }
            SmtpSecurity::Plaintext => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
            }
        }
        .port(config.port)
        .timeout(Some(config.timeout));
        if let Some(credentials) = &config.credentials {
            builder = builder.credentials(Credentials::new(
                credentials.username.expose_secret().to_owned(),
                credentials.password.expose_secret().to_owned(),
            ));
        }
        Ok(Self {
            transport: builder.build(),
        })
    }

    /// Sends a previously bounded and built MIME message.
    pub async fn send(&self, message: Message) -> Result<Response> {
        Ok(self.transport.send(message).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mailbox(value: &str) -> Mailbox {
        value.parse().unwrap()
    }

    #[test]
    fn builds_text_html_attachment_and_inline_mime() {
        let message = MailMessage::html(
            mailbox("sender@example.com"),
            "HiTool mail",
            "<img src=\"cid:logo\">",
        )
        .to(mailbox("to@example.com"))
        .cc(mailbox("cc@example.com"))
        .reply_to(mailbox("reply@example.com"))
        .attachment(MailAttachment::inline("logo", "image/png", b"png".to_vec()).unwrap())
        .attachment(MailAttachment::file("report.txt", "text/plain", b"hello".to_vec()).unwrap())
        .build(MailLimits::default())
        .unwrap();
        let formatted = String::from_utf8(message.formatted()).unwrap();
        assert!(formatted.contains("Subject: HiTool mail"));
        assert!(formatted.contains("Content-ID: <logo>"));
        assert!(formatted.contains("filename=\"report.txt\""));
    }

    #[test]
    fn rejects_missing_recipient_and_attachment_limits() {
        let message = MailMessage::text(mailbox("sender@example.com"), "subject", "body");
        assert!(matches!(
            message.build(MailLimits::default()),
            Err(ExtraError::InvalidMail(
                "at least one recipient is required"
            ))
        ));
        let message = message
            .to(mailbox("to@example.com"))
            .attachment(MailAttachment::file("x", "text/plain", vec![0; 2]).unwrap());
        let limits = MailLimits {
            max_attachment_bytes: 1,
            ..MailLimits::default()
        };
        assert!(matches!(
            message.build(limits),
            Err(ExtraError::MailAttachmentLimit("single attachment bytes"))
        ));
    }

    #[test]
    fn debug_never_exposes_smtp_credentials() {
        let config = SmtpConfig {
            host: "smtp.example.com".into(),
            port: 465,
            security: SmtpSecurity::Tls,
            credentials: Some(SmtpCredentials::new("private-user", "private-password")),
            timeout: Duration::from_secs(10),
        };
        let debug = format!("{config:?}");
        assert!(!debug.contains("private-user"));
        assert!(!debug.contains("private-password"));
        assert!(debug.contains("[REDACTED]"));
    }
}
