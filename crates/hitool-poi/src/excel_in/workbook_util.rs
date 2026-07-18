//! Workbook factory utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.WorkbookUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/WorkbookUtil.java
//!
//! 提供 `createBook(...)` / `createSXSSFBook(...)` 等创建工作簿的便捷工厂。

use crate::{PoiError, Result};

/// Workbook factory utility.
///
/// 对齐 Java: `cn.hutool.poi.excel.WorkbookUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct WorkbookUtil;

impl WorkbookUtil {
    /// 对齐 Java: `WorkbookUtil.createBook(String)`
    pub fn create_book_path(_path: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(String, boolean)`
    pub fn create_book_path_readonly(_path: &str, _read_only: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(File)`
    pub fn create_book() -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(File, boolean)`
    pub fn create_book_readonly(_read_only: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBookForWriter(File)`
    pub fn create_book_for_writer() -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBookForWriter (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(File, String)`
    pub fn create_book_password(_password: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(File, String, boolean)`
    pub fn create_book_password_readonly(_password: &str, _read_only: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(InputStream)`
    pub fn create_book_stream(_bytes: &[u8]) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(InputStream, String)`
    pub fn create_book_stream_password(_bytes: &[u8], _password: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createBook(boolean)`
    pub fn create_book_xlsx(_is_xlsx: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createSXSSFBook(String)`
    pub fn create_sxssf_book_path(_path: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createSXSSFBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createSXSSFBook(String, boolean)`
    pub fn create_sxssf_book_path_readonly(_path: &str, _read_only: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createSXSSFBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createSXSSFBook(File)`
    pub fn create_sxssf_book() -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createSXSSFBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createSXSSFBook(File, boolean)`
    pub fn create_sxssf_book_readonly(_read_only: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createSXSSFBook (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `WorkbookUtil.createSXSSFBook(File, String)`
    pub fn create_sxssf_book_password(_password: &str) -> Result<()> {
        Err(PoiError::PendingEngine(
            "WorkbookUtil::createSXSSFBook (waiting for easyexcel-rs)",
        ))
    }
}