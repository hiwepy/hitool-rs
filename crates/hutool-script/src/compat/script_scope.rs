use crate::{AST, Dynamic, Scope, ScriptEngine, ScriptError};
use std::{
    collections::BTreeMap,
    fmt,
    io::Read,
    ops::{Deref, DerefMut},
};

/// Binding scopes corresponding to JSR-223 engine and global scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptScope {
    /// Per-engine values, taking precedence over globals.
    Engine,
    /// Shared values copied into an evaluation context.
    Global,
}
