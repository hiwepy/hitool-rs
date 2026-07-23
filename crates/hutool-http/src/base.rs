//! Shared Hutool-style request and response metadata.

use crate::Header;
use encoding_rs::{Encoding, UTF_8};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};
use thiserror::Error;

/// HTTP/1.0 protocol label used by Hutool.
pub const HTTP_1_0: &str = "HTTP/1.0";
/// HTTP/1.1 protocol label used by Hutool.
pub const HTTP_1_1: &str = "HTTP/1.1";

/// Errors returned while configuring shared HTTP metadata.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HttpBaseError {
    /// The requested character encoding is not supported by Encoding Standard.
    #[error("unsupported HTTP character encoding: {0}")]
    UnsupportedCharset(String),
}

/// Shared, explicitly owned metadata for Hutool-style requests and responses.
#[derive(Debug, Clone)]
pub struct HttpBase {
    header_aggregated: bool,
    headers: HashMap<String, Vec<String>>,
    charset: &'static Encoding,
    http_version: String,
    body: Option<Vec<u8>>,
}

impl Default for HttpBase {
    fn default() -> Self {
        Self {
            header_aggregated: false,
            headers: HashMap::new(),
            charset: UTF_8,
            http_version: HTTP_1_1.to_owned(),
            body: None,
        }
    }
}

impl HttpBase {
    /// Creates empty UTF-8 HTTP/1.1 metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the first value for a non-blank, case-insensitive header name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.header_list(name)?.first().map(String::as_str)
    }

    /// Returns all values for a non-blank, case-insensitive header name.
    #[must_use]
    pub fn header_list(&self, name: &str) -> Option<&[String]> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }
        self.headers
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, values)| values.as_slice())
    }

    /// Returns the first value for a typed header name.
    #[must_use]
    pub fn named_header(&self, name: Header) -> Option<&str> {
        self.header(name.value())
    }

    /// Inserts or appends a header while treating its name case-insensitively.
    pub fn insert(&mut self, name: &str, value: &str, overwrite: bool) -> &mut Self {
        let name = name.trim();
        if name.is_empty() {
            return self;
        }
        let canonical = self
            .headers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| name.to_owned());
        match self.headers.entry(canonical) {
            Entry::Occupied(mut entry) if !overwrite => {
                entry.get_mut().push(value.trim().to_owned());
            }
            Entry::Occupied(mut entry) => {
                entry.insert(vec![value.to_owned()]);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![value.to_owned()]);
            }
        }
        self
    }

    /// Inserts or appends a typed header.
    pub fn insert_header(&mut self, name: Header, value: &str, overwrite: bool) -> &mut Self {
        self.insert(name.value(), value, overwrite)
    }

    /// Overwrites a typed header value.
    pub fn set_header(&mut self, name: Header, value: &str) -> &mut Self {
        self.insert_header(name, value, true)
    }

    /// Overwrites a string header value.
    pub fn set(&mut self, name: &str, value: &str) -> &mut Self {
        self.insert(name, value, true)
    }

    /// Inserts a flat header map using the requested overwrite mode.
    pub fn extend_flat(&mut self, headers: &HashMap<String, String>, overwrite: bool) -> &mut Self {
        for (name, value) in headers {
            self.insert(name, value, overwrite);
        }
        self
    }

    /// Inserts a multi-value header map using the requested overwrite mode.
    pub fn extend(&mut self, headers: &HashMap<String, Vec<String>>, overwrite: bool) -> &mut Self {
        for (name, values) in headers {
            for value in values {
                self.insert(name, value, overwrite);
            }
        }
        self
    }

    /// Appends every value from a flat header map.
    pub fn add_headers(&mut self, headers: &HashMap<String, String>) -> &mut Self {
        self.extend_flat(headers, false)
    }

    /// Removes a header by a case-insensitive string name.
    pub fn remove(&mut self, name: &str) -> &mut Self {
        let name = name.trim();
        if let Some(key) = self
            .headers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .cloned()
        {
            self.headers.remove(&key);
        }
        self
    }

    /// Removes a typed header.
    pub fn remove_header(&mut self, name: Header) -> &mut Self {
        self.remove(name.value())
    }

    /// Returns the read-only header map.
    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    /// Clears every header.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    /// Enables or disables comma aggregation of repeated wire headers.
    pub const fn header_aggregation(&mut self, aggregate: bool) -> &mut Self {
        self.header_aggregated = aggregate;
        self
    }

    /// Reports whether repeated wire headers are comma-aggregated.
    #[must_use]
    pub const fn is_header_aggregated(&self) -> bool {
        self.header_aggregated
    }

    /// Materializes headers as they should be applied to an HTTP request.
    #[must_use]
    pub fn wire_headers(&self) -> Vec<(String, String)> {
        if self.header_aggregated {
            self.headers
                .iter()
                .map(|(name, values)| (name.clone(), values.join(",")))
                .collect()
        } else {
            self.headers
                .iter()
                .flat_map(|(name, values)| values.iter().map(|value| (name.clone(), value.clone())))
                .collect()
        }
    }

    /// Returns the metadata-only HTTP protocol label.
    #[must_use]
    pub fn http_version(&self) -> &str {
        &self.http_version
    }

    /// Sets the metadata-only HTTP protocol label.
    pub fn set_http_version(&mut self, http_version: impl Into<String>) -> &mut Self {
        self.http_version = http_version.into();
        self
    }

    /// Returns the optional body bytes.
    #[must_use]
    pub fn body_bytes(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Replaces the optional body bytes.
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    /// Clears the body.
    pub fn clear_body(&mut self) -> &mut Self {
        self.body = None;
        self
    }

    /// Returns the canonical character-set name.
    #[must_use]
    pub fn charset(&self) -> &'static str {
        self.charset.name()
    }

    /// Resolves and sets a non-blank character-set label.
    pub fn set_charset_name(&mut self, charset: &str) -> Result<&mut Self, HttpBaseError> {
        let charset = charset.trim();
        if charset.is_empty() {
            return Ok(self);
        }
        let encoding = Encoding::for_label(charset.as_bytes())
            .ok_or_else(|| HttpBaseError::UnsupportedCharset(charset.to_owned()))?;
        Ok(self.set_charset(encoding))
    }

    /// Sets an Encoding Standard character set.
    pub const fn set_charset(&mut self, charset: &'static Encoding) -> &mut Self {
        self.charset = charset;
        self
    }
}

