//! 迁移自 hutool 的 `cn.hutool.poi.word.TableUtil`
//!
//! - 原 Java 包：`cn.hutool.poi.word`
//! - 原 Java 主类：`cn.hutool.poi.word.TableUtil`
//! - 迁移状态：🟡 占位实现，等待 `easyexcel-rs` / `easydoc-rs` / `easyofd-rs` / `easypdf-rs` 完成
//! - Java 源文件：`hutool-poi/src/main/java/word/TableUtil.java`

#![allow(dead_code, clippy::missing_docs_in_private_items)]

/// 占位结构体，对齐 Java `TableUtil`。
///
/// 当前状态：等待 `easyexcel-rs` 引擎完成后填充实现。
pub struct TableUtil;

impl TableUtil {
    /// 占位方法。当前调用会 panic。
    ///
    /// # Panics
    ///
    /// 此方法尚未实现，等待 `easyexcel-rs` / `easydoc-rs` 等引擎完成。
    pub fn new() -> Self {
        unimplemented!("TableUtil::new() 等待 easyexcel-rs / easydoc-rs / easyofd-rs / easypdf-rs 完成")
    }
}

impl Default for TableUtil {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "等待 easyexcel-rs")]
    fn tableutil_new_is_unimplemented() {
        let _ = TableUtil::new();
    }
}
