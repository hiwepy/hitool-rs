//! Word sub-facade aligned with Hutool's `cn.hutool.poi.word.*` package.
//!
//! 模块重导出 Hutool 同名 Java 类的对齐桩:
//! - [`word07_writer`] → `cn.hutool.poi.word.Word07Writer`
//! - [`word_util`]     → `cn.hutool.poi.word.WordUtil`
//! - [`doc_util`]      → `cn.hutool.poi.word.DocUtil`
//! - [`table_util`]    → `cn.hutool.poi.word.TableUtil`
//! - [`pic_type`]      → `cn.hutool.poi.word.PicType`
//! - [`word`]          → 保留 `WordDocument` / `WordRun` / `DocxLimits` 实现

#[cfg(feature = "docx")]
pub mod doc_util;
#[cfg(feature = "docx")]
pub mod pic_type;
#[cfg(feature = "docx")]
pub mod table_util;
#[cfg(feature = "docx")]
pub mod word07_writer;
#[cfg(feature = "docx")]
pub mod word;
#[cfg(feature = "docx")]
pub mod word_util;