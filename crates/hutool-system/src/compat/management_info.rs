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

use super::compilation_info::CompilationInfo;
use super::os_info::OsInfo;

/// Portable collection of native management information.
#[derive(Debug, Clone, PartialEq)]
pub struct ManagementInfo {
    /// Current process.
    pub process: Option<ProcessInfo>,
    /// Host memory.
    pub memory: MemoryInfo,
    /// OS properties.
    pub os: OsInfo,
    /// Compilation properties.
    pub compilation: CompilationInfo,
    /// Available parallelism as the portable thread-capacity measure.
    pub thread_capacity: usize,
}
