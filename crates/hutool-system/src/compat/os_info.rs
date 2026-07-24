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

/// Operating-system properties and Hutool-compatible family predicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    /// Target architecture.
    pub arch: String,
    /// Operating-system name.
    pub name: String,
    /// Operating-system version.
    pub version: String,
    /// Filesystem separator.
    pub file_separator: char,
    /// Native line separator.
    pub line_separator: &'static str,
    /// Search-path separator.
    pub path_separator: char,
}

impl OsInfo {
    /// Creates an OS view from explicit parts, useful for deterministic policy checks.
    #[must_use]
    pub fn from_parts(
        arch: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            arch: arch.into(),
            name: name.into(),
            version: version.into(),
            file_separator: std::path::MAIN_SEPARATOR,
            line_separator: LINE_SEPARATOR,
            path_separator: PATH_SEPARATOR,
        }
    }

    /// Collects current OS properties.
    #[must_use]
    pub fn collect() -> Self {
        Self::from_parts(
            env::consts::ARCH,
            value_or_else(System::name(), env::consts::OS),
            System::os_version().unwrap_or_default(),
        )
    }

    fn normalized_name(&self) -> String {
        self.name.to_ascii_lowercase().replace([' ', '-'], "")
    }

    /// Returns whether the OS name matches AIX.
    #[must_use]
    pub fn is_aix(&self) -> bool {
        self.normalized_name().contains("aix")
    }

    /// Returns whether the OS name matches HP-UX.
    #[must_use]
    pub fn is_hp_ux(&self) -> bool {
        self.normalized_name().contains("hpux")
    }

    /// Returns whether the OS name matches IRIX.
    #[must_use]
    pub fn is_irix(&self) -> bool {
        self.normalized_name().contains("irix")
    }

    /// Returns whether the OS name matches Linux.
    #[must_use]
    pub fn is_linux(&self) -> bool {
        self.normalized_name().contains("linux")
    }

    /// Returns whether the OS name matches macOS.
    #[must_use]
    pub fn is_mac(&self) -> bool {
        let name = self.normalized_name();
        name.contains("mac") || name.contains("darwin")
    }

    /// Alias for [`Self::is_mac`].
    #[must_use]
    pub fn is_mac_os_x(&self) -> bool {
        self.is_mac()
    }

    /// Returns whether the OS name matches OS/2.
    #[must_use]
    pub fn is_os2(&self) -> bool {
        self.normalized_name().contains("os/2") || self.normalized_name() == "os2"
    }

    /// Returns whether the OS name matches Solaris.
    #[must_use]
    pub fn is_solaris(&self) -> bool {
        self.normalized_name().contains("solaris")
    }

    /// Returns whether the OS name matches `SunOS`.
    #[must_use]
    pub fn is_sun_os(&self) -> bool {
        self.normalized_name().contains("sunos")
    }

    /// Returns whether this is a Windows family.
    #[must_use]
    pub fn is_windows(&self) -> bool {
        self.normalized_name().contains("windows")
    }

    fn windows_version(&self, expected: &str) -> bool {
        self.is_windows() && self.version.to_ascii_lowercase().contains(expected)
    }

    /// Windows 2000 predicate.
    #[must_use]
    pub fn is_windows_2000(&self) -> bool {
        self.windows_version("2000")
    }

    /// Windows 95 predicate.
    #[must_use]
    pub fn is_windows_95(&self) -> bool {
        self.windows_version("95")
    }

    /// Windows 98 predicate.
    #[must_use]
    pub fn is_windows_98(&self) -> bool {
        self.windows_version("98")
    }

    /// Windows ME predicate.
    #[must_use]
    pub fn is_windows_me(&self) -> bool {
        self.windows_version("me")
    }

    /// Windows NT predicate.
    #[must_use]
    pub fn is_windows_nt(&self) -> bool {
        self.windows_version("nt")
    }

    /// Windows XP predicate.
    #[must_use]
    pub fn is_windows_xp(&self) -> bool {
        self.windows_version("xp")
    }

    /// Windows 7 predicate.
    #[must_use]
    pub fn is_windows_7(&self) -> bool {
        self.windows_version("7")
    }

    /// Windows 8 predicate excluding 8.1.
    #[must_use]
    pub fn is_windows_8(&self) -> bool {
        self.windows_version("8") && !self.windows_version("8.1")
    }

    /// Windows 8.1 predicate.
    #[must_use]
    pub fn is_windows_8_1(&self) -> bool {
        self.windows_version("8.1")
    }

    /// Windows 10 predicate.
    #[must_use]
    pub fn is_windows_10(&self) -> bool {
        self.windows_version("10")
    }

    /// Windows 11 predicate.
    #[must_use]
    pub fn is_windows_11(&self) -> bool {
        self.windows_version("11")
    }
}

const PATH_SEPARATOR: char = ':';

const LINE_SEPARATOR: &str = "\n";

fn value_or_else(value: Option<String>, fallback: &str) -> String {
    value.unwrap_or_else(|| fallback.to_owned())
}
