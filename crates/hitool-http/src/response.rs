use crate::{HttpError, StatusCode, header};
use encoding_rs::Encoding;
use std::{fmt, io::Cursor};

/// Buffered HTTP response facade aligned with Hutool's `HttpResponse`.
///
/// The transport remains reqwest. This facade owns the status, headers and a
/// body that was collected under the client's configured byte limit, making
/// repeated body reads and synchronous writes deterministic.
#[derive(Clone, PartialEq, Eq)]
pub struct HttpResponse {
    status: StatusCode,
    headers: header::HeaderMap,
    body: Vec<u8>,
}

impl HttpResponse {
    pub(crate) const fn new(status: StatusCode, headers: header::HeaderMap, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Returns the numeric response status.
    #[must_use]
    pub const fn get_status(&self) -> u16 {
        self.status.as_u16()
    }

    /// Returns the typed response status.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Reports whether the status is in the `200..=299` range.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status.is_success()
    }

    /// Returns all response headers.
    #[must_use]
    pub const fn headers(&self) -> &header::HeaderMap {
        &self.headers
    }

    /// Returns the first response-header value when it is valid text.
    #[must_use]
    pub fn header(&self, name: impl header::AsHeaderName) -> Option<&str> {
        self.headers.get(name)?.to_str().ok()
    }

    /// Returns the `Content-Encoding` value.
    #[must_use]
    pub fn content_encoding(&self) -> Option<&str> {
        self.header(header::CONTENT_ENCODING)
    }

    /// Returns the protocol-valid content length, or `-1` when unknown.
    ///
    /// As in Hutool, a declared length is invalidated by transfer or content
    /// encoding because it no longer describes the decoded body exposed here.
    #[must_use]
    pub fn content_length(&self) -> i64 {
        let length = self
            .header(header::CONTENT_LENGTH)
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(-1);
        if length > 0 && (self.is_chunked() || self.has_content_encoding()) {
            -1
        } else {
            length
        }
    }

    /// Reports `Content-Encoding: gzip`.
    #[must_use]
    pub fn is_gzip(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| value.eq_ignore_ascii_case("gzip"))
    }

    /// Reports `Content-Encoding: deflate`.
    #[must_use]
    pub fn is_deflate(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| value.eq_ignore_ascii_case("deflate"))
    }

    /// Reports `Transfer-Encoding: chunked`.
    #[must_use]
    pub fn is_chunked(&self) -> bool {
        self.header(header::TRANSFER_ENCODING)
            .is_some_and(|value| value.eq_ignore_ascii_case("chunked"))
    }

    /// Returns the first raw `Set-Cookie` header.
    #[must_use]
    pub fn get_cookie_str(&self) -> Option<&str> {
        self.header(header::SET_COOKIE)
    }

    /// Returns a new reader over the buffered response body.
    #[must_use]
    pub fn body_stream(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.body)
    }

    /// Returns the response bytes without another allocation.
    #[must_use]
    pub fn body_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Replaces the buffered response body.
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Decodes the body using the response charset, defaulting to UTF-8.
    #[must_use]
    pub fn body(&self) -> String {
        let encoding = self
            .response_charset()
            .and_then(|label| Encoding::for_label(label.as_bytes()))
            .unwrap_or(encoding_rs::UTF_8);
        let (text, _, _) = encoding.decode(&self.body);
        text.into_owned()
    }

    /// Writes the entire buffered body and returns the number of bytes written.
    pub fn write_body(&self, output: &mut impl std::io::Write) -> Result<u64, HttpError> {
        output.write_all(&self.body)?;
        output.flush()?;
        Ok(self.body.len() as u64)
    }

    fn has_content_encoding(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| !value.trim().is_empty())
    }

    fn response_charset(&self) -> Option<&str> {
        self.header(header::CONTENT_TYPE)?
            .split(';')
            .skip(1)
            .find_map(|parameter| {
                let (name, value) = parameter.trim().split_once('=')?;
                name.trim()
                    .eq_ignore_ascii_case("charset")
                    .then(|| value.trim().trim_matches(['\'', '"']))
            })
    }
}

