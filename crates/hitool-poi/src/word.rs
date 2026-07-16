use std::{fmt::Write as _, io::Write as _};

use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::{PoiError, Result};

/// Resource limits applied before packaging a DOCX document.
#[derive(Debug, Clone, Copy)]
pub struct DocxLimits {
    /// Maximum paragraphs, including paragraphs inside table cells.
    pub max_paragraphs: usize,
    /// Maximum text runs.
    pub max_runs: usize,
    /// Maximum UTF-8 bytes across all text.
    pub max_text_bytes: usize,
    /// Maximum table cells.
    pub max_table_cells: usize,
    /// Maximum generated DOCX bytes.
    pub max_output_bytes: usize,
}

impl Default for DocxLimits {
    fn default() -> Self {
        Self {
            max_paragraphs: 100_000,
            max_runs: 500_000,
            max_text_bytes: 32 * 1024 * 1024,
            max_table_cells: 1_000_000,
            max_output_bytes: 64 * 1024 * 1024,
        }
    }
}

/// One formatted Word text run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordRun {
    text: String,
    bold: bool,
    italic: bool,
    size_half_points: Option<usize>,
}

impl WordRun {
    /// Creates a plain run.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            italic: false,
            size_half_points: None,
        }
    }

    /// Enables bold formatting.
    #[must_use]
    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enables italic formatting.
    #[must_use]
    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Sets font size in half-points, matching the OOXML representation.
    #[must_use]
    pub const fn size(mut self, half_points: usize) -> Self {
        self.size_half_points = Some(half_points);
        self
    }

    fn write_xml(&self, xml: &mut String) {
        xml.push_str("<w:r>");
        if self.bold || self.italic || self.size_half_points.is_some() {
            xml.push_str("<w:rPr>");
            if self.bold {
                xml.push_str("<w:b/>");
            }
            if self.italic {
                xml.push_str("<w:i/>");
            }
            if let Some(size) = self.size_half_points {
                write!(xml, "<w:sz w:val=\"{size}\"/>").expect("writing to a String cannot fail");
            }
            xml.push_str("</w:rPr>");
        }
        xml.push_str("<w:t xml:space=\"preserve\">");
        write_escaped(xml, &self.text);
        xml.push_str("</w:t></w:r>");
    }
}

#[derive(Debug, Clone)]
enum WordBlock {
    Paragraph(Vec<WordRun>),
    Heading { level: u8, text: String },
    Table(Vec<Vec<String>>),
}

/// In-memory Word 2007+ document builder.
///
/// It emits a deliberately small OOXML package and never executes macros,
/// formulas, external relationships, or embedded objects.
#[derive(Debug, Clone, Default)]
pub struct WordDocument {
    blocks: Vec<WordBlock>,
}

impl WordDocument {
    /// Creates an empty document.
    #[must_use]
    pub const fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// Appends a paragraph containing ordered runs.
    #[must_use]
    pub fn add_paragraph(mut self, runs: impl IntoIterator<Item = WordRun>) -> Self {
        self.blocks
            .push(WordBlock::Paragraph(runs.into_iter().collect()));
        self
    }

    /// Appends a heading from level 1 through 9.
    pub fn add_heading(mut self, level: u8, text: impl Into<String>) -> Result<Self> {
        if !(1..=9).contains(&level) {
            return Err(PoiError::ResourceLimit("DOCX heading level"));
        }
        self.blocks.push(WordBlock::Heading {
            level,
            text: text.into(),
        });
        Ok(self)
    }

    /// Appends a table of UTF-8 cell values.
    #[must_use]
    pub fn add_table(mut self, rows: Vec<Vec<String>>) -> Self {
        self.blocks.push(WordBlock::Table(rows));
        self
    }

    /// Packages this document as a DOCX byte vector.
    pub fn to_docx(&self, limits: DocxLimits) -> Result<Vec<u8>> {
        let mut counts = Counts::default();
        let mut document = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>"#,
        );

