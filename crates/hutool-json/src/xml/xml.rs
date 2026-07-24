use std::fmt::Write as _;

use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Number, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

/// XML conversion facade matching Hutool's `XML` utility.
pub struct XML;

impl XML {
    /// Parses XML with production defaults.
    pub fn to_json(input: &str) -> Result<JSONObject> {
        Self::to_json_with(input, ParseConfig::default())
    }

    /// Parses XML with explicit defensive limits.
    pub fn to_json_with(input: &str, config: ParseConfig) -> Result<JSONObject> {
        config.validate(input)?;
        let mut reader = Reader::from_str(input);
        reader.config_mut().trim_text(true);
        let mut stack: Vec<Element> = Vec::new();
        let mut root = Map::new();
        loop {
            match reader.read_event() {
                Ok(Event::Start(event)) => {
                    if stack.len() >= config.max_nesting_depth() {
                        return Err(JsonError::Limit("XML nesting depth"));
                    }
                    let name = String::from_utf8_lossy(event.name().as_ref()).into_owned();
                    let mut element = Element::new(name);
                    for attribute in event.attributes() {
                        let attribute = match attribute {
                            Ok(attribute) => attribute,
                            Err(error) => return Err(JsonError::Syntax(error.to_string())),
                        };
                        let key = format!("@{}", String::from_utf8_lossy(attribute.key.as_ref()));
                        let value = match attribute
                            .decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())
                        {
                            Ok(value) => value,
                            Err(error) => return Err(JsonError::Syntax(error.to_string())),
                        };
                        element
                            .fields
                            .insert(key, Value::String(value.into_owned()));
                    }
                    stack.push(element);
                }
                Ok(Event::Empty(event)) => {
                    if stack.len() >= config.max_nesting_depth() {
                        return Err(JsonError::Limit("XML nesting depth"));
                    }
                    let name = String::from_utf8_lossy(event.name().as_ref()).into_owned();
                    attach(&mut stack, &mut root, name, Value::String(String::new()));
                }
                Ok(Event::Text(event)) => {
                    if let Some(element) = stack.last_mut() {
                        element
                            .text
                            .push_str(&String::from_utf8_lossy(event.as_ref()));
                    }
                }
                Ok(Event::CData(event)) => {
                    if let Some(element) = stack.last_mut() {
                        element
                            .text
                            .push_str(&String::from_utf8_lossy(event.as_ref()));
                    }
                }
                Ok(Event::End(_)) => {
                    finish_element(&mut stack, &mut root, config.is_keep_strings());
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(error) => return Err(JsonError::Syntax(error.to_string())),
            }
        }
        if !stack.is_empty() {
            return Err(JsonError::Syntax("unclosed XML element".into()));
        }
        JSONObject::from_value(Value::Object(root), JSONConfig::default())
    }

    /// Serializes a dynamic value without an additional root tag.
    #[must_use]
    pub fn to_xml(value: &Value) -> String {
        let mut output = String::new();
        write_value(&mut output, None, value);
        output
    }

    /// Serializes a dynamic value inside an explicit root tag.
    #[must_use]
    pub fn to_xml_with_root(value: &Value, root: &str) -> String {
        let mut output = String::new();
        write_value(&mut output, Some(root), value);
        output
    }
}

impl Element {
    fn new(name: String) -> Self {
        Self {
            name,
            fields: Map::new(),
            text: String::new(),
        }
    }

    fn finish(mut self, keep_strings: bool) -> (String, Value) {
        let text = self.text.trim();
        let value = if self.fields.is_empty() {
            scalar(text, keep_strings)
        } else {
            if !text.is_empty() {
                self.fields
                    .insert("content".into(), scalar(text, keep_strings));
            }
            Value::Object(self.fields)
        };
        (self.name, value)
    }
}

fn attach(stack: &mut [Element], root: &mut Map<String, Value>, name: String, value: Value) {
    let fields = stack.last_mut().map_or(root, |parent| &mut parent.fields);
    match fields.remove(&name) {
        None => {
            fields.insert(name, value);
        }
        Some(Value::Array(mut values)) => {
            values.push(value);
            fields.insert(name, Value::Array(values));
        }
        Some(previous) => {
            fields.insert(name, Value::Array(vec![previous, value]));
        }
    }
}

fn scalar(text: &str, keep_strings: bool) -> Value {
    if keep_strings {
        return Value::String(text.to_owned());
    }
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        value => value
            .parse::<i64>()
            .map(Number::from)
            .map(Value::Number)
            .or_else(|_| {
                value
                    .parse::<f64>()
                    .ok()
                    .and_then(Number::from_f64)
                    .map(Value::Number)
                    .ok_or(())
            })
            .unwrap_or_else(|()| Value::String(value.to_owned())),
    }
}

fn write_value(output: &mut String, tag: Option<&str>, value: &Value) {
    if let Value::Array(values) = value {
        for value in values {
            write_value(output, tag, value);
        }
        return;
    }
    if let Some(tag) = tag {
        let _ = write!(output, "<{}>", escape_name(tag));
    }
    if let Value::Object(object) = value {
        for (key, value) in object {
            if !key.starts_with('@') && key != "content" {
                write_value(output, Some(key), value);
            } else if key == "content" {
                output.push_str(&escape_text(&display_scalar(value)));
            }
        }
    } else {
        output.push_str(&escape_text(&display_scalar(value)));
    }
    if let Some(tag) = tag {
        let _ = write!(output, "</{}>", escape_name(tag));
    }
}

fn finish_element(

struct Element {
    name: String,
    fields: Map<String, Value>,
    text: String,
}