impl fmt::Debug for HttpResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body_bytes", &self.body.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read as _;

    fn response(headers: &[(&'static str, &'static str)], body: &[u8]) -> HttpResponse {
        let mut map = header::HeaderMap::new();
        for (name, value) in headers {
            map.insert(*name, header::HeaderValue::from_static(value));
        }
        HttpResponse::new(StatusCode::CREATED, map, body.to_vec())
    }

    #[test]
    fn status_headers_encodings_and_lengths_match_hutool_semantics() {
        let plain = response(
            &[
                ("content-length", "4"),
                ("set-cookie", "session=abc; HttpOnly"),
            ],
            b"body",
        );
        assert_eq!(plain.get_status(), 201);
        assert_eq!(plain.status(), StatusCode::CREATED);
        assert!(plain.is_ok());
        assert_eq!(plain.headers().len(), 2);
        assert_eq!(plain.header("missing"), None);
        assert_eq!(plain.content_encoding(), None);
        assert_eq!(plain.content_length(), 4);
        assert!(!plain.is_gzip());
        assert!(!plain.is_deflate());
        assert!(!plain.is_chunked());
        assert_eq!(plain.get_cookie_str(), Some("session=abc; HttpOnly"));
        assert!(format!("{plain:?}").contains("body_bytes: 4"));
        assert_eq!(plain.clone(), plain);

        let gzip = response(
            &[("content-length", "99"), ("content-encoding", "GZIP")],
            b"encoded",
        );
        assert!(gzip.is_gzip());
        assert_eq!(gzip.content_length(), -1);
        assert_ne!(plain, gzip);

        let deflate = response(&[("content-encoding", "deflate")], b"encoded");
        assert!(deflate.is_deflate());
        assert_eq!(deflate.content_length(), -1);

        let chunked = response(
            &[("content-length", "99"), ("transfer-encoding", "Chunked")],
            b"chunk",
        );
        assert!(chunked.is_chunked());
        assert_eq!(chunked.content_length(), -1);

        assert_eq!(
            response(&[("content-length", "bad")], b"").content_length(),
            -1
        );
        assert_eq!(
            response(&[("content-length", "0")], b"").content_length(),
            0
        );
        assert_eq!(
            response(
                &[("content-length", "3"), ("content-encoding", "  ")],
                b"raw"
            )
            .content_length(),
            3
        );

        let mut invalid = header::HeaderMap::new();
        invalid.insert(
            "x-binary",
            header::HeaderValue::from_bytes(b"\xff").unwrap(),
        );
        assert_eq!(
            HttpResponse::new(StatusCode::BAD_REQUEST, invalid, Vec::new()).header("x-binary"),
            None
        );
    }

    #[test]
    fn body_decoding_stream_replacement_and_writes_are_real() {
        let mut latin = response(
            &[(
                "content-type",
                "text/plain; ignored; charset='windows-1252'",
            )],
            b"caf\xe9",
        );
        assert_eq!(latin.body(), "café");
        assert_eq!(latin.body_bytes(), b"caf\xe9");
        let mut reader = latin.body_stream();
        let mut copied = Vec::new();
        reader.read_to_end(&mut copied).unwrap();
        assert_eq!(copied, b"caf\xe9");

        latin.set_body(b"changed".to_vec());
        assert_eq!(latin.body(), "changed");
        let mut output = TestWriter::new(Failure::None);
        assert_eq!(latin.write_body(&mut output).unwrap(), 7);
        assert_eq!(output.bytes, b"changed");

        let utf8 = response(&[("content-type", "text/plain; charset=unknown")], b"\xff");
        assert_eq!(utf8.body(), "�");
        assert_eq!(response(&[], "你好".as_bytes()).body(), "你好");

        let mut write_failure = TestWriter::new(Failure::Write);
        assert!(latin.write_body(&mut write_failure).is_err());
        let mut flush_failure = TestWriter::new(Failure::Flush);
        assert!(latin.write_body(&mut flush_failure).is_err());
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Failure {
        None,
        Write,
        Flush,
    }

    struct TestWriter {
        failure: Failure,
        bytes: Vec<u8>,
    }

    impl TestWriter {
        const fn new(failure: Failure) -> Self {
            Self {
                failure,
                bytes: Vec::new(),
            }
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
            if self.failure == Failure::Write {
                Err(std::io::Error::other("write failed"))
            } else {
                self.bytes.extend_from_slice(buffer);
                Ok(buffer.len())
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            if self.failure == Failure::Flush {
                Err(std::io::Error::other("flush failed"))
            } else {
                Ok(())
            }
        }
    }
}
