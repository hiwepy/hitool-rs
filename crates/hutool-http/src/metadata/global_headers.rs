//! Hutool-aligned HTTP metadata and explicitly owned default headers.

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};

/// Explicitly owned equivalent of Hutool's mutable global header collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalHeaders {
    headers: HashMap<String, Vec<String>>,
}

impl Default for GlobalHeaders {
    fn default() -> Self {
        let mut headers = Self {
            headers: HashMap::new(),
        };
        headers.put_default(false);
        headers
    }
}

impl GlobalHeaders {
    /// Creates an owned collection populated with Hutool's safe defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Installs default headers, optionally clearing custom values first.
    pub fn put_default(&mut self, reset: bool) -> &mut Self {
        if reset {
            self.headers.clear();
        }
        self.insert(
            Header::Accept.value(),
            "text/html,application/json,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            true,
        );
        self.insert(Header::AcceptEncoding.value(), "gzip, deflate", true);
        self.insert(
            Header::UserAgent.value(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 Hutool",
            true,
        );
        self
    }

    /// Returns the first value for a non-blank header name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.header_list(name)?.first().map(String::as_str)
    }

    /// Returns all values for a non-blank header name.
    #[must_use]
    pub fn header_list(&self, name: &str) -> Option<&[String]> {
        let name = name.trim();
        (!name.is_empty())
            .then(|| self.headers.get(name).map(Vec::as_slice))
            .flatten()
    }

    /// Returns the first value for a typed header name.
    #[must_use]
    pub fn named_header(&self, name: Header) -> Option<&str> {
        self.header(name.value())
    }

    /// Inserts or appends a string header.
    pub fn insert(&mut self, name: &str, value: &str, overwrite: bool) -> &mut Self {
        let name = name.trim();
        if name.is_empty() {
            return self;
        }
        match self.headers.entry(name.to_owned()) {
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

    /// Appends every supplied header value.
    pub fn extend(&mut self, headers: &HashMap<String, Vec<String>>) -> &mut Self {
        for (name, values) in headers {
            for value in values {
                self.insert(name, value, false);
            }
        }
        self
    }

    /// Removes a string header.
    pub fn remove(&mut self, name: &str) -> &mut Self {
        self.headers.remove(name.trim());
        self
    }

    /// Removes a typed header.
    pub fn remove_header(&mut self, name: Header) -> &mut Self {
        self.remove(name.value())
    }

    /// Returns the owned collection's read-only map.
    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    /// Clears every header, including defaults.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }
}
