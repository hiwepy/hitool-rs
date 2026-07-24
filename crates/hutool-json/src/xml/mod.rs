use std::fmt::Write as _;

use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Number, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

mod xml;
mod jsonxml_parser;
mod jsonxml_serializer;
mod xml_tokener;

pub use xml::XML;
pub use jsonxml_parser::JSONXMLParser;
pub use jsonxml_serializer::JSONXMLSerializer;
pub use xml_tokener::XMLTokener;
