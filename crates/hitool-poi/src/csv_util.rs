//! CSV utilities aligned with Hutool's POI CSV support.
//!
//! 对齐: `cn.hutool.poi.excel.ExcelUtil` 中与 CSV 相关的静态方法
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/excel/ExcelUtil.java
//!
//! 保留原有的最小可用实现 —— 使用 `csv` crate 完成 UTF-8 序列化/反序列化,
//! 与 Hutool 通过 Apache POI 处理 CSV 的语义保持等价。

use crate::{PoiError, Result};

/// Serializes records as UTF-8 CSV with a header row.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelUtil.toCsv(...)` 系列
///
/// # 参数
/// - `records`: 任意可序列化的记录迭代器
///
/// # 错误
/// - `PoiError::Csv` 序列化失败
/// - `PoiError::InvalidUtf8` 输出非 UTF-8
#[cfg(feature = "csv")]
pub fn write_csv<T: serde::Serialize>(records: impl IntoIterator<Item = T>) -> Result<String> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    for record in records {
        writer.serialize(record)?;
    }
    let bytes = writer
        .into_inner()
        .map_err(csv::IntoInnerError::into_error)?;
    String::from_utf8(bytes).map_err(|_| PoiError::InvalidUtf8)
}

/// Deserializes header-based UTF-8 CSV records.
///
/// 对齐 Java: `cn.hutool.poi.excel.ExcelUtil.fromCsv(...)` 系列
///
/// # 类型参数
/// - `T`: 反序列化目标类型,需实现 `serde::de::DeserializeOwned`
///
/// # 参数
/// - `bytes`: UTF-8 编码的 CSV 内容
#[cfg(feature = "csv")]
pub fn read_csv<T: serde::de::DeserializeOwned>(bytes: impl AsRef<[u8]>) -> Result<Vec<T>> {
    csv::Reader::from_reader(bytes.as_ref())
        .deserialize()
        .collect::<std::result::Result<_, _>>()
        .map_err(PoiError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "csv")]
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "csv")]
    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u8,
    }

    #[test]
    #[cfg(feature = "csv")]
    fn csv_records_round_trip() {
        let records = vec![Person {
            name: "Ada".into(),
            age: 36,
        }];
        let csv = write_csv(&records).unwrap();
        assert_eq!(read_csv::<Person>(csv).unwrap(), records);
    }
}
