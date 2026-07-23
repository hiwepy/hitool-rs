use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

/// Defensive parser options corresponding to Hutool's `ParseConfig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseConfig {
    keep_strings: bool,
    max_nesting_depth: usize,
    max_input_bytes: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            keep_strings: false,
            max_nesting_depth: 128,
            max_input_bytes: 16 * 1024 * 1024,
        }
    }
}

impl ParseConfig {
    /// Creates production defaults.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Returns whether scalar XML values remain strings.
    #[must_use]
    pub const fn is_keep_strings(&self) -> bool {
        self.keep_strings
    }

    /// Configures scalar XML conversion.
    pub const fn set_keep_strings(&mut self, value: bool) -> &mut Self {
        self.keep_strings = value;
        self
    }

    /// Returns the maximum structured nesting depth.
    #[must_use]
    pub const fn max_nesting_depth(&self) -> usize {
        self.max_nesting_depth
    }

    /// Replaces the structured nesting limit.
    pub const fn set_max_nesting_depth(&mut self, value: usize) -> &mut Self {
        self.max_nesting_depth = value;
        self
    }

    /// Returns the maximum encoded input size.
    #[must_use]
    pub const fn max_input_bytes(&self) -> usize {
        self.max_input_bytes
    }

    /// Replaces the encoded input limit.
    pub const fn set_max_input_bytes(&mut self, value: usize) -> &mut Self {
        self.max_input_bytes = value;
        self
    }

    pub(crate) fn validate(&self, input: &str) -> Result<()> {
        if input.len() > self.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let mut depth = 0_usize;
        let mut quoted = false;
        let mut escaped = false;
        for character in input.chars() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    quoted = false;
                }
                continue;
            }
            match character {
                '"' => quoted = true,
                '{' | '[' => {
                    depth = depth.saturating_add(1);
                    if depth > self.max_nesting_depth {
                        return Err(JsonError::Limit("nesting depth"));
                    }
                }
                '}' | ']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        Ok(())
    }
}

/// Stateful Unicode tokenizer retaining Hutool's migration surface.
#[derive(Debug, Clone)]
pub struct JSONTokener {
    input: String,
    position: usize,
    previous: Option<char>,
    config: JSONConfig,
}

impl JSONTokener {
    /// Creates a tokenizer with default defensive limits.
    pub fn new(input: &str, config: JSONConfig) -> Result<Self> {
        Self::with_limits(input, config, ParseConfig::default())
    }

    /// Creates a tokenizer with explicit defensive limits.
    pub fn with_limits(input: &str, config: JSONConfig, limits: ParseConfig) -> Result<Self> {
        limits.validate(input)?;
        Ok(Self {
            input: input.to_owned(),
            position: 0,
            previous: None,
            config,
        })
    }

