use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

use super::bindings::Bindings;
use super::compiled_script::CompiledScript;
use super::script_context::ScriptContext;
use super::script_engine_factory::ScriptEngineFactory;
use super::script_interface::ScriptInterface;
use super::script_language::ScriptLanguage;
use super::script_runtime_exception::ScriptRuntimeException;
use super::script_scope::ScriptScope;

/// Full compile/evaluate/invoke facade over a constrained engine.
pub struct FullSupportScriptEngine {
    engine: ScriptEngine,
    context: ScriptContext,
    active_ast: Option<AST>,
}

impl FullSupportScriptEngine {
    /// Wraps an explicitly configured engine.
    #[must_use]
    pub fn from_engine(engine: ScriptEngine) -> Self {
        Self {
            engine,
            context: ScriptContext::new(),
            active_ast: None,
        }
    }

    /// Creates a supported named engine.
    pub fn new(name_or_extension_or_mime: &str) -> Result<Self, ScriptRuntimeException> {
        match ScriptLanguage::resolve(name_or_extension_or_mime)
            .map_err(|error| ScriptRuntimeException::from_error(&error, None))?
        {
            ScriptLanguage::Rhai => Ok(Self::from_engine(ScriptEngine::default())),
            language => Err(ScriptRuntimeException::new(format!(
                "script language `{language:?}` requires an optional engine"
            ))),
        }
    }

    /// Compiles script text.
    pub fn compile(&self, script: &str) -> Result<CompiledScript, ScriptRuntimeException> {
        self.engine
            .compile(script)
            .map(|ast| CompiledScript {
                ast,
                source_name: self.context.source_name.clone(),
            })
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Compiles script text read from a bounded reader.
    pub fn compile_reader(
        &self,
        reader: impl Read,
    ) -> Result<CompiledScript, ScriptRuntimeException> {
        let script = read_script(reader)?;
        self.compile(&script)
    }

    /// Evaluates text in the engine's current context.
    pub fn eval(&mut self, script: &str) -> Result<Dynamic, ScriptRuntimeException> {
        let compiled = self.compile(script)?;
        let result = compiled.eval(&self.engine, &mut self.context)?;
        self.active_ast = Some(compiled.ast);
        Ok(result)
    }

    /// Evaluates bounded reader input in the current context.
    pub fn eval_reader(&mut self, reader: impl Read) -> Result<Dynamic, ScriptRuntimeException> {
        let script = read_script(reader)?;
        self.eval(&script)
    }

    /// Evaluates text with an explicit context.
    pub fn eval_with_context(
        &mut self,
        script: &str,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let compiled = self.compile(script)?;
        let result = compiled.eval(&self.engine, context)?;
        self.active_ast = Some(compiled.ast);
        Ok(result)
    }

    /// Evaluates reader input with an explicit context.
    pub fn eval_reader_with_context(
        &mut self,
        reader: impl Read,
        context: &mut ScriptContext,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        self.eval_with_context(&read_script(reader)?, context)
    }

    /// Evaluates text with temporary engine bindings.
    pub fn eval_with_bindings(
        &mut self,
        script: &str,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let mut context = ScriptContext::new();
        context.set_bindings(ScriptScope::Engine, bindings);
        self.eval_with_context(script, &mut context)
    }

    /// Evaluates reader input with temporary bindings.
    pub fn eval_reader_with_bindings(
        &mut self,
        reader: impl Read,
        bindings: Bindings,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        self.eval_with_bindings(&read_script(reader)?, bindings)
    }

    /// Invokes a function in the most recently compiled/evaluated AST.
    pub fn invoke_function(
        &self,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let ast = self.active_ast.as_ref().ok_or_else(|| {
            ScriptRuntimeException::new("no compiled script is active for invocation")
        })?;
        let mut scope = self.context.scope();
        self.engine
            .call_fn(&mut scope, ast, name, args)
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Invokes a method with a bound dynamic `this` value.
    pub fn invoke_method(
        &self,
        this: &mut Dynamic,
        name: &str,
        args: Vec<Dynamic>,
    ) -> Result<Dynamic, ScriptRuntimeException> {
        let ast = self.active_ast.as_ref().ok_or_else(|| {
            ScriptRuntimeException::new("no compiled script is active for invocation")
        })?;
        let mut scope = self.context.scope();
        self.engine
            .call_method(&mut scope, ast, this, name, args)
            .map_err(|error| ScriptRuntimeException::from_error(&error, self.context.source_name()))
    }

    /// Returns a typed Rust invocation facade.
    pub fn interface(&mut self) -> ScriptInterface<'_> {
        ScriptInterface {
            engine: self,
            this: None,
        }
    }

    /// Returns an invocation facade bound to a `this` value.
    pub fn interface_on(&mut self, this: Dynamic) -> ScriptInterface<'_> {
        ScriptInterface {
            engine: self,
            this: Some(this),
        }
    }

    /// Stores an engine-scope value.
    pub fn put(&mut self, key: impl Into<String>, value: impl Into<Dynamic>) -> Option<Dynamic> {
        self.context.engine.insert(key, value)
    }

    /// Returns an engine-scope value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Dynamic> {
        self.context.engine.get(key)
    }

    /// Returns bindings for one scope.
    #[must_use]
    pub fn bindings(&self, scope: ScriptScope) -> &Bindings {
        self.context.bindings(scope)
    }

    /// Replaces bindings for one scope.
    pub fn set_bindings(&mut self, bindings: Bindings, scope: ScriptScope) {
        self.context.set_bindings(scope, bindings);
    }

    /// Creates independent empty bindings.
    #[must_use]
    pub const fn create_bindings(&self) -> Bindings {
        Bindings::new()
    }

    /// Returns the current context.
    #[must_use]
    pub const fn context(&self) -> &ScriptContext {
        &self.context
    }

    /// Replaces the current context.
    pub fn set_context(&mut self, context: ScriptContext) {
        self.context = context;
    }

    /// Returns engine metadata.
    #[must_use]
    pub const fn factory(&self) -> ScriptEngineFactory {
        ScriptEngineFactory
    }
}

fn read_script(reader: impl Read) -> Result<String, ScriptRuntimeException> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_READER_BYTES)
        .read_to_end(&mut bytes)
        .map_err(|error| ScriptRuntimeException::from_error(&error.into(), None))?;
    if bytes.len() as u64 >= MAX_READER_BYTES {
        return Err(ScriptRuntimeException::new("script reader exceeds 1 MiB"));
    }
    String::from_utf8(bytes).map_err(|error| ScriptRuntimeException::new(error.to_string()))
}
