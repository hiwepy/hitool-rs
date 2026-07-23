//! 对齐: `cn.hutool.core.io.file.FileCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileCopier.java

use std::path::{Path, PathBuf};

use super::path_util::PathUtil;

/// 对齐 Java 类: `cn.hutool.core.io.file.FileCopier`
#[derive(Debug, Clone)]
pub struct FileCopier {
    src: PathBuf,
    dest: PathBuf,
    override_existing: bool,
}

impl FileCopier {
    /// 对齐 Java: `FileCopier.create(String, String)`
    pub fn create(src_path: impl AsRef<Path>, dest_path: impl AsRef<Path>) -> Self {
        Self {
            src: src_path.as_ref().to_path_buf(),
            dest: dest_path.as_ref().to_path_buf(),
            override_existing: false,
        }
    }

    /// 对齐 Java: `FileCopier.setOverride(boolean)`
    pub fn set_override(mut self, override_existing: bool) -> Self {
        self.override_existing = override_existing;
        self
    }

    /// 对齐 Java: `FileCopier.copy()`
    pub fn copy(self) -> std::io::Result<PathBuf> {
        if !self.src.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("source not found: {}", self.src.display()),
            ));
        }
        if self.src == self.dest {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "source and destination are equal",
            ));
        }
        if self.dest.exists() && self.dest.is_file() && !self.override_existing {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("destination exists: {}", self.dest.display()),
            ));
        }
        PathUtil::copy(&self.src, &self.dest)
    }
}
