//! Global POI configuration aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.GlobalPoiConfig`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/GlobalPoiConfig.java
//!
//! Hutool 通过 `GlobalPoiConfig` 提供 Excel 写入时是否覆盖已存在文件等全局开关。
//! Rust 版本提供零字节静态值对象 + 关联函数,语义对齐但底层状态可由调用方注入。

use std::sync::atomic::{AtomicBool, Ordering};

/// Global POI configuration facade.
///
/// 对齐 Java: `cn.hutool.poi.GlobalPoiConfig`
/// 类型: Java 单例(static utility class) → Rust ZST + 原子全局开关
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalPoiConfig;

impl GlobalPoiConfig {
    /// Atomically set the "overwrite existing files" flag.
    ///
    /// 对齐 Java: `GlobalPoiConfig.setOverwriteFile(boolean)`
    pub fn set_overwrite_file(overwrite: bool) {
        OVERWRITE_FILE.store(overwrite, Ordering::Relaxed);
    }

    /// Returns whether existing destination files will be overwritten.
    ///
    /// 对齐 Java: `GlobalPoiConfig.isOverwriteFile()`
    pub fn is_overwrite_file() -> bool {
        OVERWRITE_FILE.load(Ordering::Relaxed)
    }

    /// Resets the global flag to its documented default of `true`.
    ///
    /// 对齐 Java: `GlobalPoiConfig.setOverwriteFile(true)` 默认行为。
    pub fn reset() {
        OVERWRITE_FILE.store(true, Ordering::Relaxed);
    }
}

static OVERWRITE_FILE: AtomicBool = AtomicBool::new(true);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overwrite_flag_round_trips_and_resets() {
        GlobalPoiConfig::reset();
        assert!(GlobalPoiConfig::is_overwrite_file());
        GlobalPoiConfig::set_overwrite_file(false);
        assert!(!GlobalPoiConfig::is_overwrite_file());
        GlobalPoiConfig::reset();
        assert!(GlobalPoiConfig::is_overwrite_file());
    }
}