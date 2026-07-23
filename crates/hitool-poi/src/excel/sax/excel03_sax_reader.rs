//! 迁移自 hutool 的 `cn.hutool.poi.excel.sax.Excel03SaxReader`
//!
//! - 原 Java 包：`cn.hutool.poi.excel.sax`
//! - 原 Java 主类：`cn.hutool.poi.excel.sax.Excel03SaxReader`
//! - 迁移状态：🟡 占位实现，等待 `easyexcel-rs` / `easydoc-rs` / `easyofd-rs` / `easypdf-rs` 完成
//! - Java 源文件：`hutool-poi/src/main/java/excel/sax/Excel03SaxReader.java`

#![allow(dead_code, clippy::missing_docs_in_private_items)]

/// 占位结构体，对齐 Java `Excel03SaxReader`。
///
/// 当前状态：等待 `easyexcel-rs` 引擎完成后填充实现。
pub struct Excel03SaxReader;

impl Excel03SaxReader {
    /// 占位方法。当前调用会 panic。
    ///
    /// # Panics
    ///
    /// 此方法尚未实现，等待 `easyexcel-rs` / `easydoc-rs` 等引擎完成。
    pub fn new() -> Self {
        unimplemented!("Excel03SaxReader::new() 等待 easyexcel-rs / easydoc-rs / easyofd-rs / easypdf-rs 完成")
    }
}

impl Default for Excel03SaxReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "等待 easyexcel-rs")]
    fn excel03saxreader_new_is_unimplemented() {
        let _ = Excel03SaxReader::new();
    }
}
