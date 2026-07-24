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

/// Current-user and locale properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    /// User name.
    pub name: Option<String>,
    /// Home directory.
    pub home_dir: Option<PathBuf>,
    /// Current working directory.
    pub current_dir: Option<PathBuf>,
    /// Temporary directory.
    pub temp_dir: PathBuf,
    /// ISO-like language component.
    pub language: Option<String>,
    /// ISO-like country component.
    pub country: Option<String>,
}

impl UserInfo {
    /// Creates user information from explicit portable inputs.
    #[must_use]
    pub fn from_parts(
        name: Option<String>,
        home_dir: Option<PathBuf>,
        current_dir: Option<PathBuf>,
        temp_dir: PathBuf,
        locale: &str,
    ) -> Self {
        let locale = locale.split('.').next().unwrap_or_default();
        let (language, country) = locale.split_once('_').map_or_else(
            || (non_empty(locale), None),
            |(language, country)| (non_empty(language), non_empty(country)),
        );
        Self {
            name,
            home_dir,
            current_dir,
            temp_dir,
            language,
            country,
        }
    }

    /// Collects user, path, and locale properties.
    #[must_use]
    pub fn collect() -> Self {
        let locale = option_or_default(first_env(env::var("LC_ALL"), env::var("LANG")));
        Self::from_parts(
            first_env(env::var(SystemPropsKeys::USER_NAME), env::var("USERNAME")),
            optional_path(env::var_os(SystemPropsKeys::USER_HOME)),
            result_path(env::current_dir()),
            env::temp_dir(),
            &locale,
        )
    }
}

fn optional_path(value: Option<OsString>) -> Option<PathBuf> {
    value.map(PathBuf::from)
}

fn option_or_default(value: Option<String>) -> String {
    value.unwrap_or_default()
}

fn result_path(value: io::Result<PathBuf>) -> Option<PathBuf> {
    value.ok()
}

fn first_env(

fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}
