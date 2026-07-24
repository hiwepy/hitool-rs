//! Bounded streaming XML reader, visitor, transformer, and writer.

use std::{
    io::{BufRead, Read, Take, Write},
    ops::ControlFlow,
};

use indexmap::IndexMap;
use quick_xml::{
    escape::resolve_predefined_entity,
    events::{BytesEnd, BytesRef, BytesStart, Event},
    Reader, Writer, XmlVersion,
};

use crate::{CoreError, Result};

/// Namespace handling applied when XML names are copied into the DOM.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NamespaceMode {
    /// Keep qualified names such as `soap:Envelope`.
    #[default]
    Preserve,
    /// Keep only the local part such as `Envelope`.
    LocalName,
}

/// Defensive limits and parsing policy for XML input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlParseOptions {
    /// Maximum number of input bytes consumed by the parser.
    pub max_input_bytes: usize,
    /// Maximum element nesting depth.
    pub max_depth: usize,
    /// Maximum number of elements, including empty elements.
    pub max_nodes: usize,
    /// Maximum number of attributes on one element.
    pub max_attributes_per_element: usize,
    /// Maximum cumulative decoded text and CDATA bytes.
    pub max_text_bytes: usize,
    /// Whether insignificant leading and trailing text whitespace is removed.
    pub trim_text: bool,
    /// Namespace name handling used by DOM construction.
    pub namespace_mode: NamespaceMode,
    /// Whether a document type declaration may be observed.
    pub allow_doctype: bool,
    /// Whether unknown named general references are retained literally.
    ///
    /// Numeric references and the five predefined XML references are always
    /// accepted. This option never expands declarations from a DTD.
    pub allow_general_references: bool,
    /// Whether illegal XML control characters are removed before string parsing.
    ///
    /// Generic `BufRead` input is buffered only when this is explicitly enabled.
    pub sanitize_invalid_chars: bool,
}

impl Default for XmlParseOptions {
    fn default() -> Self {
        Self {
            max_input_bytes: 16 * 1024 * 1024,
            max_depth: 128,
            max_nodes: 100_000,
            max_attributes_per_element: 256,
            max_text_bytes: 8 * 1024 * 1024,
            trim_text: true,
            namespace_mode: NamespaceMode::Preserve,
            allow_doctype: false,
            allow_general_references: false,
            sanitize_invalid_chars: false,
        }
    }
}

#[derive(Debug)]
struct ParseState {
    depth: usize,
    nodes: usize,
    text_bytes: usize,
    root_seen: bool,
    root_closed: bool,
    version: XmlVersion,
}

impl Default for ParseState {
    fn default() -> Self {
        Self {
            depth: 0,
            nodes: 0,
            text_bytes: 0,
            root_seen: false,
            root_closed: false,
            version: XmlVersion::Implicit1_0,
        }
    }
}

/// Pull reader that reuses one allocation buffer and enforces [`XmlParseOptions`].
pub struct XmlEventReader<R: BufRead> {
    reader: Reader<Take<R>>,
    buffer: Vec<u8>,
    options: XmlParseOptions,
    state: ParseState,
}

impl<R: BufRead> std::fmt::Debug for XmlEventReader<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("XmlEventReader")
            .field("buffer_capacity", &self.buffer.capacity())
            .field("options", &self.options)
            .field("depth", &self.state.depth)
            .field("nodes", &self.state.nodes)
            .field("text_bytes", &self.state.text_bytes)
            .finish_non_exhaustive()
    }
}

impl<R: BufRead> XmlEventReader<R> {
    /// Creates a bounded reader. No input is consumed until [`Self::read_event`].
    #[must_use]
    pub fn new(source: R, options: XmlParseOptions) -> Self {
        let limit = u64::try_from(options.max_input_bytes)
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let mut reader = Reader::from_reader(source.take(limit));
        reader.config_mut().trim_text(options.trim_text);
        Self {
            reader,
            buffer: Vec::new(),
            options,
            state: ParseState::default(),
        }
    }

