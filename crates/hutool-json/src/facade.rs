use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// Static Hutool-compatible convenience facade.
pub struct JSONUtil;

impl JSONUtil {
    /// Creates an empty object.
    #[must_use]
    pub fn create_obj() -> JSONObject {
        JSONObject::new()
    }

    /// Creates an empty configured object.
    #[must_use]
    pub fn create_obj_with(config: JSONConfig) -> JSONObject {
        JSONObject::with_config(config)
    }

    /// Creates an empty array.
    #[must_use]
    pub fn create_array() -> JSONArray {
        JSONArray::new()
    }

    /// Creates an empty configured array.
    #[must_use]
    pub fn create_array_with(config: JSONConfig) -> JSONArray {
        JSONArray::with_config(config)
    }

    /// Parses an object.
    pub fn parse_obj(input: &str) -> Result<JSONObject> {
        JSONObject::parse(input)
    }

    /// Converts a serializable value to an object.
    pub fn object_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONObject::from_value(serde_json::to_value(value)?, config)
    }

    /// Parses an array.
    pub fn parse_array(input: &str) -> Result<JSONArray> {
        JSONArray::parse(input)
    }

    /// Converts a serializable value to an array.
    pub fn array_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONArray::from_value(serde_json::to_value(value)?, config)
    }

    /// Parses any JSON value.
    pub fn parse(input: &str) -> Result<Value> {
        crate::parse(input)
    }

    /// Serializes a value compactly.
    pub fn to_json_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string(value)
    }

    /// Serializes a value with indentation.
    pub fn to_pretty_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string_pretty(value)
    }

    /// Deserializes a typed Rust value.
    pub fn to_bean<T: DeserializeOwned>(input: &str) -> Result<T> {
        crate::from_str(input)
    }

    /// Deserializes every array element to a typed Rust value.
    pub fn to_list<T: DeserializeOwned>(array: &JSONArray) -> Result<Vec<T>> {
        Ok(serde_json::from_value(array.to_value())?)
    }

    /// Borrows a value at a JSON path.
    #[must_use]
    pub fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        get_by_path(value, path)
    }

    /// Writes a value at a JSON path.
    pub fn put_by_path(value: &mut Value, path: &str, replacement: Value) -> Result<()> {
        put_by_path(value, path, replacement)
    }

    /// Quotes a JSON string.
    #[must_use]
    pub fn quote(value: &str) -> String {
        Value::String(value.to_owned()).to_string()
    }

    /// Escapes a string without surrounding quotes.
    #[must_use]
    pub fn escape(value: &str) -> String {
        let quoted = Self::quote(value);
        quoted[1..quoted.len() - 1].to_owned()
    }

    /// Formats valid JSON with indentation.
    pub fn format_json_str(value: &str) -> Result<String> {
        crate::pretty(value)
    }

    /// Returns whether the complete input is JSON.
    #[must_use]
    pub fn is_json(value: &str) -> bool {
        crate::is_valid(value)
    }

    /// Returns whether the complete input is an object.
    #[must_use]
    pub fn is_json_obj(value: &str) -> bool {
        crate::is_json_object(value)
    }

    /// Returns whether the complete input is an array.
    #[must_use]
    pub fn is_json_array(value: &str) -> bool {
        crate::is_json_array(value)
    }

    /// Returns whether a dynamic value is JSON null.
    #[must_use]
    pub fn is_null(value: &Value) -> bool {
        value.is_null()
    }
}

/// Pretty-formatting facade corresponding to Hutool's `JSONStrFormatter`.
pub struct JSONStrFormatter;

impl JSONStrFormatter {
    /// Formats one complete JSON document.
    pub fn format(input: &str) -> Result<String> {
        crate::pretty(input)
    }
}

/// Serde-backed support mixed into application types.
pub trait JSONSupport: Serialize + DeserializeOwned + Sized {
    /// Parses one instance.
    fn parse(input: &str) -> Result<Self> {
        crate::from_str(input)
    }

    /// Converts this value to a dynamic JSON value.
    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }

    /// Serializes this value compactly.
    fn to_json_string(&self) -> Result<String> {
        crate::to_string(self)
    }

    /// Serializes this value with indentation.
    fn to_pretty_string(&self) -> Result<String> {
        crate::to_string_pretty(self)
    }
}

impl<T: Serialize + DeserializeOwned> JSONSupport for T {}

/// Dynamic JSON conversion helper.
pub struct JSONConverter;

