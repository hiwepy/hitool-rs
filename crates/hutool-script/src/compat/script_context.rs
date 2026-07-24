use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::bindings::Bindings;
use super::script_scope::ScriptScope;

/// Explicit script context; no process-global bindings are used.
#[derive(Debug, Clone, Default)]
pub struct ScriptContext {
    engine: Bindings,
    global: Bindings,
    source_name: Option<String>,
}

impl ScriptContext {
    /// Creates an empty context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            engine: Bindings::new(),
            global: Bindings::new(),
            source_name: None,
        }
    }

    /// Returns bindings for one scope.
    #[must_use]
    pub fn bindings(&self, scope: ScriptScope) -> &Bindings {
        match scope {
            ScriptScope::Engine => &self.engine,
            ScriptScope::Global => &self.global,
        }
    }

    /// Returns mutable bindings for one scope.
    pub fn bindings_mut(&mut self, scope: ScriptScope) -> &mut Bindings {
        match scope {
            ScriptScope::Engine => &mut self.engine,
            ScriptScope::Global => &mut self.global,
        }
    }

    /// Replaces bindings for one scope.
    pub fn set_bindings(&mut self, scope: ScriptScope, bindings: Bindings) {
        *self.bindings_mut(scope) = bindings;
    }

    /// Sets a diagnostic source name.
    pub fn set_source_name(&mut self, source_name: impl Into<String>) {
        self.source_name = Some(source_name.into());
    }

    /// Returns the diagnostic source name.
    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    fn scope(&self) -> Scope<'static> {
        let mut scope = Scope::new();
        self.global.append_to_scope(&mut scope);
        self.engine.append_to_scope(&mut scope);
        scope
    }

    fn update_engine(&mut self, scope: &Scope<'_>) {
        self.engine.replace_from_scope(scope);
    }
}