    /// Reads the next borrowed event.
    ///
    /// The returned event borrows the reader's single reusable buffer, so it
    /// must be dropped before reading another event.
    pub fn read_event(&mut self) -> Result<Event<'_>> {
        self.buffer.clear();
        let event = self.reader.read_event_into(&mut self.buffer);
        let position = usize::try_from(self.reader.buffer_position()).unwrap_or(usize::MAX);
        if position > self.options.max_input_bytes {
            return Err(CoreError::XmlLimit {
                resource: "input bytes",
                max: self.options.max_input_bytes,
            });
        }
        let event = event.map_err(|error| CoreError::Xml(error.to_string()))?;
        validate_event(&event, &self.options, &mut self.state)?;
        Ok(event)
    }

    /// Returns the current reusable buffer capacity.
    #[must_use]
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Returns the active parsing policy.
    #[must_use]
    pub fn options(&self) -> &XmlParseOptions {
        &self.options
    }

    /// Returns the XML version observed in the declaration, or implicit XML 1.0.
    #[must_use]
    pub(crate) fn xml_version(&self) -> XmlVersion {
        self.state.version
    }

    /// Returns the decoder selected by the XML declaration.
    #[must_use]
    pub(crate) fn decoder(&self) -> quick_xml::encoding::Decoder {
        self.reader.decoder()
    }
}

/// Streaming XML writer backed by `quick_xml::Writer`.
pub struct XmlEventWriter<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> std::fmt::Debug for XmlEventWriter<W> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("XmlEventWriter")
            .finish_non_exhaustive()
    }
}

impl<W: Write> XmlEventWriter<W> {
    /// Creates a compact writer.
    #[must_use]
    pub fn new(target: W) -> Self {
        Self {
            writer: Writer::new(target),
        }
    }

    /// Creates a writer that indents structural events.
    #[must_use]
    pub fn with_indent(target: W, indent_char: u8, indent_size: usize) -> Self {
        Self {
            writer: Writer::new_with_indent(target, indent_char, indent_size),
        }
    }

    /// Selects whether empty elements are written as `<tag />` instead of `<tag/>`.
    pub fn set_space_before_empty_slash(&mut self, enabled: bool) {
        self.writer
            .config_mut()
            .add_space_before_slash_in_empty_elements = enabled;
    }

    /// Writes one XML event.
    pub fn write_event(&mut self, event: Event<'_>) -> Result<()> {
        self.writer.write_event(event).map_err(CoreError::Io)
    }

    /// Returns the underlying writer.
    #[must_use]
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}

/// Action returned by a streaming transform callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTransformAction {
    /// Copy the current event to the target.
    Keep,
    /// Omit the current event from the target.
    Drop,
}

/// Visits validated events without constructing a DOM.
///
/// Returning [`ControlFlow::Break`] stops before the remaining input is read.
pub fn visit_xml<R, B, F>(
    source: R,
    options: XmlParseOptions,
    mut visitor: F,
) -> Result<ControlFlow<B>>
where
    R: BufRead,
    F: for<'event> FnMut(&Event<'event>) -> Result<ControlFlow<B>>,
{
    let mut reader = XmlEventReader::new(source, options);
    loop {
        let event = reader.read_event()?;
        if matches!(event, Event::Eof) {
            return Ok(ControlFlow::Continue(()));
        }
        if let ControlFlow::Break(value) = visitor(&event)? {
            return Ok(ControlFlow::Break(value));
        }
    }
}

/// Copies validated events through a filtering transform into a writer.
pub fn transform_xml<R, W, F>(
    source: R,
    target: W,
    options: XmlParseOptions,
    mut transform: F,
) -> Result<W>
where
    R: BufRead,
    W: Write,
    F: for<'event> FnMut(&Event<'event>) -> Result<XmlTransformAction>,
{
    let mut reader = XmlEventReader::new(source, options);
    let mut writer = XmlEventWriter::new(target);
    loop {
        let event = reader.read_event()?;
        if matches!(event, Event::Eof) {
            return Ok(writer.into_inner());
        }
        if transform(&event)? == XmlTransformAction::Keep {
            writer.write_event(event.borrow())?;
        }
    }
}

