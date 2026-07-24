use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::bindings::Bindings;
use super::compiled_script::CompiledScript;
use super::full_support_script_engine::FullSupportScriptEngine;
use super::java_script_engine::JavaScriptEngine;
use super::script_context::ScriptContext;
use super::script_runtime_exception::ScriptRuntimeException;

/// Stateless constructors and one-shot helpers corresponding to Hutool's utility class.
pub struct ScriptUtil;

impl ScriptUtil {
    /// Creates a named engine. Rust intentionally does not hide one in a global cache.
    pub fn get_script(name: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script(name)
    }

    /// Creates a named engine.
    pub fn create_script(name: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        FullSupportScriptEngine::new(name)
    }

    /// Creates a JavaScript-shaped compatibility wrapper.
    #[must_use]
    pub fn get_java_script_engine() -> JavaScriptEngine {
        JavaScriptEngine::new()
    }

    /// Creates the supported JS/Rhai facade.
    pub fn get_js_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::get_script("js")
    }

    /// Creates an independent JS/Rhai facade.
    pub fn create_js_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("js")
    }

    /// Reports that the optional Python engine is not linked.
    pub fn get_python_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("python")
    }

    /// Reports that the optional Python engine is not linked.
    pub fn create_python_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("python")
    }

    /// Reports that the optional Lua engine is not linked.
    pub fn get_lua_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("lua")
    }

    /// Reports that the optional Lua engine is not linked.
    pub fn create_lua_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("lua")
    }

    /// Reports that the optional Groovy engine is not linked.
    pub fn get_groovy_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("groovy")
    }

    /// Reports that the optional Groovy engine is not linked.
    pub fn create_groovy_engine() -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        Self::create_script("groovy")
    }

    /// Evaluates a script and retains its AST for invocation.
    pub fn eval_invocable(script: &str) -> Result<FullSupportScriptEngine, ScriptRuntimeException> {
        let mut engine = FullSupportScriptEngine::from_engine(ScriptEngine::default());
        let _ = engine.eval(script)?;
        Ok(engine)
    }

    /// Evaluates a script with a fresh engine.
    pub fn eval(script: &str) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default()).eval(script)
    }

    /// Evaluates a script with an explicit context.
    pub fn eval_with_context(
        script: &str,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default())
            .eval_with_context(script, context)
    }

    /// Evaluates a script with explicit bindings.
    pub fn eval_with_bindings(
        script: &str,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default())
            .eval_with_bindings(script, bindings)
    }

    /// Evaluates and invokes one function.
    pub fn invoke(
        script: &str,
        function: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        Self::eval_invocable(script)?.invoke_function(function, args)
    }

    /// Compiles with a fresh engine.
    pub fn compile(script: &str) -> Result<CompiledScript, ScriptRuntimeException> {
        FullSupportScriptEngine::from_engine(ScriptEngine::default()).compile(script)
    }

    /// Compiles with an explicitly configured engine.
    pub fn compile_with_engine(
        engine: &FullSupportScriptEngine,
        script: &str,
    ) -> Result<CompiledScript, ScriptRuntimeException> {
        engine.compile(script)
    }
}
