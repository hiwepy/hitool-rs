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

mod namespace_mode;
mod xml_parse_options;
mod xml_event_reader;
mod xml_event_writer;
mod xml_transform_action;

pub use namespace_mode::NamespaceMode;
pub use xml_parse_options::XmlParseOptions;
pub use xml_event_reader::XmlEventReader;
pub use xml_event_writer::XmlEventWriter;
pub use xml_transform_action::XmlTransformAction;
pub use namespace_mode::visit_xml;
pub use namespace_mode::transform_xml;
