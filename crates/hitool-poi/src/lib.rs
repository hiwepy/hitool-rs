//! Office and tabular data helpers aligned with Hutool's POI module.
//!
//! The implementation is pure Rust and does not depend on Apache POI.

#![forbid(unsafe_code)]

#[cfg(feature = "docx")]
mod word;
#[cfg(feature = "xlsx")]
mod xlsx;

#[cfg(feature = "csv")]
pub use csv;
#[cfg(feature = "xlsx")]
pub use rust_xlsxwriter;
#[cfg(feature = "csv")]
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;
#[cfg(feature = "docx")]
pub use word::{DocxLimits, WordDocument, WordRun};
#[cfg(feature = "xlsx")]
pub use xlsx::{XlsxReadLimits, read_first_sheet};

/// Tabular document errors.
#[derive(Debug, Error)]
pub enum PoiError {
    /// XLSX creation failed.
    #[cfg(feature = "xlsx")]
    #[error(transparent)]
    XlsxWrite(#[from] rust_xlsxwriter::XlsxError),
    /// CSV serialization or parsing failed.
    #[cfg(feature = "csv")]
    #[error(transparent)]
    Csv(#[from] csv::Error),
    /// In-memory writer finalization failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// XLSX ZIP container processing failed.
    #[cfg(any(feature = "docx", feature = "xlsx"))]
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    /// XLSX XML content is malformed or unsupported.
    #[cfg(feature = "xlsx")]
    #[error("malformed XLSX: {0}")]
    MalformedXlsx(String),
    /// A document exceeded configured resource limits.
    #[error("document resource limit exceeded: {0}")]
    ResourceLimit(&'static str),
    /// A row or column index cannot be represented by XLSX.
    #[cfg(feature = "xlsx")]
    #[error("table dimensions exceed XLSX limits")]
    TableTooLarge,
    /// CSV output was not valid UTF-8.
    #[cfg(feature = "csv")]
    #[error("CSV output was not valid UTF-8")]
    InvalidUtf8,
}

/// Result type for tabular document operations.
pub type Result<T> = std::result::Result<T, PoiError>;

/// Writes a string table into an in-memory XLSX workbook.
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

/// Serializes records as UTF-8 CSV with a header row.
#[cfg(feature = "csv")]
pub fn write_csv<T: Serialize>(records: impl IntoIterator<Item = T>) -> Result<String> {
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
#[cfg(feature = "csv")]
pub fn read_csv<T: DeserializeOwned>(bytes: impl AsRef<[u8]>) -> Result<Vec<T>> {
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
