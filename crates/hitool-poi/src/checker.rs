//! POI precondition checks aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.PoiChecker`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/PoiChecker.java
//!
//! Hutool 在 POI 操作前调用 `PoiChecker.checkFoo()` 校验依赖库版本,
//! 主要检测 `poi-ooxml` 与 `poi` 的冲突。Rust 版本提供基础空对齐桩,
//! 等待 easyexcel-rs 完成时填充具体的版本约束。

use crate::{PoiError, Result};

/// POI dependency and environment checks.
///
/// 对齐 Java: `cn.hutool.poi.PoiChecker` (static utility)
#[derive(Debug, Clone, Copy, Default)]
pub struct PoiChecker;

impl PoiChecker {
    /// Verifies that the POI stack is reachable on the classpath.
    ///
    /// 对齐 Java: `PoiChecker.checkPoiPresent()`
    pub fn check_poi_present() -> Result<()> {
        Err(PoiError::PendingEngine(
            "PoiChecker::check_poi_present (waiting for easyexcel-rs)",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checker_reports_pending_engine() {
        let error = PoiChecker::check_poi_present().unwrap_err();
        assert!(matches!(error, PoiError::PendingEngine(_)));
    }
}