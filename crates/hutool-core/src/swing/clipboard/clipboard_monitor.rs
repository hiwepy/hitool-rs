//! 对齐: `cn.hutool.core.swing.clipboard.ClipboardMonitor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/clipboard/ClipboardMonitor.java
//!
//! 单例入口，委托 `ClipboardUtil::listen` 注册监听。

use super::clipboard_util::ClipboardUtil;

/// 对齐 Java enum: `cn.hutool.core.swing.clipboard.ClipboardMonitor`
#[derive(Debug, Clone, Copy, Default)]
pub struct ClipboardMonitor;

impl ClipboardMonitor {
    /// 对齐 Java: `INSTANCE`
    pub const INSTANCE: Self = Self;

    /// 对齐 Java: 启动监听（parity 测试注册两个 listener）。
    pub fn monitor<F1, F2>(first: F1, second: F2)
    where
        F1: Fn(&str) + Send + Sync + 'static,
        F2: Fn(&str) + Send + Sync + 'static,
    {
        ClipboardUtil::listen(first, false);
        ClipboardUtil::listen(second, true);
    }
}