        for block in &self.blocks {
            match block {
                WordBlock::Paragraph(runs) => {
                    counts.add_paragraph(limits)?;
                    document.push_str("<w:p>");
                    for run in runs {
                        counts.add_run(run.text.len(), limits)?;
                        run.write_xml(&mut document);
                    }
                    document.push_str("</w:p>");
                }
                WordBlock::Heading { level, text } => {
                    counts.add_paragraph(limits)?;
                    counts.add_run(text.len(), limits)?;
                    let outline = level - 1;
                    let size = 34_usize.saturating_sub(usize::from(*level) * 2).max(20);
                    write!(
                        document,
                        "<w:p><w:pPr><w:outlineLvl w:val=\"{outline}\"/></w:pPr><w:r><w:rPr><w:b/><w:sz w:val=\"{size}\"/></w:rPr><w:t>"
                    )
                    .expect("writing to a String cannot fail");
                    write_escaped(&mut document, text);
                    document.push_str("</w:t></w:r></w:p>");
                }
                WordBlock::Table(rows) => {
                    document.push_str("<w:tbl><w:tblPr><w:tblBorders><w:top w:val=\"single\" w:sz=\"4\"/><w:left w:val=\"single\" w:sz=\"4\"/><w:bottom w:val=\"single\" w:sz=\"4\"/><w:right w:val=\"single\" w:sz=\"4\"/><w:insideH w:val=\"single\" w:sz=\"4\"/><w:insideV w:val=\"single\" w:sz=\"4\"/></w:tblBorders></w:tblPr>");
                    for row in rows {
                        document.push_str("<w:tr>");
                        for value in row {
                            counts.add_table_cell(limits)?;
                            counts.add_paragraph(limits)?;
                            counts.add_run(value.len(), limits)?;
                            document.push_str("<w:tc><w:p><w:r><w:t xml:space=\"preserve\">");
                            write_escaped(&mut document, value);
                            document.push_str("</w:t></w:r></w:p></w:tc>");
                        }
                        document.push_str("</w:tr>");
                    }
                    document.push_str("</w:tbl>");
                }
            }
        }
        document.push_str("<w:sectPr><w:pgSz w:w=\"11906\" w:h=\"16838\"/><w:pgMar w:top=\"1440\" w:right=\"1440\" w:bottom=\"1440\" w:left=\"1440\"/></w:sectPr></w:body></w:document>");
        package_docx(&document, limits.max_output_bytes)
    }
}

#[derive(Debug, Default)]
struct Counts {
    paragraphs: usize,
    runs: usize,
    text_bytes: usize,
    table_cells: usize,
}

impl Counts {
    fn add_paragraph(&mut self, limits: DocxLimits) -> Result<()> {
        self.paragraphs = self.paragraphs.saturating_add(1);
        if self.paragraphs > limits.max_paragraphs {
            return Err(PoiError::ResourceLimit("DOCX paragraph count"));
        }
        Ok(())
    }

    fn add_run(&mut self, bytes: usize, limits: DocxLimits) -> Result<()> {
        self.runs = self.runs.saturating_add(1);
        self.text_bytes = self.text_bytes.saturating_add(bytes);
        if self.runs > limits.max_runs {
            return Err(PoiError::ResourceLimit("DOCX run count"));
        }
        if self.text_bytes > limits.max_text_bytes {
            return Err(PoiError::ResourceLimit("DOCX text bytes"));
        }
        Ok(())
    }

    fn add_table_cell(&mut self, limits: DocxLimits) -> Result<()> {
        self.table_cells = self.table_cells.saturating_add(1);
        if self.table_cells > limits.max_table_cells {
            return Err(PoiError::ResourceLimit("DOCX table cell count"));
        }
        Ok(())
    }
}

fn package_docx(document: &str, max_output_bytes: usize) -> Result<Vec<u8>> {
    const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    const RELATIONSHIPS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let mut writer = ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, bytes) in [
        ("[Content_Types].xml", CONTENT_TYPES.as_bytes()),
        ("_rels/.rels", RELATIONSHIPS.as_bytes()),
        ("word/document.xml", document.as_bytes()),
    ] {
        writer.start_file(name, options)?;
        writer.write_all(bytes)?;
    }
    let bytes = writer.finish()?.into_inner();
    if bytes.len() > max_output_bytes {
        return Err(PoiError::ResourceLimit("DOCX output bytes"));
    }
    Ok(bytes)
}

fn write_escaped(output: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '\"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            character if character.is_control() && !matches!(character, '\t' | '\n' | '\r') => {
                output.push('\u{FFFD}');
            }
            character => output.push(character),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn writes_text_headings_and_tables_to_docx() {
        let document = WordDocument::new()
            .add_heading(1, "HiTool & report")
            .unwrap()
            .add_paragraph([
                WordRun::new("production ").bold(),
                WordRun::new("utilities").italic(),
            ])
            .add_table(vec![vec!["name".into(), "value".into()]]);
        let bytes = document.to_docx(DocxLimits::default()).unwrap();
        assert!(bytes.starts_with(b"PK"));
        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut xml = String::new();
        archive
            .by_name("word/document.xml")
            .unwrap()
            .read_to_string(&mut xml)
            .unwrap();
        assert!(xml.contains("HiTool &amp; report"));
        assert!(xml.contains("production "));
        assert!(xml.contains("<w:tbl>"));
    }

    #[test]
    fn rejects_invalid_heading_and_resource_amplification() {
        assert!(WordDocument::new().add_heading(0, "bad").is_err());
        let document = WordDocument::new().add_paragraph([WordRun::new("too long")]);
        let limits = DocxLimits {
            max_text_bytes: 1,
            ..DocxLimits::default()
        };
        assert!(matches!(
            document.to_docx(limits),
            Err(PoiError::ResourceLimit("DOCX text bytes"))
        ));
    }
}
