//! Hutool-aligned HTTP metadata and explicitly owned default headers.

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};

/// Common media types exposed by Hutool's `ContentType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    /// `application/x-www-form-urlencoded`.
    FormUrlEncoded,
    /// `multipart/form-data`.
    Multipart,
    /// `application/json`.
    Json,
    /// `application/xml`.
    Xml,
    /// `text/plain`.
    TextPlain,
    /// `text/xml`.
    TextXml,
    /// `text/html`.
    TextHtml,
    /// `application/octet-stream`.
    OctetStream,
    /// `text/event-stream`.
    EventStream,
}

impl fmt::Display for ContentType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

macro_rules! status_codes {
    ($($name:ident = $value:literal),+ $(,)?) => {
        $(
            #[doc = concat!("HTTP status code `", stringify!($value), "`.")]
            pub const $name: u16 = $value;
        )+
    };
}
