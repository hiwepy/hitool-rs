//! Excel date utilities aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelDateUtil`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelDateUtil.java
//!
//! 提供 Excel 序列号(1900/1904 基准)与 Java 时间对象互转。

use crate::{PoiError, Result};

/// Excel date utility.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelDateUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelDateUtil;

impl ExcelDateUtil {
    /// 对齐 Java: `ExcelDateUtil.isExcelDate(Date)`
    pub fn is_excel_date(_epoch_days: i64) -> bool {
        false
    }
    /// 对齐 Java: `ExcelDateUtil.javaToDate(double, boolean)`
    pub fn java_to_date(_serial: f64, _use_1904_windowing: bool) -> Result<()> {
        Err(PoiError::PendingEngine(
            "ExcelDateUtil::javaToDate (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelDateUtil.dateToJava(Date, boolean)`
    pub fn date_to_java(_value: &str, _use_1904_windowing: bool) -> Result<f64> {
        Err(PoiError::PendingEngine(
            "ExcelDateUtil::dateToJava (waiting for easyexcel-rs)",
        ))
    }
    /// 对齐 Java: `ExcelDateUtil.getMs(Date)`
    pub fn get_ms(_value: &str) -> Result<i64> {
        Err(PoiError::PendingEngine(
            "ExcelDateUtil::getMs (waiting for easyexcel-rs)",
        ))
    }
}