fn validate_event(
    event: &Event<'_>,
    options: &XmlParseOptions,
    state: &mut ParseState,
) -> Result<()> {
    match event {
        Event::Decl(declaration) => {
            let version = declaration
                .version()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            state.version = if version.as_ref() == b"1.1" {
                XmlVersion::Explicit1_1
            } else {
                XmlVersion::Explicit1_0
            };
        }
        Event::Start(start) => {
            begin_element(start, options, state)?;
            state.depth += 1;
            if state.depth > options.max_depth {
                return Err(CoreError::XmlLimit {
                    resource: "depth",
                    max: options.max_depth,
                });
            }
        }
        Event::Empty(start) => {
            begin_element(start, options, state)?;
            if state.depth.saturating_add(1) > options.max_depth {
                return Err(CoreError::XmlLimit {
                    resource: "depth",
                    max: options.max_depth,
                });
            }
            if state.depth == 0 {
                state.root_closed = true;
            }
        }
        Event::End(_) => {
            if state.depth == 0 {
                return Err(CoreError::Xml(
                    "closing tag outside the root element".to_owned(),
                ));
            }
            state.depth -= 1;
            if state.depth == 0 {
                state.root_closed = true;
            }
        }
        Event::Text(text) => {
            let value = text
                .decode()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            validate_text(value.as_ref(), options, state)?;
        }
        Event::CData(text) => {
            let value = text
                .decode()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            validate_text(value.as_ref(), options, state)?;
        }
        Event::DocType(_) if !options.allow_doctype => {
            return Err(CoreError::XmlForbidden("DOCTYPE"));
        }
        Event::GeneralRef(reference) => {
            let value = resolve_reference(reference, options)?;
            validate_text(&value, options, state)?;
        }
        Event::Eof => {
            if state.depth != 0 {
                return Err(CoreError::Xml(
                    "unexpected EOF inside an element".to_owned(),
                ));
            }
            if !state.root_seen {
                return Err(CoreError::Xml("missing root element".to_owned()));
            }
        }
        Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
    }
    Ok(())
}

fn begin_element(
    start: &BytesStart<'_>,
    options: &XmlParseOptions,
    state: &mut ParseState,
) -> Result<()> {
    if state.depth == 0 {
        if state.root_seen || state.root_closed {
            return Err(CoreError::Xml("multiple root elements".to_owned()));
        }
        state.root_seen = true;
    }
    state.nodes += 1;
    if state.nodes > options.max_nodes {
        return Err(CoreError::XmlLimit {
            resource: "node count",
            max: options.max_nodes,
        });
    }
    validate_attributes(start, options, state.version)?;
    Ok(())
}

fn validate_attributes(
    start: &BytesStart<'_>,
    options: &XmlParseOptions,
    version: XmlVersion,
) -> Result<()> {
    let mut count = 0;
    for attribute in start.attributes() {
        count += 1;
        if count > options.max_attributes_per_element {
            return Err(CoreError::XmlLimit {
                resource: "attributes per element",
                max: options.max_attributes_per_element,
            });
        }
        let attribute = attribute.map_err(|error| CoreError::Xml(error.to_string()))?;
        let value = attribute
            .decoded_and_normalized_value(version, start.decoder())
            .map_err(|error| CoreError::Xml(error.to_string()))?;
        validate_xml_chars(value.as_ref())?;
    }
    Ok(())
}

fn validate_text(value: &str, options: &XmlParseOptions, state: &mut ParseState) -> Result<()> {
    validate_xml_chars(value)?;
    if state.depth == 0 && !value.trim().is_empty() {
        return Err(CoreError::Xml("text outside the root element".to_owned()));
    }
    state.text_bytes = state.text_bytes.saturating_add(value.len());
    if state.text_bytes > options.max_text_bytes {
        return Err(CoreError::XmlLimit {
            resource: "text bytes",
            max: options.max_text_bytes,
        });
    }
    Ok(())
}

pub(crate) fn element_name(event: &BytesStart<'_>, mode: NamespaceMode) -> Result<String> {
    decode_name(
        event.decoder(),
        event.name().as_ref(),
        event.local_name().as_ref(),
        mode,
    )
}

