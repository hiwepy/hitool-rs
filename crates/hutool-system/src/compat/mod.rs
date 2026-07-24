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

mod system_props_keys;
mod host_info;
mod os_info;
mod user_info;
mod runtime_info;
mod java_info;
mod java_runtime_info;
mod compilation_info;
mod management_info;
mod system_util;

pub use system_props_keys::SystemPropsKeys;
pub use host_info::HostInfo;
pub use os_info::OsInfo;
pub use user_info::UserInfo;
pub use runtime_info::RuntimeInfo;
pub use java_info::JavaInfo;
pub use java_runtime_info::JavaRuntimeInfo;
pub use compilation_info::CompilationInfo;
pub use management_info::ManagementInfo;
pub use system_util::SystemUtil;
