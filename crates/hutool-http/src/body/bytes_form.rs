//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Marker trait for request body writers (Hutool `RequestBody`).
///
/// Java: `cn.hutool.http.body.RequestBody`
pub trait RequestBody {
    /// Writes the body bytes to `out`.
    ///
    /// Java: `RequestBody.write(OutputStream out)`
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()>;
}

/// Raw byte body.
///
/// Java: `cn.hutool.http.body.BytesBody`
#[derive(Debug, Clone)]
pub struct BytesBody {
    content: Vec<u8>,
}

impl BytesBody {
    /// Java: `BytesBody.create(byte[])` / `new BytesBody(byte[])`
    #[must_use]
    pub fn create(content: impl Into<Vec<u8>>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Alias for [`Self::create`].
    #[must_use]
    pub fn new(content: impl Into<Vec<u8>>) -> Self {
        Self::create(content)
    }

    /// Returns the owned content bytes.
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Java: `BytesBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(&self.content)?;
        out.flush()
    }
}

impl RequestBody for BytesBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(&self.content)?;
        out.flush()
    }
}

/// `application/x-www-form-urlencoded` body.
///
/// Java: `cn.hutool.http.body.FormUrlEncodedBody`
#[derive(Debug, Clone)]
pub struct FormUrlEncodedBody {
    encoded: String,
    charset: String,
}

impl FormUrlEncodedBody {
    /// Java: `FormUrlEncodedBody.create(Map, Charset)` / constructor.
    #[must_use]
    pub fn create(form: &IndexMap<String, String>, charset: &str) -> Self {
        Self {
            encoded: HttpUtil::to_params_form(form, true),
            charset: charset.to_string(),
        }
    }

    /// Alias for [`Self::create`].
    #[must_use]
    pub fn new(form: &IndexMap<String, String>, charset: &str) -> Self {
        Self::create(form, charset)
    }

    /// Returns the encoded payload.
    #[must_use]
    pub fn encoded(&self) -> &str {
        &self.encoded
    }

    /// Java: `FormUrlEncodedBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(self.encoded.as_bytes())?;
        out.flush()
    }
}

impl fmt::Display for FormUrlEncodedBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hutool toString returns the encoded body string.
        f.write_str(&self.encoded)
    }
}

impl RequestBody for FormUrlEncodedBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(self.encoded.as_bytes())?;
        out.flush()
    }
}

/// Resource-backed body (bytes + optional name / content-type).
///
/// Java: `cn.hutool.http.body.ResourceBody` over Hutool `Resource`.
#[derive(Debug, Clone)]
pub struct ResourceBody {
    name: Option<String>,
    content_type: Option<String>,
    bytes: Vec<u8>,
    source: Option<PathBuf>,
}

impl ResourceBody {
    /// Creates a resource body from in-memory bytes.
    ///
    /// Java: `ResourceBody.create(Resource)` / constructor — Rust takes bytes directly.
    #[must_use]
    pub fn create(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            name: None,
            content_type: None,
            bytes: bytes.into(),
            source: None,
        }
    }

    /// Creates a resource body by reading a filesystem path.
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned());
        Ok(Self {
            name,
            content_type: None,
            bytes,
            source: Some(path.to_path_buf()),
        })
    }

    /// Sets the resource display name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the content type.
    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Java: `Resource.getName()`
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Java: `Resource.getContentType()` / HttpResource content type.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Returns the body bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the filesystem source path when created via [`Self::from_path`].
    #[must_use]
    pub fn source_path(&self) -> Option<&Path> {
        self.source.as_deref()
    }

    /// Java: `ResourceBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(&self.bytes)?;
        out.flush()
    }
}

impl fmt::Display for ResourceBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceBody{{name={:?}, bytes={}, source={:?}}}",
            self.name,
            self.bytes.len(),
            self.source
        )
    }
}

impl RequestBody for ResourceBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(&self.bytes)?;
        out.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_and_form_bodies_write() {
        let bytes = BytesBody::create(b"abc".as_slice());
        let mut out = Vec::new();
        bytes.write(&mut out).unwrap();
        assert_eq!(out, b"abc");

        let mut form = IndexMap::new();
        form.insert("a".into(), "1".into());
        let body = FormUrlEncodedBody::create(&form, "UTF-8");
        assert_eq!(body.to_string(), "a=1");
        let mut out2 = Vec::new();
        body.write(&mut out2).unwrap();
        assert_eq!(out2, b"a=1");
    }

    #[test]
    fn resource_body_from_bytes() {
        let body = ResourceBody::create(b"data".as_slice())
            .with_name("f.txt")
            .with_content_type("text/plain");
        assert_eq!(body.name(), Some("f.txt"));
        assert_eq!(body.content_type(), Some("text/plain"));
        let mut out = Vec::new();
        RequestBody::write(&body, &mut out).unwrap();
        assert_eq!(out, b"data");
        assert!(body.to_string().contains("f.txt"));
    }
}
