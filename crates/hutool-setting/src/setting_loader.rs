use crate::{GroupedMap, SettingError};
use encoding_rs::Encoding;
use std::io::{Read, Write};

/// Parser and writer for Hutool `.setting` syntax.
#[derive(Debug, Clone)]
pub struct SettingLoader {
    pub(crate) charset: &'static Encoding,
    pub(crate) use_variable: bool,
    pub(crate) variable_prefix: String,
    pub(crate) variable_suffix: String,
    pub(crate) assign_flag: char,
}

impl Default for SettingLoader {
    fn default() -> Self {
        Self::new(crate::DEFAULT_ENCODING, false)
    }
}

impl SettingLoader {
    /// Creates a loader.
    #[must_use]
    pub fn new(charset: &'static Encoding, use_variable: bool) -> Self {
        Self {
            charset,
            use_variable,
            variable_prefix: "${".into(),
            variable_suffix: "}".into(),
            assign_flag: '=',
        }
    }
    /// Changes the variable delimiters. A form such as `$(...)` is supported.
    pub fn set_var_regex(
        &mut self,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Result<&mut Self, SettingError> {
        self.set_var_delimiters(prefix.into(), suffix.into())
    }
    fn set_var_delimiters(
        &mut self,
        prefix: String,
        suffix: String,
    ) -> Result<&mut Self, SettingError> {
        if prefix.is_empty() || suffix.is_empty() {
            return Err(SettingError::Invalid(
                "variable delimiters must not be empty".into(),
            ));
        }
        self.variable_prefix = prefix;
        self.variable_suffix = suffix;
        Ok(self)
    }
    /// Changes the assignment character.
    pub fn set_assign_flag(&mut self, flag: char) -> &mut Self {
        self.assign_flag = flag;
        self
    }
    /// Parses a reader and atomically replaces the destination.
    pub fn load(
        &self,
        reader: &mut dyn Read,
        destination: &mut GroupedMap,
    ) -> Result<(), SettingError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        let (text, _, malformed) = self.charset.decode(&bytes);
        if malformed {
            return Err(SettingError::Decode(self.charset.name()));
        }
        let parsed = self.parse(&text)?;
        *destination = parsed;
        Ok(())
    }
    fn parse(&self, text: &str) -> Result<GroupedMap, SettingError> {
        // Strip UTF-8 BOM (U+FEFF) so `[group]` headers still parse — 对齐 IssueI7G34E.
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let mut result = GroupedMap::new();
        let mut group = String::new();
        for (index, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                group = line[1..line.len() - 1].trim().to_owned();
                continue;
            }
            let Some(split) = line.find(self.assign_flag) else {
                return Err(SettingError::Invalid(format!(
                    "line {} has no assignment",
                    index + 1
                )));
            };
            let key = line[..split].trim();
            if key.is_empty() {
                return Err(SettingError::Invalid(format!(
                    "line {} has a blank key",
                    index + 1
                )));
            }
            let mut value = line[split + self.assign_flag.len_utf8()..]
                .trim()
                .to_owned();
            if self.use_variable {
                value = self.expand(&value, &group, &result);
            }
            result.put(group.clone(), key, value);
        }
        Ok(result)
    }
    fn expand(&self, value: &str, group: &str, values: &GroupedMap) -> String {
        let mut output = value.to_owned();
        for _ in 0..64 {
            let Some(start) = output.find(&self.variable_prefix) else {
                break;
            };
            let key_start = start + self.variable_prefix.len();
            let Some(relative_end) = output[key_start..].find(&self.variable_suffix) else {
                break;
            };
            let end = key_start + relative_end;
            let key = &output[key_start..end];
            // Hutool order: same group → cross-group `group.key` → system/env;
            // also keep default-group fallback for existing Rust callers.
            let replacement = values
                .get(group, key)
                .map(str::to_owned)
                .or_else(|| {
                    key.split_once('.')
                        .and_then(|(g, k)| values.get(g, k).map(str::to_owned))
                })
                .or_else(|| values.get("", key).map(str::to_owned))
                .or_else(|| std::env::var(key).ok());
            let Some(replacement) = replacement else {
                break;
            };
            output.replace_range(start..end + self.variable_suffix.len(), &replacement);
        }
        output
    }
    /// Stores grouped values.
    ///
    /// Empty (default) groups are written as `[]`, matching Hutool `SettingLoader.store`.
    pub fn store(&self, values: &GroupedMap, writer: &mut dyn Write) -> Result<(), SettingError> {
        for group in values.groups() {
            writeln!(writer, "[{group}]")?;
            for (key, value) in values.entries(group) {
                writeln!(writer, "{key} {} {value}", self.assign_flag)?;
            }
        }
        Ok(())
    }
}
