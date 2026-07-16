use std::io::{Cursor, Read};

use quick_xml::{Reader, events::Event};
use zip::ZipArchive;

use crate::{PoiError, Result};

/// Defensive resource limits for XLSX input.
#[derive(Debug, Clone, Copy)]
pub struct XlsxReadLimits {
    /// Maximum compressed workbook size accepted in memory.
    pub max_archive_bytes: usize,
    /// Maximum expanded size of any XML entry read by this parser.
    pub max_entry_bytes: usize,
    /// Maximum number of cells materialized from the first worksheet.
    pub max_cells: usize,
    /// Maximum accepted one-based worksheet row number.
    pub max_rows: usize,
    /// Maximum accepted one-based worksheet column number.
    pub max_columns: usize,
}

impl Default for XlsxReadLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 32 * 1024 * 1024,
            max_entry_bytes: 16 * 1024 * 1024,
            max_cells: 1_000_000,
            max_rows: 100_000,
            max_columns: 16_384,
        }
    }
}

/// Reads the first worksheet of an XLSX document as a sparse-aware string
/// table.
///
/// Shared strings, inline strings, booleans, numbers, and cached formula
/// values are supported. Styling, macros, external links, and formula
/// execution are deliberately ignored.
pub fn read_first_sheet(bytes: &[u8], limits: XlsxReadLimits) -> Result<Vec<Vec<String>>> {
    if bytes.len() > limits.max_archive_bytes {
        return Err(PoiError::ResourceLimit("compressed archive bytes"));
    }
    if limits.max_entry_bytes == 0
        || limits.max_cells == 0
        || limits.max_rows == 0
        || limits.max_columns == 0
    {
        return Err(PoiError::ResourceLimit("zero read limit"));
    }

    let mut archive = ZipArchive::new(Cursor::new(bytes))?;
    let workbook = read_entry(&mut archive, "xl/workbook.xml", limits.max_entry_bytes)?;
    let relationship_id = first_sheet_relationship_id(&workbook)?;
    let relationships = read_entry(
        &mut archive,
        "xl/_rels/workbook.xml.rels",
        limits.max_entry_bytes,
    )?;
    let target = relationship_target(&relationships, &relationship_id)?;
    let worksheet_path = normalize_worksheet_target(&target)?;
    let worksheet = read_entry(&mut archive, &worksheet_path, limits.max_entry_bytes)?;
    let shared_strings =
        match read_optional_entry(&mut archive, "xl/sharedStrings.xml", limits.max_entry_bytes)? {
            Some(xml) => parse_shared_strings(&xml)?,
            None => Vec::new(),
        };
    parse_worksheet(&worksheet, &shared_strings, limits)
}

fn read_entry(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
    limit: usize,
) -> Result<Vec<u8>> {
    let mut entry = archive.by_name(name)?;
    read_bounded(&mut entry, limit)
}

fn read_optional_entry(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    name: &str,
    limit: usize,
) -> Result<Option<Vec<u8>>> {
    match archive.by_name(name) {
        Ok(mut entry) => read_bounded(&mut entry, limit).map(Some),
        Err(zip::result::ZipError::FileNotFound) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

fn read_bounded(reader: &mut impl Read, limit: usize) -> Result<Vec<u8>> {
    let take_limit = u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1);
    let mut bytes = Vec::new();
    reader.take(take_limit).read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(PoiError::ResourceLimit("expanded XML entry bytes"));
    }
    Ok(bytes)
}

fn first_sheet_relationship_id(xml: &[u8]) -> Result<String> {
    let mut reader = Reader::from_reader(xml);
    loop {
        match reader.read_event() {
            Ok(Event::Start(element) | Event::Empty(element))
                if element.local_name().as_ref() == b"sheet" =>
            {
                return attribute(&reader, &element, b"r:id")?
                    .or_else(|| attribute(&reader, &element, b"id").ok().flatten())
                    .ok_or_else(|| {
                        PoiError::MalformedXlsx("first sheet has no relationship id".into())
                    });
            }
            Ok(Event::Eof) => {
                return Err(PoiError::MalformedXlsx("workbook has no worksheet".into()));
            }
            Err(error) => return Err(xml_error(&error)),
            _ => {}
        }
    }
}

