//! Shared Hutool-style request and response metadata.

use crate::Header;
use encoding_rs::{Encoding, UTF_8};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};
use thiserror::Error;

mod http_base_error;
mod http_base;

pub use http_base_error::HttpBaseError;
pub use http_base::HttpBase;
pub use http_base_error::HTTP_1_0;
pub use http_base::HTTP_1_1;