    /// Reads bounded UTF-8 bytes into a tokenizer.
    pub fn from_reader(
        reader: &mut dyn Read,
        config: JSONConfig,
        limits: ParseConfig,
    ) -> Result<Self> {
        let mut bytes = Vec::new();
        reader
            .take(limits.max_input_bytes as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > limits.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let input = String::from_utf8(bytes)?;
        Self::with_limits(&input, config, limits)
    }

    /// Returns the JSON container configuration.
    #[must_use]
    pub const fn config(&self) -> &JSONConfig {
        &self.config
    }

    /// Returns whether all input is consumed.
    #[must_use]
    pub fn end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Returns whether another character is available.
    #[must_use]
    pub fn more(&self) -> bool {
        !self.end()
    }

    /// Returns the next Unicode scalar.
    pub fn next_char(&mut self) -> Option<char> {
        let character = self.input[self.position..].chars().next()?;
        self.position += character.len_utf8();
        self.previous = Some(character);
        Some(character)
    }

    /// Moves back one previously read scalar.
    pub fn back(&mut self) -> Result<()> {
        let previous = self
            .previous
            .take()
            .ok_or_else(|| JsonError::Syntax("cannot step back twice".into()))?;
        self.position = self.position.saturating_sub(previous.len_utf8());
        Ok(())
    }

    /// Requires the next scalar to equal `expected`.
    pub fn next_expected(&mut self, expected: char) -> Result<char> {
        let Some(actual) = self.next_char() else {
            return Err(JsonError::Syntax(format!("expected {expected}, found end")));
        };
        if actual != expected {
            return Err(JsonError::Syntax(format!(
                "expected {expected}, found {actual}"
            )));
        }
        Ok(actual)
    }

    /// Reads exactly `count` Unicode scalars.
    pub fn next_n(&mut self, count: usize) -> Result<String> {
        let mut output = String::new();
        for _ in 0..count {
            output.push(
                self.next_char()
                    .ok_or_else(|| JsonError::Syntax("unexpected end".into()))?,
            );
        }
        Ok(output)
    }

    /// Skips whitespace and returns the next scalar.
    pub fn next_clean(&mut self) -> Option<char> {
        loop {
            let character = self.next_char()?;
            if !character.is_whitespace() {
                return Some(character);
            }
        }
    }

    /// Reads a quoted JSON string after its opening quote was consumed.
    pub fn next_string(&mut self, quote: char) -> Result<String> {
        let mut output = String::new();
        while let Some(character) = self.next_char() {
            if character == quote {
                return Ok(output);
            }
            if character != '\\' {
                output.push(character);
                continue;
            }
            let escaped = self
                .next_char()
                .ok_or_else(|| JsonError::Syntax("unterminated escape".into()))?;
            match escaped {
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{c}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '"' | '\\' | '/' => output.push(escaped),
                _ => return Err(JsonError::Syntax("unsupported escape".into())),
            }
        }
        Err(JsonError::Syntax("unterminated string".into()))
    }

    /// Reads text until one delimiter without consuming it.
    #[must_use]
    pub fn next_to(&mut self, delimiters: &str) -> String {
        let start = self.position;
        while let Some(character) = self.next_char() {
            if delimiters.contains(character) {
                let end = self.position - character.len_utf8();
                self.position = end;
                self.previous = None;
                return self.input[start..end].trim().to_owned();
            }
        }
        self.input[start..].trim().to_owned()
    }

    /// Skips through and including a marker.
    pub fn skip_past(&mut self, marker: &str) -> bool {
        if let Some(offset) = self.input[self.position..].find(marker) {
            self.position += offset + marker.len();
            self.previous = None;
            true
        } else {
            self.position = self.input.len();
            false
        }
    }

    /// Skips to a scalar without consuming it.
    pub fn skip_to(&mut self, target: char) -> Option<char> {
        let offset = self.input[self.position..].find(target)?;
        self.position += offset;
        self.previous = None;
        Some(target)
    }

    /// Parses the remaining input as one complete JSON value.
    pub fn next_value(&mut self) -> Result<Value> {
        let remaining = self.input[self.position..].trim();
        let value = crate::parse(remaining)?;
        self.position = self.input.len();
        self.previous = None;
        Ok(value)
    }

    /// Parses the remaining input as an array.
    pub fn to_array(&mut self) -> Result<JSONArray> {
        JSONArray::from_value(self.next_value()?, self.config.clone())
    }
}

/// Parser facade that creates configured containers.
#[derive(Debug, Clone)]
pub struct JSONParser {
    tokener: JSONTokener,
}

impl JSONParser {
    /// Creates a parser around an owned tokenizer.
    #[must_use]
    pub const fn new(tokener: JSONTokener) -> Self {
        Self { tokener }
    }

    /// Parses an object.
    pub fn parse_object(&mut self) -> Result<JSONObject> {
        let config = self.tokener.config.clone();
        JSONObject::from_value(self.tokener.next_value()?, config)
    }

    /// Parses an array.
    pub fn parse_array(&mut self) -> Result<JSONArray> {
        let config = self.tokener.config.clone();
        JSONArray::from_value(self.tokener.next_value()?, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    struct FailingReader;
    impl Read for FailingReader {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("injected"))
        }
    }

