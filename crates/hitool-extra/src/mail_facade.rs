//! Hutool-named mail facades over injectable SMTP/MIME helpers.
//!
//! 对齐: `cn.hutool.extra.mail.MailUtil`
//! 对齐: `cn.hutool.extra.mail.Mail`
//! 对齐: `cn.hutool.extra.mail.MailAccount`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/mail/

use std::time::Duration;

use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::response::Response;

use crate::mail::{
    MailAttachment, MailBody, MailLimits, MailMessage, SmtpClient, SmtpConfig, SmtpCredentials,
    SmtpSecurity,
};
use crate::{ExtraError, Result};

/// Hutool `MailAccount` — SMTP relay settings without a global singleton.
///
/// 对齐 Java 类: `cn.hutool.extra.mail.MailAccount`
#[derive(Debug)]
pub struct MailAccount {
    inner: SmtpConfig,
    from: Option<Mailbox>,
}

impl MailAccount {
    /// Creates an empty account (Hutool no-arg constructor defaults filled later).
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: SmtpConfig {
                host: String::new(),
                port: 465,
                security: SmtpSecurity::Tls,
                credentials: None,
                timeout: Duration::from_secs(30),
            },
            from: None,
        }
    }

    /// Creates an account from an existing SMTP config.
    #[must_use]
    pub fn from_config(config: SmtpConfig) -> Self {
        Self {
            inner: config,
            from: None,
        }
    }

    /// Sets SMTP host (Hutool `setHost`).
    #[must_use]
    pub fn set_host(mut self, host: impl Into<String>) -> Self {
        self.inner.host = host.into();
        self
    }

    /// Returns SMTP host (Hutool `getHost`).
    #[must_use]
    pub fn host(&self) -> &str {
        &self.inner.host
    }

    /// Sets SMTP port (Hutool `setPort`).
    #[must_use]
    pub fn set_port(mut self, port: u16) -> Self {
        self.inner.port = port;
        self
    }

    /// Returns SMTP port (Hutool `getPort`).
    #[must_use]
    pub fn port(&self) -> u16 {
        self.inner.port
    }

    /// Enables TLS / STARTTLS style security (Hutool `setSslEnable` / `setStarttlsEnable`).
    #[must_use]
    pub fn set_ssl_enable(mut self, enable: bool) -> Self {
        self.inner.security = if enable {
            SmtpSecurity::Tls
        } else {
            SmtpSecurity::Plaintext
        };
        self
    }

    /// Forces STARTTLS (Hutool `setStarttlsEnable(true)`).
    #[must_use]
    pub fn set_starttls_enable(mut self, enable: bool) -> Self {
        if enable {
            self.inner.security = SmtpSecurity::StartTls;
        }
        self
    }

    /// Sets username/password (Hutool `setUser` / `setPass` / `setAuth`).
    #[must_use]
    pub fn set_auth(mut self, user: impl Into<String>, pass: impl Into<String>) -> Self {
        self.inner.credentials = Some(SmtpCredentials::new(user, pass));
        self
    }

    /// Sets the From mailbox used by [`Mail`] builders.
    #[must_use]
    pub fn set_from(mut self, from: Mailbox) -> Self {
        self.from = Some(from);
        self
    }

    /// Returns the configured From mailbox when set.
    #[must_use]
    pub fn from(&self) -> Option<&Mailbox> {
        self.from.as_ref()
    }

    /// Borrows the underlying SMTP configuration.
    #[must_use]
    pub fn smtp_config(&self) -> &SmtpConfig {
        &self.inner
    }

    /// Builds an injectable [`SmtpClient`] (Hutool session factory without globals).
    pub fn create_client(&self) -> Result<SmtpClient> {
        SmtpClient::new(&self.inner)
    }
}

impl Default for MailAccount {
    fn default() -> Self {
        Self::new()
    }
}

