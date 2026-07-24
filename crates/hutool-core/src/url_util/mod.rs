//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

use crate::net::rfc3986::Rfc3986;
use crate::net::url_decoder::UrlDecoder;
use crate::string::{is_blank, trim};
use crate::{CoreError, Result};

mod hit_uri;
mod url_util;

pub use hit_uri::HitUri;
pub use url_util::UrlUtil;
