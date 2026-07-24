use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::full_support_script_engine::FullSupportScriptEngine;
use super::script_runtime_exception::ScriptRuntimeException;

/// Rust replacement for Java interface proxies over invocable scripts.
pub struct ScriptInterface<'a> {
    engine: &'a mut FullSupportScriptEngine,
    this: Option<Dynamic>,
}

impl ScriptInterface<'_> {
    /// Invokes a function or a method when this interface is object-bound.
    pub fn invoke(
        &mut self,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        match &mut self.this {
            Some(this) => self.engine.invoke_method(this, name, args),
            None => self.engine.invoke_function(name, args),
        }
    }

    /// Returns the current bound object after method mutations.
    #[must_use]
    pub fn bound_this(&self) -> Option<&Dynamic> {
        self.this.as_ref()
    }
}
