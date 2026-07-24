use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::full_support_script_engine::FullSupportScriptEngine;

/// JavaScript-named compatibility wrapper backed by sandboxed Rhai.
pub struct JavaScriptEngine(FullSupportScriptEngine);

impl JavaScriptEngine {
    /// Creates a fresh constrained engine.
    #[must_use]
    pub fn new() -> Self {
        Self(FullSupportScriptEngine::from_engine(ScriptEngine::default()))
    }

    /// Creates a fresh instance, mirroring Hutool's non-singleton method.
    #[must_use]
    pub fn instance() -> Self {
        Self::new()
    }
}

impl Default for JavaScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for JavaScriptEngine {
    type Target = FullSupportScriptEngine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JavaScriptEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
