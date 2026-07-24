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

/// Common environment/property keys corresponding to Hutool's constants.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemPropsKeys;

impl SystemPropsKeys {
    /// User name environment key on Unix-like systems.
    pub const USER_NAME: &'static str = "USER";
    /// User home environment key.
    pub const USER_HOME: &'static str = "HOME";
    /// Temporary directory environment key.
    pub const TEMP_DIR: &'static str = "TMPDIR";
    /// Java home environment key.
    pub const JAVA_HOME: &'static str = "JAVA_HOME";
    /// Java version override key used by this portable facade.
    pub const JAVA_VERSION: &'static str = "JAVA_VERSION";
}