impl JSONConverter {
    /// Returns an object or array wrapper matching the dynamic shape.
    pub fn convert(value: Value, config: JSONConfig) -> Result<Box<dyn JsonContainerObject>> {
        match value {
            Value::Object(entries) => Ok(Box::new(JSONObject::from_entries(entries, config))),
            Value::Array(values) => Ok(Box::new(JSONArray::from_values(values, config))),
            Value::Null => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "null",
            }),
            Value::Bool(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "boolean",
            }),
            Value::Number(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "number",
            }),
            Value::String(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "string",
            }),
        }
    }
}

/// Object-safe view used by [`JSONConverter`].
pub trait JsonContainerObject: std::fmt::Display + Send + Sync {
    /// Returns an owned dynamic representation.
    fn to_dynamic(&self) -> Value;
}

impl JsonContainerObject for JSONObject {
    fn to_dynamic(&self) -> Value {
        self.to_value()
    }
}

impl JsonContainerObject for JSONArray {
    fn to_dynamic(&self) -> Value {
        self.to_value()
    }
}

/// Serde-backed object mapper.
pub struct ObjectMapper;

impl ObjectMapper {
    /// Maps a serializable value to a configured object.
    pub fn to_object<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONUtil::object_from(value, config)
    }

    /// Maps a serializable value to a configured array.
    pub fn to_array<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONUtil::array_from(value, config)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WriterMode {
    Object,
    Array,
}

/// Streaming JSON writer with explicit ownership and error propagation.
pub struct JSONWriter<W: Write> {
    writer: W,
    mode: Option<WriterMode>,
    first: bool,
    pending_key: bool,
    config: JSONConfig,
}

impl<W: Write> JSONWriter<W> {
    /// Creates a writer around an owned destination.
    #[must_use]
    pub const fn new(writer: W, config: JSONConfig) -> Self {
        Self {
            writer,
            mode: None,
            first: true,
            pending_key: false,
            config,
        }
    }

    /// Starts an object.
    pub fn begin_obj(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"{")?;
        self.mode = Some(WriterMode::Object);
        Ok(self)
    }

    /// Starts an array.
    pub fn begin_array(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"[")?;
        self.mode = Some(WriterMode::Array);
        Ok(self)
    }

    /// Writes an object key.
    pub fn write_key(&mut self, key: &str) -> Result<&mut Self> {
        if self.mode != Some(WriterMode::Object) || self.pending_key {
            return Err(JsonError::Syntax(
                "key outside object or without value".into(),
            ));
        }
        self.separator()?;
        write!(self.writer, "{}:", JSONUtil::quote(key))?;
        self.pending_key = true;
        Ok(self)
    }

    /// Writes one array element or the value for the pending object key.
    pub fn write_value(&mut self, value: &Value) -> Result<&mut Self> {
        match self.mode {
            Some(WriterMode::Object) if !self.pending_key => {
                return Err(JsonError::Syntax("object value requires a key".into()));
            }
            Some(WriterMode::Array) => self.separator()?,
            Some(WriterMode::Object) => self.pending_key = false,
            None => return Err(JsonError::Syntax("value outside container".into())),
        }
        let value = normalize_writer_value(value, &self.config);
        serde_json::to_writer(&mut self.writer, &value)?;
        Ok(self)
    }

    /// Writes one object field, respecting null omission.
    pub fn write_field(&mut self, key: &str, value: &Value) -> Result<&mut Self> {
        if self.config.is_ignore_null_value() && value.is_null() {
            return Ok(self);
        }
        self.write_key(key)?.write_value(value)
    }

    /// Ends the active container and flushes the destination.
    pub fn end(&mut self) -> Result<&mut Self> {
        if self.pending_key {
            return Err(JsonError::Syntax("object key has no value".into()));
        }
        let closing = match self.mode.take() {
            Some(WriterMode::Object) => b'}',
            Some(WriterMode::Array) => b']',
            None => return Err(JsonError::Syntax("no active container".into())),
        };
        self.writer.write_all(&[closing])?;
        self.writer.flush()?;
        Ok(self)
    }

    /// Returns the owned destination.
    #[must_use]
    pub fn into_inner(self) -> W {
        self.writer
    }

    fn separator(&mut self) -> Result<()> {
        if self.first {
            self.first = false;
        } else {
            self.writer.write_all(b",")?;
        }
        Ok(())
    }
}