fn relationship_target(xml: &[u8], relationship_id: &str) -> Result<String> {
    let mut reader = Reader::from_reader(xml);
    loop {
        match reader.read_event() {
            Ok(Event::Start(element) | Event::Empty(element))
                if element.local_name().as_ref() == b"Relationship" =>
            {
                if attribute(&reader, &element, b"Id")?.as_deref() == Some(relationship_id) {
                    return attribute(&reader, &element, b"Target")?.ok_or_else(|| {
                        PoiError::MalformedXlsx("worksheet relationship has no target".into())
                    });
                }
            }
            Ok(Event::Eof) => {
                return Err(PoiError::MalformedXlsx(
                    "worksheet relationship was not found".into(),
                ));
            }
            Err(error) => return Err(xml_error(&error)),
            _ => {}
        }
    }
}

fn normalize_worksheet_target(target: &str) -> Result<String> {
    let normalized = if let Some(target) = target.strip_prefix('/') {
        target.to_owned()
    } else if target.starts_with("xl/") {
        target.to_owned()
    } else {
        format!("xl/{target}")
    };
    if normalized.split('/').any(|part| part == "..") || normalized.contains('\\') {
        return Err(PoiError::MalformedXlsx(
            "unsafe worksheet relationship target".into(),
        ));
    }
    Ok(normalized)
}

fn parse_shared_strings(xml: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(xml);
    let mut strings = Vec::new();
    let mut current = String::new();
    let mut in_item = false;
    let mut in_text = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) if element.local_name().as_ref() == b"si" => {
                current.clear();
                in_item = true;
            }
            Ok(Event::Start(element)) if in_item && element.local_name().as_ref() == b"t" => {
                in_text = true;
            }
            Ok(Event::Text(text)) if in_text => current.push_str(&decode_text(&text)?),
            Ok(Event::End(element)) if element.local_name().as_ref() == b"t" => in_text = false,
            Ok(Event::End(element)) if element.local_name().as_ref() == b"si" => {
                strings.push(std::mem::take(&mut current));
                in_item = false;
            }
            Ok(Event::Eof) => return Ok(strings),
            Err(error) => return Err(xml_error(&error)),
            _ => {}
        }
    }
}

fn parse_worksheet(
    xml: &[u8],
    shared: &[String],
    limits: XlsxReadLimits,
) -> Result<Vec<Vec<String>>> {
    let mut reader = Reader::from_reader(xml);
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut cell: Option<(usize, usize, Option<String>, String)> = None;
    let mut capture_value = false;
    let mut cells = 0_usize;
    let mut materialized_slots = 0_usize;

    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) if element.local_name().as_ref() == b"c" => {
                cells = cells.saturating_add(1);
                if cells > limits.max_cells {
                    return Err(PoiError::ResourceLimit("worksheet cell count"));
                }
                let reference = attribute(&reader, &element, b"r")?
                    .ok_or_else(|| PoiError::MalformedXlsx("cell has no reference".into()))?;
                let (row, column) = parse_cell_reference(&reference)?;
                if row >= limits.max_rows || column >= limits.max_columns {
                    return Err(PoiError::ResourceLimit("worksheet dimensions"));
                }
                let kind = attribute(&reader, &element, b"t")?;
                cell = Some((row, column, kind, String::new()));
            }
            Ok(Event::Start(element))
                if cell.is_some() && matches!(element.local_name().as_ref(), b"v" | b"t") =>
            {
                capture_value = true;
            }
            Ok(Event::Text(text)) if capture_value => {
                if let Some((_, _, _, value)) = &mut cell {
                    value.push_str(&decode_text(&text)?);
                }
            }
            Ok(Event::End(element)) if matches!(element.local_name().as_ref(), b"v" | b"t") => {
                capture_value = false;
            }
            Ok(Event::End(element)) if element.local_name().as_ref() == b"c" => {
                let (row, column, kind, raw) = cell
                    .take()
                    .ok_or_else(|| PoiError::MalformedXlsx("unexpected cell close".into()))?;
                let value = match kind.as_deref() {
                    Some("s") => {
                        let index = raw.parse::<usize>().map_err(|_| {
                            PoiError::MalformedXlsx("invalid shared string index".into())
                        })?;
                        shared.get(index).cloned().ok_or_else(|| {
                            PoiError::MalformedXlsx("shared string index is out of range".into())
                        })?
                    }
                    Some("b") if raw == "1" => "true".to_owned(),
                    Some("b") if raw == "0" => "false".to_owned(),
                    _ => raw,
                };
                if table.len() <= row {
                    table.resize_with(row + 1, Vec::new);
                }
                if table[row].len() <= column {
                    let added = column + 1 - table[row].len();
                    materialized_slots = materialized_slots.saturating_add(added);
                    if materialized_slots > limits.max_cells {
                        return Err(PoiError::ResourceLimit("materialized worksheet cells"));
                    }
                    table[row].resize(column + 1, String::new());
                }
                table[row][column] = value;
            }
            Ok(Event::Eof) => return Ok(table),
            Err(error) => return Err(xml_error(&error)),
            _ => {}
        }
    }
}

