//! XLSX writer facade aligned with Hutool's POI Excel write utilities.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelUtil.getWriter(...)` / `ExcelWriter` 顶层快速写入
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelUtil.java
//!       hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelWriter.java
//!
//! 保留原有最小可用实现 —— 使用 `rust_xlsxwriter` 完成内存 XLSX 包装,
//! 复杂的 `ExcelWriter` API 完整对齐桩见 `excel/excel_writer.rs`。

use crate::{PoiError, Result};

/// Writes a string table into an in-memory XLSX workbook.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelUtil.getWriter(...)` 后调用
/// `ExcelWriter.write(...)` 的常见简写场景。
///
/// # 参数
/// - `sheet_name`: 工作表名称
/// - `rows`: 行向量,每行是字符串单元格向量
///
/// # 错误
/// - `PoiError::TableTooLarge` 行/列超出 XLSX 维度
/// - `PoiError::XlsxWrite` `rust_xlsxwriter` 失败
#[cfg(feature = "xlsx")]
pub fn write_xlsx(sheet_name: &str, rows: &[Vec<String>]) -> Result<Vec<u8>> {
    let mut workbook = rust_xlsxwriter::Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name(sheet_name)?;
    for (row_index, row) in rows.iter().enumerate() {
        let row_index = u32::try_from(row_index).map_err(|_| PoiError::TableTooLarge)?;
        for (column_index, value) in row.iter().enumerate() {
            let column_index = u16::try_from(column_index).map_err(|_| PoiError::TableTooLarge)?;
            worksheet.write_string(row_index, column_index, value)?;
        }
    }
    Ok(workbook.save_to_buffer()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xlsx::XlsxReadLimits;
    use crate::xlsx::read_first_sheet;

    #[test]
    #[cfg(feature = "xlsx")]
    fn writes_a_valid_xlsx_container() {
        let rows = vec![
            vec!["name".into(), "age".into()],
            vec!["Ada".into(), "36".into()],
        ];
        let bytes = write_xlsx("People", &rows).unwrap();
        assert!(bytes.starts_with(b"PK"));
        assert!(bytes.len() > 1_000);
        assert_eq!(
            read_first_sheet(&bytes, XlsxReadLimits::default()).unwrap(),
            rows
        );
    }
}