/// Hutool `Mail` builder — delegates to [`MailMessage`] (no global session).
///
/// 对齐 Java 类: `cn.hutool.extra.mail.Mail`
#[derive(Debug)]
pub struct Mail {
    account: Option<MailAccount>,
    from: Option<Mailbox>,
    to: Vec<Mailbox>,
    cc: Vec<Mailbox>,
    bcc: Vec<Mailbox>,
    reply: Vec<Mailbox>,
    title: String,
    content: String,
    html: bool,
    attachments: Vec<MailAttachment>,
}

impl Mail {
    /// Creates a mail builder with an account (Hutool `Mail.create(account)`).
    #[must_use]
    pub fn create(account: MailAccount) -> Self {
        let from = account.from().cloned();
        Self {
            account: Some(account),
            from,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply: Vec::new(),
            title: String::new(),
            content: String::new(),
            html: false,
            attachments: Vec::new(),
        }
    }

    /// Creates a mail builder without an account (Hutool `Mail.create()`).
    #[must_use]
    pub fn create_empty() -> Self {
        Self {
            account: None,
            from: None,
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            reply: Vec::new(),
            title: String::new(),
            content: String::new(),
            html: false,
            attachments: Vec::new(),
        }
    }

    /// Sets primary recipients (Hutool `to` / `setTos`).
    #[must_use]
    pub fn to(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.to = recipients.into_iter().collect();
        self
    }

    /// Sets CC recipients (Hutool `setCcs`).
    #[must_use]
    pub fn set_ccs(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.cc = recipients.into_iter().collect();
        self
    }

    /// Sets BCC recipients (Hutool `setBccs`).
    #[must_use]
    pub fn set_bccs(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.bcc = recipients.into_iter().collect();
        self
    }

    /// Sets Reply-To addresses (Hutool `setReply`).
    #[must_use]
    pub fn set_reply(mut self, recipients: impl IntoIterator<Item = Mailbox>) -> Self {
        self.reply = recipients.into_iter().collect();
        self
    }

    /// Sets the From mailbox when not taken from [`MailAccount`].
    #[must_use]
    pub fn set_from(mut self, from: Mailbox) -> Self {
        self.from = Some(from);
        self
    }

    /// Sets the subject (Hutool `setTitle`).
    #[must_use]
    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets body text and HTML flag (Hutool `setContent` / `setHtml`).
    #[must_use]
    pub fn set_content(mut self, content: impl Into<String>, is_html: bool) -> Self {
        self.content = content.into();
        self.html = is_html;
        self
    }

    /// Marks the body as HTML (Hutool `setHtml`).
    #[must_use]
    pub fn set_html(mut self, is_html: bool) -> Self {
        self.html = is_html;
        self
    }

