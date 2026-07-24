use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

/// Metadata for the built-in constrained engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptEngineFactory;

impl ScriptEngineFactory {
    /// Engine display name.
    #[must_use]
    pub const fn engine_name(self) -> &'static str {
        "HiTool Rhai"
    }

    /// Language name.
    #[must_use]
    pub const fn language_name(self) -> &'static str {
        "Rhai"
    }

    /// Accepted aliases.
    #[must_use]
    pub const fn names(self) -> &'static [&'static str] {
        &["rhai", "js", "javascript"]
    }
}
