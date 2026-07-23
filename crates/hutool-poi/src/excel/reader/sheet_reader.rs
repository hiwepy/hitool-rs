//! 迁移自 hutool 的 `cn.hutool.poi.excel.reader.SheetReader`
//!
//! - 原 Java 包：`cn.hutool.poi.excel.reader`
//! - 原 Java 主类：`cn.hutool.poi.excel.reader.SheetReader`
//! - 迁移状态：🟡 占位实现，等待 `easyexcel-rs` / `easydoc-rs` / `easyofd-rs` / `easypdf-rs` 完成
//! - Java 源文件：`hutool-poi/src/main/java/excel/reader/SheetReader.java`

#![allow(dead_code, clippy::missing_docs_in_private_items)]

/// 占位结构体，对齐 Java `SheetReader`。
///
/// 当前状态：等待 `easyexcel-rs` 引擎完成后填充实现。
pub struct SheetReader;

impl SheetReader {
    /// 占位方法。当前调用会 panic。
    ///
    /// # Panics
    ///
    /// 此方法尚未实现，等待 `easyexcel-rs` / `easydoc-rs` 等引擎完成。
    pub fn new() -> Self {
        unimplemented!("SheetReader::new() 等待 easyexcel-rs / easydoc-rs / easyofd-rs / easypdf-rs 完成")
    }
}

impl Default for SheetReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "等待 easyexcel-rs")]
    fn sheetreader_new_is_unimplemented() {
        let _ = SheetReader::new();
    }
}