fn normalize_writer_value(value: &Value, config: &JSONConfig) -> Value {
    if config.is_write_long_as_string() {
        if let Value::Number(number) = value {
            if number.as_i64().is_some() || number.as_u64().is_some() {
                return Value::String(number.to_string());
            }
        }
    }
    value.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::io::{self, Write};

    enum FlexibleOutput {
        Object,
        Array,
        Error,
    }

    struct FlexibleSerialize(FlexibleOutput);

    impl Serialize for FlexibleSerialize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self.0 {
                FlexibleOutput::Object => json!({"value": 2}).serialize(serializer),
                FlexibleOutput::Array => json!([1, 2]).serialize(serializer),
                FlexibleOutput::Error => Err(serde::ser::Error::custom(
                    "intentional serialization failure",
                )),
            }
        }
    }

    #[derive(Deserialize)]
    struct FlexibleSupport(bool);

    impl Serialize for FlexibleSupport {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.0 {
                Err(serde::ser::Error::custom(
                    "intentional serialization failure",
                ))
            } else {
                json!({"value": 5}).serialize(serializer)
            }
        }
    }

    #[derive(Default)]
    struct TestWriter {
        bytes: Vec<u8>,
        byte: Option<u8>,
        fail_flush: bool,
    }

    impl TestWriter {
        const fn byte(byte: u8) -> Self {
            Self {
                bytes: Vec::new(),
                byte: Some(byte),
                fail_flush: false,
            }
        }

        const fn flush() -> Self {
            Self {
                bytes: Vec::new(),
                byte: None,
                fail_flush: true,
            }
        }
    }

    impl Write for TestWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.byte.is_some_and(|byte| bytes.contains(&byte)) {
                Err(io::Error::other("intentional write failure"))
            } else {
                self.bytes.extend_from_slice(bytes);
                Ok(bytes.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                Err(io::Error::other("intentional flush failure"))
            } else {
                Ok(())
            }
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Item {
        value: u8,
    }

    #[test]
    fn util_support_converter_and_mapper_delegate_to_serde() {
        assert!(JSONUtil::create_obj().is_empty());
        assert!(JSONUtil::create_array().is_empty());
        assert!(JSONUtil::create_obj_with(JSONConfig::default()).is_empty());
        assert!(JSONUtil::create_array_with(JSONConfig::default()).is_empty());
        assert_eq!(JSONUtil::parse_obj(r#"{"a":1}"#).unwrap()["a"], 1);
        assert_eq!(JSONUtil::parse_array("[1]").unwrap()[0], 1);
        assert_eq!(JSONUtil::parse("true").unwrap(), true);
        assert_eq!(
            JSONUtil::object_from(
                &FlexibleSerialize(FlexibleOutput::Object),
                JSONConfig::default(),
            )
            .unwrap()["value"],
            2
        );
        assert_eq!(
            JSONUtil::array_from(
                &FlexibleSerialize(FlexibleOutput::Array),
                JSONConfig::default(),
            )
            .unwrap()[1],
            2
        );
        assert_eq!(
            ObjectMapper::to_object(
                &FlexibleSerialize(FlexibleOutput::Object),
                JSONConfig::default(),
            )
            .unwrap()["value"],
            2
        );
        assert_eq!(
            ObjectMapper::to_array(
                &FlexibleSerialize(FlexibleOutput::Array),
                JSONConfig::default(),
            )
            .unwrap()[0],
            1
        );
        let encoded = JSONUtil::to_json_string(&Item { value: 4 }).unwrap();
        assert_eq!(JSONUtil::to_bean::<Item>(&encoded).unwrap().value, 4);
        assert!(
            JSONUtil::to_pretty_string(&json!({"a": 1}))
                .unwrap()
                .contains('\n')
        );
        assert_eq!(
            JSONUtil::to_list::<u8>(&JSONArray::parse("[1,2]").unwrap()).unwrap(),
            vec![1, 2]
        );
        let mut value = json!({});
        JSONUtil::put_by_path(&mut value, "a[0]", json!(7)).unwrap();
        assert_eq!(JSONUtil::get_by_path(&value, "a[0]"), Some(&json!(7)));
        assert_eq!(JSONUtil::quote("a\n"), "\"a\\n\"");
        assert_eq!(JSONUtil::escape("a\n"), "a\\n");
        assert!(JSONUtil::format_json_str("[1]").unwrap().contains('\n'));
        assert_eq!(JSONStrFormatter::format("{}").unwrap(), "{}");
        assert!(JSONUtil::is_json("null"));
        assert!(JSONUtil::is_json_obj("{}"));
        assert!(JSONUtil::is_json_array("[]"));
        assert!(JSONUtil::is_null(&Value::Null));
        assert_eq!(Item::parse(&encoded).unwrap(), Item { value: 4 });
        let item = Item { value: 5 };
        assert_eq!(item.to_json_string().unwrap(), r#"{"value":5}"#);
        assert!(item.to_pretty_string().unwrap().contains('\n'));
        assert_eq!(FlexibleSupport(false).to_json().unwrap()["value"], 5);
        assert!(FlexibleSupport(true).to_json().is_err());
        let converted = JSONConverter::convert(json!({"x": 1}), JSONConfig::default()).unwrap();
        assert_eq!(converted.to_dynamic()["x"], 1);
        let converted = JSONConverter::convert(json!([1]), JSONConfig::default()).unwrap();
        assert_eq!(converted.to_dynamic()[0], 1);
        assert!(JSONConverter::convert(json!(1), JSONConfig::default()).is_err());
        for value in [Value::Null, json!(false), json!("x")] {
            assert!(JSONConverter::convert(value, JSONConfig::default()).is_err());
        }
        assert!(
            JSONUtil::object_from(
                &FlexibleSerialize(FlexibleOutput::Error),
                JSONConfig::default()
            )
            .is_err()
        );
        assert!(
            JSONUtil::array_from(
                &FlexibleSerialize(FlexibleOutput::Error),
                JSONConfig::default()
            )
            .is_err()
        );
        assert!(JSONUtil::to_list::<u8>(&JSONArray::parse("[true]").unwrap()).is_err());
    }

    #[test]
    fn writer_enforces_state_nulls_and_long_strings() {
        let mut config = JSONConfig::default();
        config
            .set_ignore_null_value(true)
            .set_write_long_as_string(true);
        let mut writer = JSONWriter::new(TestWriter::default(), config);
        writer.begin_obj().unwrap();
        writer.write_field("missing", &Value::Null).unwrap();
        writer.write_field("id", &json!(7)).unwrap();
        writer.write_field("name", &json!("hi")).unwrap();
        writer.end().unwrap();
        assert_eq!(
            String::from_utf8(writer.into_inner().bytes).unwrap(),
            r#"{"id":"7","name":"hi"}"#
        );

        let mut writer = JSONWriter::new(TestWriter::default(), JSONConfig::default());
        assert!(writer.write_value(&json!(1)).is_err());
        assert!(writer.end().is_err());
        writer.begin_array().unwrap();
        writer
            .write_value(&json!(1))
            .unwrap()
            .write_value(&json!(2.5))
            .unwrap();
        assert!(writer.write_key("bad").is_err());
        writer.end().unwrap();
        assert_eq!(
            String::from_utf8(writer.into_inner().bytes).unwrap(),
            "[1,2.5]"
        );

        let mut writer = JSONWriter::new(TestWriter::default(), JSONConfig::default());
        writer.begin_obj().unwrap().write_key("x").unwrap();
        assert!(writer.write_key("y").is_err());
        assert!(writer.end().is_err());
        writer.write_value(&json!(true)).unwrap().end().unwrap();

        let mut writer = JSONWriter::new(TestWriter::default(), JSONConfig::default());
        writer.begin_obj().unwrap();
        assert!(writer.write_value(&json!(1)).is_err());

        let mut config = JSONConfig::default();
        config.set_write_long_as_string(true);
        assert_eq!(normalize_writer_value(&json!(1.5), &config), json!(1.5));

        assert!(
            JSONWriter::new(TestWriter::byte(b'{'), JSONConfig::default())
                .begin_obj()
                .is_err()
        );
        assert!(
            JSONWriter::new(TestWriter::byte(b'['), JSONConfig::default())
                .begin_array()
                .is_err()
        );

        let mut writer = JSONWriter::new(TestWriter::byte(b':'), JSONConfig::default());
        writer.begin_obj().unwrap();
        assert!(writer.write_key("x").is_err());
        let mut writer = JSONWriter::new(TestWriter::byte(b':'), JSONConfig::default());
        writer.begin_obj().unwrap();
        assert!(writer.write_field("x", &json!(1)).is_err());

        let mut writer = JSONWriter::new(TestWriter::byte(b','), JSONConfig::default());
        writer
            .begin_obj()
            .unwrap()
            .write_field("x", &json!(1))
            .unwrap();
        assert!(writer.write_key("y").is_err());
        let mut writer = JSONWriter::new(TestWriter::byte(b','), JSONConfig::default());
        writer
            .begin_array()
            .unwrap()
            .write_value(&json!(1))
            .unwrap();
        assert!(writer.write_value(&json!(2)).is_err());

        let mut writer = JSONWriter::new(TestWriter::byte(b'1'), JSONConfig::default());
        writer.begin_array().unwrap();
        assert!(writer.write_value(&json!(1)).is_err());
        let mut writer = JSONWriter::new(TestWriter::byte(b'}'), JSONConfig::default());
        writer.begin_obj().unwrap();
        assert!(writer.end().is_err());
        let mut writer = JSONWriter::new(TestWriter::flush(), JSONConfig::default());
        writer.begin_array().unwrap();
        assert!(writer.end().is_err());
        let mut writer = JSONWriter::new(TestWriter::default(), JSONConfig::default());
        writer.begin_array().unwrap().end().unwrap();
    }
}
