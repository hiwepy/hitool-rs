//! ZIP creation and bounded, path-safe extraction.
//!
//! 对齐: `cn.hutool.extra.compress.CompressUtil`
//! 对齐: `cn.hutool.core.util.ZipUtil`（extra 侧安全 ZIP 子集）

use crate::{ExtraError, Result};
use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, Write},
    path::{Component, Path},
};
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

mod extraction_limits;
mod compress_util;
mod zip_util;

pub use extraction_limits::ExtractionLimits;
pub use compress_util::CompressUtil;
pub use zip_util::ZipUtil;
pub use compress_util::create_zip;
pub use compress_util::extract_zip;
