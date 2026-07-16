//! Core utilities shared by the `HiTool` workspace.
//!
//! The crate intentionally avoids async runtimes, HTTP clients, and database
//! drivers. It provides small, deterministic building blocks with explicit
//! errors and allocation behavior.

#![forbid(unsafe_code)]

mod codec;
mod collection;
mod date;
mod error;
mod id;
mod string;

pub use codec::{
    base64_decode, base64_encode, base64_url_decode, base64_url_encode, hex_decode, hex_encode,
    percent_decode, percent_encode_component,
};
pub use collection::{distinct, group_by, partition};
pub use date::DateUtil;
pub use error::{CoreError, Result};
pub use id::IdUtil;
pub use string::{
    StrExt, format_template, is_blank, lower_first, remove_all, remove_chars, split, upper_first,
};

/// Common imports for applications using `hitool-core`.
pub mod prelude {
    pub use crate::{DateUtil, IdUtil, StrExt};
}
