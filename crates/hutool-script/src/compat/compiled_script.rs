use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::script_context::ScriptContext;
use super::script_runtime_exception::ScriptRuntimeException;

/// Compiled, reusable script AST.
#[derive(Debug, Clone)]
pub struct CompiledScript {
    ast: AST,
    source_name: Option<String>,
}

impl CompiledScript {
    /// Returns the compiled AST.
    #[must_use]
    pub const fn ast(&self) -> &AST {
        &self.ast
    }

    /// Evaluates this script using an engine and context.
    pub fn eval(
        &self,
        engine: &ScriptEngine,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let mut scope = context.scope();
        let result = engine
            .eval_ast_with_scope(&mut scope, &self.ast)
            .map_err(|error| {
                ScriptRuntimeException::from_error(&error, self.source_name.as_deref())
            })?;
        context.update_engine(&scope);
        Ok(result)
    }
}
