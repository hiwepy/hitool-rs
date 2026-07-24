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

use super::system_props_keys::SystemPropsKeys;

/// Java runtime path properties, retained only when explicitly configured.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaRuntimeInfo {
    /// Runtime name.
    pub name: Option<String>,
    /// Runtime version.
    pub version: Option<String>,
    /// Java home.
    pub home_dir: Option<PathBuf>,
    /// Class path.
    pub class_path: Option<String>,
    /// Native library path.
    pub library_path: Option<String>,
    /// Architecture data model.
    pub arch_data_model: Option<String>,
    /// Boot class path when supplied.
    pub boot_class_path: Option<String>,
    /// Extension directories when supplied.
    pub ext_dirs: Option<String>,
    /// Endorsed directories when supplied.
    pub endorsed_dirs: Option<String>,
    /// Class-file version when supplied.
    pub class_version: Option<String>,
    /// Protocol handler packages when supplied.
    pub protocol_packages: Option<String>,
}

impl JavaRuntimeInfo {
    /// Detects Java runtime environment variables without executing Java.
    #[must_use]
    pub fn detect() -> Self {
        Self {
            name: env::var("JAVA_RUNTIME_NAME").ok(),
            version: env::var(SystemPropsKeys::JAVA_VERSION).ok(),
            home_dir: env::var_os(SystemPropsKeys::JAVA_HOME).map(PathBuf::from),
            class_path: env::var("CLASSPATH").ok(),
            library_path: env::var("JAVA_LIBRARY_PATH").ok(),
            arch_data_model: env::var("SUN_ARCH_DATA_MODEL").ok(),
            boot_class_path: env::var("SUN_BOOT_CLASS_PATH").ok(),
            ext_dirs: env::var("JAVA_EXT_DIRS").ok(),
            endorsed_dirs: env::var("JAVA_ENDORSED_DIRS").ok(),
            class_version: env::var("JAVA_CLASS_VERSION").ok(),
            protocol_packages: env::var("JAVA_PROTOCOL_HANDLER_PKGS").ok(),
        }
    }

    /// Splits class path using the host path separator.
    #[must_use]
    pub fn class_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.class_path.as_deref())
    }

    /// Splits native library path using the host path separator.
    #[must_use]
    pub fn library_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.library_path.as_deref())
    }
}

fn split_paths(value: Option<&str>) -> Vec<PathBuf> {
    value.map_or_else(Vec::new, |paths| env::split_paths(paths).collect())
}
