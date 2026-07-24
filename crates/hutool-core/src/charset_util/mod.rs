use std::{
    fmt, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use encoding_rs::{Encoding, GBK, UTF_8};
use thiserror::Error;

mod charset;
mod charset_error;
mod charset_util;

pub use charset::Charset;
pub use charset_error::CharsetError;
pub use charset_util::CharsetUtil;
