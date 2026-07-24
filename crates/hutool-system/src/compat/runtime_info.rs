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

/// Rust process/runtime memory counterpart of Hutool's `RuntimeInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeInfo {
    /// Maximum usable memory (physical memory on native Rust).
    pub max_memory: u64,
    /// Total physical memory.
    pub total_memory: u64,
    /// Available physical memory.
    pub free_memory: u64,
    /// Current process resident memory.
    pub process_memory: u64,
}

impl RuntimeInfo {
    /// Collects runtime memory counters.
    #[must_use]
    pub fn collect() -> Self {
        let memory = OshiUtil::memory();
        let process_memory = OshiUtil::current_process().map_or(0, |process| process.memory);
        Self {
            max_memory: memory.total,
            total_memory: memory.total,
            free_memory: memory.available,
            process_memory,
        }
    }

    /// Returns memory usable without exceeding the native host limit.
    #[must_use]
    pub fn usable_memory(self) -> u64 {
        self.free_memory.saturating_add(self.process_memory)
    }
}
