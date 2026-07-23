//! 对齐: `cn.hutool.core.io.file.FileNameUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileNameUtil.java
//!
//! 文件名拆分 / 非法字符清理；委托 [`crate::FileUtil`] 的路径片段语义。

use crate::FileUtil;
use std::path::Path;

/// Windows 文件名非法字符（对齐 Hutool `FILE_NAME_INVALID_PATTERN_WIN`）。
const INVALID_WIN: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|', '\r', '\n'];

/// 对齐 Java 类: `cn.hutool.core.io.file.FileNameUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct FileNameUtil;

impl FileNameUtil {
    /// 对齐 Java: `FileNameUtil.EXT_JAVA`
    pub const EXT_JAVA: &'static str = ".java";
    /// 对齐 Java: `FileNameUtil.EXT_CLASS`
    pub const EXT_CLASS: &'static str = ".class";
    /// 对齐 Java: `FileNameUtil.EXT_JAR`
    pub const EXT_JAR: &'static str = ".jar";

    /// 对齐 Java: `FileNameUtil.getName(File)` / `getName(String)`
    pub fn name(path: &str) -> &str {
        FileUtil::name(Path::new(path))
    }

    /// 对齐 Java: `FileNameUtil.getSuffix(File)` / `getSuffix(String)`
    pub fn suffix(path: &str) -> &str {
        Self::ext_name(path)
    }

    /// 对齐 Java: `FileNameUtil.getPrefix(File)` / `getPrefix(String)`
    pub fn prefix(path: &str) -> &str {
        Self::main_name(path)
    }

    /// 对齐 Java: `FileNameUtil.mainName(File)` / `mainName(String)`
    pub fn main_name(path: &str) -> &str {
        FileUtil::main_name(Path::new(path))
    }

    /// 对齐 Java: `FileNameUtil.extName(File)` / `extName(String)`（不带点）
    pub fn ext_name(path: &str) -> &str {
        let name = Self::name(path);
        // 特殊复合后缀（对齐 Hutool SPECIAL_SUFFIX）
        for special in ["tar.bz2", "tar.Z", "tar.gz", "tar.xz"] {
            if name.ends_with(special) {
                return special;
            }
        }
        FileUtil::suffix(Path::new(name))
    }

    /// 对齐 Java: `FileNameUtil.cleanInvalid(String)` — 剔除 Windows 非法字符。
    pub fn clean_invalid(file_name: &str) -> String {
        file_name
            .chars()
            .filter(|c| !INVALID_WIN.contains(c))
            .collect()
    }

    /// 对齐 Java: `FileNameUtil.containsInvalid(String)`
    pub fn contains_invalid(file_name: &str) -> bool {
        file_name.chars().any(|c| INVALID_WIN.contains(&c))
    }

    /// 对齐 Java: `FileNameUtil.isType(String, String...)` — 扩展名大小写不敏感匹配。
    pub fn is_type(file_name: &str, types: &[&str]) -> bool {
        let ext = Self::ext_name(file_name).to_ascii_lowercase();
        types
            .iter()
            .any(|t| t.trim_start_matches('.').eq_ignore_ascii_case(&ext))
    }
}
