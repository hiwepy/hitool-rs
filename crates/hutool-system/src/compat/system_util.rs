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
use super::host_info::HostInfo;
use super::java_info::JavaInfo;
use super::java_runtime_info::JavaRuntimeInfo;
use super::management_info::ManagementInfo;
use super::os_info::OsInfo;
use super::runtime_info::RuntimeInfo;
use super::user_info::UserInfo;

/// Hutool-aligned static system facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemUtil;

impl SystemUtil {
    /// Returns the current process identifier.
    #[must_use]
    pub fn current_pid() -> u32 {
        std::process::id()
    }

    /// Collects native management information.
    #[must_use]
    pub fn management_info() -> ManagementInfo {
        ManagementInfo {
            process: OshiUtil::current_process(),
            memory: OshiUtil::memory(),
            os: OsInfo::collect(),
            compilation: CompilationInfo {
                compiler: "rustc",
                target_arch: env::consts::ARCH,
                debug_assertions: cfg!(debug_assertions),
            },
            thread_capacity: std::thread::available_parallelism().map_or(1, usize::from),
        }
    }

    /// Returns JVM memory pools. Native Rust has no managed heap pools.
    #[must_use]
    pub const fn memory_pools() -> &'static [&'static str] {
        &[]
    }

    /// Returns JVM memory managers. Native Rust has no JVM managers.
    #[must_use]
    pub const fn memory_managers() -> &'static [&'static str] {
        &[]
    }

    /// Returns JVM garbage collectors. Native Rust has no JVM GC.
    #[must_use]
    pub const fn garbage_collectors() -> &'static [&'static str] {
        &[]
    }

    /// Returns Java specification properties supplied by the environment.
    #[must_use]
    pub fn java_spec_info() -> JavaSpecInfo {
        JavaSpecInfo {
            name: env::var("JAVA_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_SPECIFICATION_VENDOR").ok(),
        }
    }

    /// Returns JVM properties supplied by the environment.
    #[must_use]
    pub fn jvm_info() -> JvmInfo {
        JvmInfo {
            name: env::var("JAVA_VM_NAME").ok(),
            version: env::var("JAVA_VM_VERSION").ok(),
            vendor: env::var("JAVA_VM_VENDOR").ok(),
            info: env::var("JAVA_VM_INFO").ok(),
        }
    }

    /// Returns JVM specification properties supplied by the environment.
    #[must_use]
    pub fn jvm_spec_info() -> JvmSpecInfo {
        JvmSpecInfo {
            name: env::var("JAVA_VM_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_VM_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_VM_SPECIFICATION_VENDOR").ok(),
        }
    }

    /// Returns Java installation properties.
    #[must_use]
    pub fn java_info() -> JavaInfo {
        JavaInfo::detect()
    }

    /// Returns Java runtime path properties.
    #[must_use]
    pub fn java_runtime_info() -> JavaRuntimeInfo {
        JavaRuntimeInfo::detect()
    }

    /// Returns operating-system properties.
    #[must_use]
    pub fn os_info() -> OsInfo {
        OsInfo::collect()
    }

    /// Returns user and locale properties.
    #[must_use]
    pub fn user_info() -> UserInfo {
        UserInfo::collect()
    }

    /// Returns host identity.
    #[must_use]
    pub fn host_info() -> HostInfo {
        HostInfo::collect()
    }

    /// Returns native runtime memory.
    #[must_use]
    pub fn runtime_info() -> RuntimeInfo {
        RuntimeInfo::collect()
    }

    /// Returns total physical memory.
    #[must_use]
    pub fn total_memory() -> u64 {
        OshiUtil::memory().total
    }

    /// Returns available physical memory.
    #[must_use]
    pub fn free_memory() -> u64 {
        OshiUtil::memory().available
    }

    /// Returns the native maximum memory boundary.
    #[must_use]
    pub fn max_memory() -> u64 {
        Self::total_memory()
    }

    /// Returns portable thread execution capacity.
    #[must_use]
    pub fn total_thread_count() -> usize {
        std::thread::available_parallelism().map_or(1, usize::from)
    }

    /// Produces a stable human-readable system dump.
    #[must_use]
    pub fn system_info_dump() -> String {
        let snapshot = SystemSnapshot::collect();
        let user = Self::user_info();
        let mut output = String::new();
        let _ = writeln!(
            output,
            "host={}",
            snapshot.host_name.as_deref().unwrap_or("")
        );
        let _ = writeln!(output, "os={}", snapshot.os_name.as_deref().unwrap_or(""));
        let _ = writeln!(output, "cpus={}", snapshot.cpu_count);
        let _ = writeln!(output, "memory.total={}", snapshot.total_memory);
        let _ = writeln!(output, "memory.used={}", snapshot.used_memory);
        let _ = writeln!(output, "user={}", user.name.as_deref().unwrap_or(""));
        output
    }

    /// Writes a system dump to an injected writer.
    pub fn dump_system_info(writer: &mut dyn io::Write) -> io::Result<()> {
        writer.write_all(Self::system_info_dump().as_bytes())
    }
}

impl fmt::Display for OsInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {} ({})", self.name, self.version, self.arch)
    }
}
