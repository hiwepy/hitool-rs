use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

/// Location-rich script failure corresponding to Hutool's runtime exception.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRuntimeException {
    message: String,
    file_name: Option<String>,
    line_number: Option<usize>,
    column_number: Option<usize>,
}

impl ScriptRuntimeException {
    /// Creates a message-only exception.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file_name: None,
            line_number: None,
            column_number: None,
        }
    }

    /// Creates an exception with source location.
    #[must_use]
    pub fn with_location(
        message: impl Into<String>,
        file_name: impl Into<String>,
        line_number: usize,
        column_number: Option<usize>,
    ) -> Self {
        Self {
            message: message.into(),
            file_name: Some(file_name.into()),
            line_number: Some(line_number),
            column_number,
        }
    }

    /// Formats sequential `{}` placeholders.
    #[must_use]
    pub fn formatted(template: &str, parameters: &[&dyn fmt::Display]) -> Self {
        let mut message = template.to_owned();
        for parameter in parameters {
            message = message.replacen("{}", &parameter.to_string(), 1);
        }
        Self::new(message)
    }

    fn from_error(error: &ScriptError, file_name: Option<&str>) -> Self {
        let (line_number, column_number) = match &error {
            ScriptError::Evaluation(error) => position(error.position()),
            ScriptError::Compilation(error) => position(error.position()),
            _ => (None, None),
        };
        Self {
            message: error.to_string(),
            file_name: file_name.map(str::to_owned),
            line_number,
            column_number,
        }
    }

    /// Returns the base message without appended location text.
    #[must_use]
    pub fn base_message(&self) -> &str {
        &self.message
    }

    /// Returns the one-based line number when available.
    #[must_use]
    pub const fn line_number(&self) -> Option<usize> {
        self.line_number
    }

    /// Returns the one-based column number when available.
    #[must_use]
    pub const fn column_number(&self) -> Option<usize> {
        self.column_number
    }

    /// Returns the source file name when available.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }
}

impl fmt::Display for ScriptRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rendered = self.message.clone();
        if let Some(file_name) = &self.file_name {
            rendered.push_str(" in ");
            rendered.push_str(file_name);
            if let Some(line) = self.line_number {
                rendered.push_str(" at line number ");
                rendered.push_str(&line.to_string());
            }
            if let Some(column) = self.column_number {
                rendered.push_str(" at column number ");
                rendered.push_str(&column.to_string());
            }
        }
        formatter.write_str(&rendered)
    }
}

impl std::error::Error for ScriptRuntimeException {}

fn position(position: rhai::Position) -> (Option<usize>, Option<usize>) {
    (position.line(), position.position())
}