fn parse_cell_reference(reference: &str) -> Result<(usize, usize)> {
    let split = reference
        .find(|character: char| character.is_ascii_digit())
        .ok_or_else(|| PoiError::MalformedXlsx("invalid cell reference".into()))?;
    let (letters, digits) = reference.split_at(split);
    if letters.is_empty()
        || digits.is_empty()
        || !letters.bytes().all(|byte| byte.is_ascii_alphabetic())
    {
        return Err(PoiError::MalformedXlsx("invalid cell reference".into()));
    }
    let column = letters
        .bytes()
        .try_fold(0_usize, |value, byte| {
            value.checked_mul(26).and_then(|value| {
                value.checked_add(usize::from(byte.to_ascii_uppercase() - b'A' + 1))
            })
        })
        .ok_or(PoiError::ResourceLimit("worksheet column index"))?;
    let row = digits
        .parse::<usize>()
        .ok()
        .and_then(|row| row.checked_sub(1))
        .ok_or_else(|| PoiError::MalformedXlsx("invalid cell row".into()))?;
    Ok((row, column - 1))
}

fn attribute(
    reader: &Reader<&[u8]>,
    element: &quick_xml::events::BytesStart<'_>,
    name: &[u8],
) -> Result<Option<String>> {
    for attribute in element.attributes() {
        let attribute = attribute.map_err(|error| PoiError::MalformedXlsx(error.to_string()))?;
        if attribute.key.as_ref() == name || attribute.key.local_name().as_ref() == name {
            return attribute
                .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
                .map(|value| Some(value.into_owned()))
                .map_err(|error| xml_error(&error));
        }
    }
    Ok(None)
}

fn decode_text(text: &quick_xml::events::BytesText<'_>) -> Result<String> {
    let decoded = text
        .decode()
        .map_err(|error| PoiError::MalformedXlsx(error.to_string()))?;
    quick_xml::escape::unescape(&decoded)
        .map(std::borrow::Cow::into_owned)
        .map_err(|error| PoiError::MalformedXlsx(error.to_string()))
}

fn xml_error(error: &quick_xml::Error) -> PoiError {
    PoiError::MalformedXlsx(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sparse_cell_references() {
        assert_eq!(parse_cell_reference("A1").unwrap(), (0, 0));
        assert_eq!(parse_cell_reference("AA12").unwrap(), (11, 26));
        assert!(parse_cell_reference("A0").is_err());
    }

    #[test]
    fn rejects_traversing_relationship_targets() {
        assert!(normalize_worksheet_target("../secrets.xml").is_err());
    }

    #[test]
    fn rejects_sparse_dimension_amplification() {
        let xml =
            br#"<worksheet><sheetData><row><c r="Z999"><v>1</v></c></row></sheetData></worksheet>"#;
        let limits = XlsxReadLimits {
            max_rows: 10,
            max_columns: 10,
            ..XlsxReadLimits::default()
        };
        assert!(matches!(
            parse_worksheet(xml, &[], limits),
            Err(PoiError::ResourceLimit("worksheet dimensions"))
        ));
    }
}