    #[test]
    fn limits_cover_depth_bytes_quotes_and_mutation() {
        let mut limits = ParseConfig::create();
        assert!(!limits.is_keep_strings());
        assert_eq!(limits.max_nesting_depth(), 128);
        assert_eq!(limits.max_input_bytes(), 16 * 1024 * 1024);
        limits
            .set_keep_strings(true)
            .set_max_nesting_depth(2)
            .set_max_input_bytes(32);
        assert!(limits.is_keep_strings());
        assert!(limits.validate(r#"{"value":"{{", "x":[]}"#).is_ok());
        assert!(limits.validate("[[[]]]").is_err());
        assert!(limits.validate(&"x".repeat(33)).is_err());
        limits.set_max_input_bytes(1);
        assert!(JSONTokener::with_limits("xx", JSONConfig::default(), limits).is_err());
    }

    #[test]
    fn tokener_navigates_unicode_strings_and_delimiters() {
        let mut tokener =
            JSONTokener::new(" λ,\"line\\ntext\" tail:end", JSONConfig::default()).unwrap();
        assert!(tokener.more());
        assert_eq!(tokener.next_clean(), Some('λ'));
        tokener.back().unwrap();
        assert_eq!(tokener.next_n(1).unwrap(), "λ");
        assert_eq!(tokener.next_expected(',').unwrap(), ',');
        assert!(tokener.next_expected('!').is_err());
        tokener.back().unwrap();
        assert_eq!(tokener.next_expected('"').unwrap(), '"');
        assert_eq!(tokener.next_string('"').unwrap(), "line\ntext");
        assert_eq!(tokener.next_to(":"), "tail");
        assert_eq!(tokener.next_char(), Some(':'));
        assert_eq!(tokener.skip_to('e'), Some('e'));
        assert!(tokener.skip_past("end"));
        assert!(tokener.end());
        assert!(!tokener.more());
        assert!(!tokener.skip_past("missing"));
        assert_eq!(tokener.skip_to('x'), None);
        assert!(tokener.back().is_err());
        assert!(tokener.next_n(1).is_err());
        let mut empty = JSONTokener::new("", JSONConfig::default()).unwrap();
        assert!(empty.next_expected('x').is_err());
        assert_eq!(empty.next_clean(), None);
    }

    #[test]
    fn readers_parsers_and_string_errors_are_explicit() {
        let limits = ParseConfig {
            max_input_bytes: 8,
            ..ParseConfig::default()
        };
        let mut reader = Cursor::new("[1,2]");
        let mut tokener =
            JSONTokener::from_reader(&mut reader, JSONConfig::default(), limits).unwrap();
        assert_eq!(tokener.config(), &JSONConfig::default());
        assert_eq!(tokener.to_array().unwrap()[1], 2);
        let mut large = Cursor::new("123456789");
        assert!(JSONTokener::from_reader(&mut large, JSONConfig::default(), limits).is_err());
        assert!(
            JSONTokener::from_reader(&mut FailingReader, JSONConfig::default(), limits).is_err()
        );
        let mut invalid = Cursor::new(vec![0xff]);
        assert!(JSONTokener::from_reader(&mut invalid, JSONConfig::default(), limits).is_err());

        let tokener = JSONTokener::new(r#"{"a":1}"#, JSONConfig::default()).unwrap();
        assert_eq!(JSONParser::new(tokener).parse_object().unwrap()["a"], 1);
        let tokener = JSONTokener::new("[true]", JSONConfig::default()).unwrap();
        assert_eq!(JSONParser::new(tokener).parse_array().unwrap()[0], true);
        let mut tokener = JSONTokener::new("null", JSONConfig::default()).unwrap();
        assert!(tokener.to_array().is_err());
        let mut tokener = JSONTokener::new("\"unterminated", JSONConfig::default()).unwrap();
        tokener.next_char();
        assert!(tokener.next_string('"').is_err());
        let mut tokener = JSONTokener::new("\"bad\\x\"", JSONConfig::default()).unwrap();
        tokener.next_char();
        assert!(tokener.next_string('"').is_err());
        let mut tokener = JSONTokener::new("\"bad\\", JSONConfig::default()).unwrap();
        tokener.next_char();
        assert!(tokener.next_string('"').is_err());
        let mut tokener =
            JSONTokener::new("\"\\b\\f\\r\\t\\\"\\\\\\/\"", JSONConfig::default()).unwrap();
        tokener.next_char();
        assert_eq!(tokener.next_string('"').unwrap(), "\u{8}\u{c}\r\t\"\\/");
        let mut tokener = JSONTokener::new("tail", JSONConfig::default()).unwrap();
        assert_eq!(tokener.next_to(","), "tail");

        let mut invalid = JSONTokener::new("{", JSONConfig::default()).unwrap();
        assert!(invalid.next_value().is_err());
        let mut invalid = JSONTokener::new("[", JSONConfig::default()).unwrap();
        assert!(invalid.to_array().is_err());
        let tokener = JSONTokener::new("{", JSONConfig::default()).unwrap();
        assert!(JSONParser::new(tokener).parse_object().is_err());
        let tokener = JSONTokener::new("[", JSONConfig::default()).unwrap();
        assert!(JSONParser::new(tokener).parse_array().is_err());
    }
}
