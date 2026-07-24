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

/// Host identity snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostInfo {
    /// Host name.
    pub name: Option<String>,
    /// Primary address when supplied by the caller.
    pub address: Option<String>,
}

impl HostInfo {
    /// Collects portable host identity.
    #[must_use]
    pub fn collect() -> Self {
        Self {
            name: System::host_name(),
            address: None,
        }
    }
}