impl fmt::Display for HttpBase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Headers: \r\n")?;
        for (name, values) in &self.headers {
            formatter.write_str("    ")?;
            formatter.write_str(name)?;
            formatter.write_str(": ")?;
            formatter.write_str(&values.join(","))?;
            formatter.write_str("\r\n")?;
        }
        formatter.write_str("Body: \r\n    ")?;
        if let Some(body) = &self.body {
            formatter.write_str(&self.charset.decode(body).0)?;
        } else {
            formatter.write_str("null")?;
        }
        formatter.write_str("\r\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding_rs::GBK;

    struct FailingWriter {
        fail_at: usize,
        writes: usize,
    }

    impl fmt::Write for FailingWriter {
        fn write_str(&mut self, _value: &str) -> fmt::Result {
            if self.writes == self.fail_at {
                return Err(fmt::Error);
            }
            self.writes += 1;
            Ok(())
        }
    }

    fn assert_each_display_write_propagates(base: &HttpBase, write_count: usize) {
        for fail_at in 0..write_count {
            let mut sink = FailingWriter { fail_at, writes: 0 };
            assert!(fmt::write(&mut sink, format_args!("{base}")).is_err());
        }
    }

    #[test]
    fn headers_are_case_insensitive_mutable_and_wire_aggregation_is_explicit() {
        let mut base = HttpBase::new();
        assert_eq!(base.header(""), None);
        assert_eq!(base.header("missing"), None);
        base.insert(" ", "ignored", true)
            .insert("X-Test", " first ", true)
            .insert("x-test", " second ", false);
        assert_eq!(base.headers().len(), 1);
        assert_eq!(base.header(" X-TEST "), Some(" first "));
        assert_eq!(
            base.header_list("x-test"),
            Some([" first ".to_owned(), "second".to_owned()].as_slice())
        );

        base.set("X-Test", "third")
            .insert_header(Header::ContentType, "text/plain", false)
            .set_header(Header::ContentType, "application/json");
        assert_eq!(
            base.named_header(Header::ContentType),
            Some("application/json")
        );

        let flat = HashMap::from([
            ("X-Flat".to_owned(), "a".to_owned()),
            ("x-test".to_owned(), "fourth".to_owned()),
        ]);
        base.extend_flat(&flat, false);
        let multi = HashMap::from([
            (
                "X-Multi".to_owned(),
                vec!["one".to_owned(), " two ".to_owned()],
            ),
            ("X-Empty".to_owned(), Vec::new()),
        ]);
        base.extend(&multi, false);
        base.add_headers(&HashMap::from([("X-Added".to_owned(), "value".to_owned())]));

        let unaggregated = base.wire_headers();
        assert!(unaggregated.contains(&("X-Multi".to_owned(), "one".to_owned())));
        assert!(unaggregated.contains(&("X-Multi".to_owned(), "two".to_owned())));
        base.header_aggregation(true);
        assert!(base.is_header_aggregated());
        let aggregated = base.wire_headers();
        assert!(aggregated.contains(&("X-Multi".to_owned(), "one,two".to_owned())));

        base.remove("not-there").remove("X-FLAT");
        assert_eq!(base.header("X-Flat"), None);
        base.remove_header(Header::ContentType);
        assert_eq!(base.named_header(Header::ContentType), None);
        base.clear_headers();
        assert!(base.headers().is_empty());
    }

    #[test]
    fn version_body_charset_and_display_match_hutool_metadata_semantics() {
        let mut base = HttpBase::default();
        assert_eq!(base.http_version(), HTTP_1_1);
        assert_eq!(base.charset(), "UTF-8");
        assert_eq!(base.body_bytes(), None);
        assert!(base.to_string().ends_with("Body: \r\n    null\r\n"));
        assert_each_display_write_propagates(&base, 4);

        base.set_http_version(HTTP_1_0)
            .set_header(Header::ContentType, "text/plain");
        assert_eq!(base.http_version(), HTTP_1_0);
        base.set_charset(GBK);
        assert_eq!(base.charset(), "GBK");
        base.set_charset_name("  ").unwrap();
        assert_eq!(base.charset(), "GBK");
        assert_eq!(
            base.set_charset_name("not-a-real-charset").unwrap_err(),
            HttpBaseError::UnsupportedCharset("not-a-real-charset".to_owned())
        );
        base.set_charset_name("utf-8").unwrap();
        base.set_body("你好".as_bytes());
        assert_eq!(base.body_bytes(), Some("你好".as_bytes()));
        let display = base.to_string();
        assert!(display.contains("Content-Type: text/plain\r\n"));
        assert!(display.ends_with("Body: \r\n    你好\r\n"));
        assert_each_display_write_propagates(&base, 9);
        base.clear_body();
        assert_eq!(base.body_bytes(), None);
    }
}
