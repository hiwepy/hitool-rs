//! Sandboxed embedded scripting backed by Rhai.

#![forbid(unsafe_code)]

use rhai::Engine;
pub use rhai::{Dynamic, EvalAltResult, Scope};
use std::{any::Any, any::type_name};
use thiserror::Error;

/// Script evaluation failures.
#[derive(Debug, Error)]
pub enum ScriptError {
    /// The Rhai engine rejected or terminated the script.
    #[error(transparent)]
    Evaluation(#[from] Box<EvalAltResult>),
    /// The dynamic result did not match the requested Rust type.
    #[error("script result cannot be converted to `{0}`")]
    TypeMismatch(&'static str),
}

/// Resource limits applied to every script engine.
#[derive(Debug, Clone, Copy)]
pub struct ScriptLimits {
    /// Maximum expression operations before termination.
    pub max_operations: u64,
    /// Maximum nested function-call depth.
    pub max_call_levels: usize,
    /// Maximum expression depth.
    pub max_expression_depth: usize,
    /// Maximum string length in bytes.
    pub max_string_size: usize,
    /// Maximum array element count.
    pub max_array_size: usize,
    /// Maximum map entry count.
    pub max_map_size: usize,
}

impl Default for ScriptLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_call_levels: 32,
            max_expression_depth: 64,
            max_string_size: 1_048_576,
            max_array_size: 10_000,
            max_map_size: 10_000,
        }
    }
}

/// A constrained script engine. No filesystem, process, or network APIs are
/// registered by this crate.
pub struct ScriptEngine {
    inner: Engine,
}

impl ScriptEngine {
    /// Creates an engine with explicit resource limits and dynamic `eval`
    /// disabled.
    #[must_use]
    pub fn new(limits: ScriptLimits) -> Self {
        let mut inner = Engine::new();
        inner
            .set_max_operations(limits.max_operations)
            .set_max_call_levels(limits.max_call_levels)
            .set_max_expr_depths(limits.max_expression_depth, limits.max_expression_depth)
            .set_max_string_size(limits.max_string_size)
            .set_max_array_size(limits.max_array_size)
            .set_max_map_size(limits.max_map_size)
            .disable_symbol("eval");
        Self { inner }
    }

    /// Returns the underlying engine for deliberate function registration.
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.inner
    }

    /// Evaluates a script into a typed result.
    pub fn eval<T: Any>(&self, script: &str) -> Result<T, ScriptError> {
        self.inner
            .eval::<Dynamic>(script)?
            .try_cast::<T>()
            .ok_or_else(|| ScriptError::TypeMismatch(type_name::<T>()))
    }

    /// Evaluates a script using the supplied variable scope.
    pub fn eval_with_scope<T: Any>(
        &self,
        scope: &mut Scope<'_>,
        script: &str,
    ) -> Result<T, ScriptError> {
        self.inner
            .eval_with_scope::<Dynamic>(scope, script)?
            .try_cast::<T>()
            .ok_or_else(|| ScriptError::TypeMismatch(type_name::<T>()))
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new(ScriptLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_typed_expressions_and_rejects_eval() {
        let engine = ScriptEngine::default();
        assert_eq!(engine.eval::<i64>("40 + 2").unwrap(), 42);
        assert!(engine.eval::<Dynamic>(r#"eval("40 + 2")"#).is_err());
    }

    #[test]
    fn uses_typed_scope_values() {
        let engine = ScriptEngine::default();
        let mut scope = Scope::new();
        scope.push("base", 40_i64);
        assert_eq!(
            engine
                .eval_with_scope::<i64>(&mut scope, "base + 2")
                .unwrap(),
            42
        );
    }
}
