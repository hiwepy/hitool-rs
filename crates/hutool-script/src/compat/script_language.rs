use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

/// Script languages recognized by the compatibility facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptLanguage {
    /// Sandboxed Rhai, also used for the JavaScript-shaped compatibility facade.
    Rhai,
    /// Optional Python engine; not linked by the base crate.
    Python,
    /// Optional Lua engine; not linked by the base crate.
    Lua,
    /// Optional Groovy engine; not linked by the base crate.
    Groovy,
}

impl ScriptLanguage {
    fn resolve(value: &str) -> Result<Self, ScriptError> {
        match value.to_ascii_lowercase().as_str() {
            "rhai" | "js" | "javascript" | "application/javascript" | "text/javascript" => {
                Ok(Self::Rhai)
            }
            "python" | "py" => Ok(Self::Python),
            "lua" => Ok(Self::Lua),
            "groovy" => Ok(Self::Groovy),
            _ => Err(ScriptError::UnsupportedLanguage(value.to_owned())),
        }
    }
}