    /// Adds a file attachment (Hutool `setFiles` / `setAttachments` byte path).
    #[must_use]
    pub fn add_attachment(mut self, attachment: MailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    /// Adds an inline CID image (Hutool `addImage`).
    #[must_use]
    pub fn add_image(
        mut self,
        cid: impl Into<String>,
        content_type: &str,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self> {
        self.attachments
            .push(MailAttachment::inline(cid, content_type, bytes)?);
        Ok(self)
    }

    /// Builds a [`MailMessage`] after validating From/To.
    pub fn to_message(&self) -> Result<MailMessage> {
        let from = self
            .from
            .clone()
            .ok_or(ExtraError::InvalidMail("from address is required"))?;
        let body = if self.html {
            MailBody::Html(self.content.clone())
        } else {
            MailBody::Text(self.content.clone())
        };
        let mut message = match body {
            MailBody::Text(text) => MailMessage::text(from, self.title.clone(), text),
            MailBody::Html(html) => MailMessage::html(from, self.title.clone(), html),
        };
        for recipient in &self.to {
            message = message.to(recipient.clone());
        }
        for recipient in &self.cc {
            message = message.cc(recipient.clone());
        }
        for recipient in &self.bcc {
            message = message.bcc(recipient.clone());
        }
        for recipient in &self.reply {
            message = message.reply_to(recipient.clone());
        }
        for attachment in &self.attachments {
            message = message.attachment(attachment.clone());
        }
        Ok(message)
    }

    /// Builds a MIME [`Message`] (Hutool `build`).
    pub fn build(&self) -> Result<Message> {
        self.to_message()?.build(MailLimits::default())
    }

    /// Sends via the account's client (Hutool `send` without a global session).
    pub async fn send(&self) -> Result<Response> {
        let account = self
            .account
            .as_ref()
            .ok_or(ExtraError::InvalidMail("mail account is required to send"))?;
        let client = account.create_client()?;
        client.send(self.build()?).await
    }
}

/// Hutool `MailUtil` static helpers — build/send without global `GlobalMailAccount`.
///
/// 对齐 Java 类: `cn.hutool.extra.mail.MailUtil`
pub struct MailUtil;

impl MailUtil {
    /// Builds a plain-text [`MailMessage`] (Hutool `sendText` body path, no global SMTP).
    pub fn text(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MailMessage> {
        let mut message = MailMessage::text(from, subject, content);
        let mut count = 0_usize;
        for recipient in tos {
            message = message.to(recipient);
            count += 1;
        }
        if count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        Ok(message)
    }

    /// Builds an HTML [`MailMessage`] (Hutool `sendHtml` body path).
    pub fn html(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<MailMessage> {
        let mut message = MailMessage::html(from, subject, content);
        let mut count = 0_usize;
        for recipient in tos {
            message = message.to(recipient);
            count += 1;
        }
        if count == 0 {
            return Err(ExtraError::InvalidMail(
                "at least one recipient is required",
            ));
        }
        Ok(message)
    }

    /// Builds text or HTML from a boolean flag (Hutool `send(..., isHtml, ...)`).
    pub fn message(
        from: Mailbox,
        tos: impl IntoIterator<Item = Mailbox>,
        subject: impl Into<String>,
        content: impl Into<String>,
        is_html: bool,
    ) -> Result<MailMessage> {
        if is_html {
            Self::html(from, tos, subject, content)
        } else {
            Self::text(from, tos, subject, content)
        }
    }

    /// Sends a previously built message through an injected client (Hutool `send` transport).
    pub async fn send(client: &SmtpClient, message: &MailMessage) -> Result<Response> {
        client.send(message.build(MailLimits::default())?).await
    }

    /// Builds then sends using a [`MailAccount`] (Hutool `send(MailAccount, ...)`).
    pub async fn send_with_account(
        account: &MailAccount,
        message: &MailMessage,
    ) -> Result<Response> {
        let client = account.create_client()?;
        Self::send(&client, message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mb(value: &str) -> Mailbox {
        value.parse().unwrap()
    }

    #[test]
    fn mail_util_builds_text_and_html() {
        let text = MailUtil::text(
            mb("a@b.c"),
            [mb("t@b.c")],
            "subj",
            "hello",
        )
        .unwrap()
        .build(MailLimits::default())
        .unwrap();
        assert!(String::from_utf8(text.formatted()).unwrap().contains("hello"));
        let html = MailUtil::html(mb("a@b.c"), [mb("t@b.c")], "h", "<b>x</b>")
            .unwrap()
            .build(MailLimits::default())
            .unwrap();
        assert!(String::from_utf8(html.formatted())
            .unwrap()
            .to_lowercase()
            .contains("text/html"));
    }

    #[test]
    fn mail_builder_matches_hutool_flow() {
        let account = MailAccount::new()
            .set_host("smtp.example.com")
            .set_port(465)
            .set_ssl_enable(true)
            .set_auth("u", "p")
            .set_from(mb("from@example.com"));
        let mime = Mail::create(account)
            .to([mb("to@example.com")])
            .set_title("HiTool")
            .set_content("<p>ok</p>", true)
            .build()
            .unwrap();
        let formatted = String::from_utf8(mime.formatted()).unwrap();
        assert!(formatted.contains("Subject: HiTool"));
        assert!(formatted.contains("from@example.com"));
    }
}
