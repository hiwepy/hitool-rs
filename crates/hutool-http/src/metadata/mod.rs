//! Hutool-aligned HTTP metadata and explicitly owned default headers.

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};

mod content_type;
mod http_status;
mod status;
mod global_headers;

pub use content_type::ContentType;
pub use http_status::HttpStatus;
pub use status::Status;
pub use global_headers::GlobalHeaders;
