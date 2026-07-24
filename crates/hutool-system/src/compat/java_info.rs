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

/// Parsed Java version/vendor properties when a Java installation is configured.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JavaInfo {
    /// Version string.
    pub version: Option<String>,
    /// Vendor name.
    pub vendor: Option<String>,
    /// Vendor URL.
    pub vendor_url: Option<String>,
}

impl JavaInfo {
    /// Creates explicit Java properties.
    #[must_use]
    pub fn new(
        version: Option<String>,
        vendor: Option<String>,
        vendor_url: Option<String>,
    ) -> Self {
        Self {
            version,
            vendor,
            vendor_url,
        }
    }

    /// Detects opt-in Java environment properties without spawning a JVM.
    #[must_use]
    pub fn detect() -> Self {
        Self::new(
            env::var(SystemPropsKeys::JAVA_VERSION).ok(),
            env::var("JAVA_VENDOR").ok(),
            env::var("JAVA_VENDOR_URL").ok(),
        )
    }

    /// Returns a decimal representation of the leading version components.
    #[must_use]
    pub fn version_float(&self) -> Option<f32> {
        let (major, minor) = self.version_components()?;
        format!("{major}.{minor}").parse().ok()
    }

    fn version_components(&self) -> Option<(u32, u32)> {
        let mut parts = self
            .version
            .as_deref()?
            .trim_start_matches(|character: char| !character.is_ascii_digit())
            .split(|character: char| !character.is_ascii_digit())
            .filter(|part| !part.is_empty());
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .unwrap_or(0);
        Some((major, minor))
    }

    /// Returns the Java feature version (`1.8` becomes `8`).
    #[must_use]
    pub fn version_int(&self) -> Option<u32> {
        let (major, minor) = self.version_components()?;
        if major == 1 { Some(minor) } else { Some(major) }
    }

    /// Checks an exact feature version.
    #[must_use]
    pub fn is_version(&self, version: u32) -> bool {
        self.version_int() == Some(version)
    }

    /// Checks a minimum feature version.
    #[must_use]
    pub fn is_version_at_least(&self, version: u32) -> bool {
        self.version_int().is_some_and(|current| current >= version)
    }
}

macro_rules! property_info {
    ($name:ident { $($field:ident),+ $(,)? }) => {
        #[doc = concat!(stringify!($name), " property snapshot.")]
        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            $(
                #[doc = concat!(stringify!($field), " property.")]
                pub $field: Option<String>,
            )+
        }
    };
}