pub(crate) fn end_name(
    event: &BytesEnd<'_>,
    decoder: quick_xml::encoding::Decoder,
    mode: NamespaceMode,
) -> Result<String> {
    decode_name(
        decoder,
        event.name().as_ref(),
        event.local_name().as_ref(),
        mode,
    )
}

fn decode_name(
    decoder: quick_xml::encoding::Decoder,
    qualified: &[u8],
    local: &[u8],
    mode: NamespaceMode,
) -> Result<String> {
    let bytes = match mode {
        NamespaceMode::Preserve => qualified,
        NamespaceMode::LocalName => local,
    };
    decoder
        .decode(bytes)
        .map(|value| value.into_owned())
        .map_err(|error| CoreError::Xml(error.to_string()))
}

pub(crate) fn read_attributes(
    event: &BytesStart<'_>,
    mode: NamespaceMode,
    version: XmlVersion,
) -> Result<IndexMap<String, String>> {
    let mut attributes = IndexMap::new();
    for attribute in event.attributes() {
        let attribute = attribute.map_err(|error| CoreError::Xml(error.to_string()))?;
        let qualified = attribute.key.as_ref();
        let local = qualified
            .splitn(2, |byte| *byte == b':')
            .nth(1)
            .unwrap_or(qualified);
        let name_bytes = if qualified == b"xmlns" || qualified.starts_with(b"xmlns:") {
            qualified
        } else {
            match mode {
                NamespaceMode::Preserve => qualified,
                NamespaceMode::LocalName => local,
            }
        };
        let key = event
            .decoder()
            .decode(name_bytes)
            .map(|value| value.into_owned())
            .map_err(|error| CoreError::Xml(error.to_string()))?;
        let value = attribute
            .decoded_and_normalized_value(version, event.decoder())
            .map_err(|error| CoreError::Xml(error.to_string()))?
            .into_owned();
        validate_xml_chars(&value)?;
        attributes.insert(key, value);
    }
    Ok(attributes)
}

pub(crate) fn resolve_reference(
    reference: &BytesRef<'_>,
    options: &XmlParseOptions,
) -> Result<String> {
    if let Some(character) = reference
        .resolve_char_ref()
        .map_err(|error| CoreError::Xml(error.to_string()))?
    {
        validate_xml_chars(&character.to_string())?;
        return Ok(character.to_string());
    }
    let name = reference
        .decode()
        .map_err(|error| CoreError::Xml(error.to_string()))?;
    if let Some(value) = resolve_predefined_entity(name.as_ref()) {
        return Ok(value.to_owned());
    }
    if options.allow_general_references {
        Ok(format!("&{name};"))
    } else {
        Err(CoreError::XmlForbidden("unknown general reference"))
    }
}

pub(crate) fn validate_xml_chars(value: &str) -> Result<()> {
    if value.chars().all(is_valid_xml_char) {
        Ok(())
    } else {
        Err(CoreError::Xml("illegal XML character".to_owned()))
    }
}

pub(crate) fn is_valid_xml_char(character: char) -> bool {
    matches!(
        character,
        '\u{9}'
            | '\u{A}'
            | '\u{D}'
            | '\u{20}'..='\u{D7FF}'
            | '\u{E000}'..='\u{FFFD}'
            | '\u{10000}'..='\u{10FFFF}'
    )
}

pub(crate) fn read_bounded_and_sanitize<R: BufRead>(
    source: R,
    options: &XmlParseOptions,
) -> Result<Vec<u8>> {
    let limit = u64::try_from(options.max_input_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::new();
    source
        .take(limit)
        .read_to_end(&mut bytes)
        .map_err(CoreError::Io)?;
    if bytes.len() > options.max_input_bytes {
        return Err(CoreError::XmlLimit {
            resource: "input bytes",
            max: options.max_input_bytes,
        });
    }
    let text = String::from_utf8(bytes).map_err(|error| CoreError::Xml(error.to_string()))?;
    Ok(text
        .chars()
        .filter(|character| is_valid_xml_char(*character))
        .collect::<String>()
        .into_bytes())
}
