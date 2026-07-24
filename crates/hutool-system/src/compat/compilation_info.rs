//! Hutool-named portable system property and runtime views.

use std::{
    env,
    ffi::OsString,
    fmt::{self, Write as _},
    io,
    path::PathBuf,
};

use sysinfo::System;

use crate::{MemoryInfo, OshiUtil, ProcessInfo, SystemSnapshot};

/// Native Rust compiler/runtime information replacing JVM-only `MXBeans`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationInfo {
    /// Compiler family.
    pub compiler: &'static str,
    /// Target architecture.
    pub target_arch: &'static str,
    /// Whether debug assertions are enabled.
    pub debug_assertions: bool,
}
