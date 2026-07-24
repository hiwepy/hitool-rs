use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

/// Bindings passed into a script scope.
#[derive(Debug, Clone, Default)]
pub struct Bindings {
    values: BTreeMap<String, Dynamic>,
}

impl Bindings {
    /// Creates empty bindings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts or replaces one dynamic value.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<Dynamic>,
    ) -> Option<Dynamic> {
        self.values.insert(name.into(), value.into())
    }

    /// Returns a bound value.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Dynamic> {
        self.values.get(name)
    }

    /// Removes a bound value.
    pub fn remove(&mut self, name: &str) -> Option<Dynamic> {
        self.values.remove(name)
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the binding set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn append_to_scope(&self, scope: &mut Scope<'_>) {
        for (name, value) in &self.values {
            if scope.contains(name) {
                scope.set_value(name, value.clone());
            } else {
                scope.push_dynamic(name.clone(), value.clone());
            }
        }
    }

    fn replace_from_scope(&mut self, scope: &Scope<'_>) {
        self.values = scope
            .iter_raw()
            .map(|(name, _, value)| (name.to_owned(), value.clone()))
            .collect();
    }
}
