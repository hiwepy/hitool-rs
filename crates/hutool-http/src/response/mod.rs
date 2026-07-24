//! Hutool-aligned buffered HTTP response facade (`cn.hutool.http.HttpResponse`).

use crate::{HttpError, StatusCode, header};
use encoding_rs::Encoding;
use std::{
    fmt,
    io::Cursor,
    path::{Path, PathBuf},
};

mod http_cookie;
mod http_response;

pub use http_cookie::HttpCookie;
pub use http_response::HttpResponse;
