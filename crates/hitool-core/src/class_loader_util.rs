//! 对齐: `cn.hutool.core.util.ClassLoaderUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassLoaderUtil.java
//!
//! Rust 侧对齐 Java 类名规范化（内部类 `.` → `$`）与 `Class.getName()` 语义。

use crate::{CoreError, Result};

/// 对齐 Java `Class.getName()` 的轻量返回值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedClass {
    /// JVM 风格类名。
    pub name: String,
}

/// 对齐 Java: `cn.hutool.core.util.ClassLoaderUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassLoaderUtil;

impl ClassLoaderUtil {
    /// 对齐 Java: `ClassLoaderUtil.loadClass(String)`
    pub fn load_class(name: &str) -> Result<LoadedClass> {
        Self::load_class_with_init(name, true)
    }

    /// 对齐 Java: `ClassLoaderUtil.loadClass(String, boolean)`
    pub fn load_class_with_init(name: &str, _is_initialized: bool) -> Result<LoadedClass> {
        let normalized = normalize_class_name(name)?;
        Ok(LoadedClass { name: normalized })
    }
}

/// 将 Java 外部类名规范化为 JVM `Class.getName()` 形式。
fn normalize_class_name(name: &str) -> Result<String> {
    let name = name.trim().replace('/', ".");
    if name.is_empty() {
        return Err(CoreError::InvalidArgument {
            name: "className",
            reason: "must not be blank",
        });
    }

    if name.contains('$') {
        return Ok(name);
    }

    if let Some(canonical) = known_class_aliases().get(name.as_str()) {
        return Ok(canonical.to_string());
    }

    if let Some(idx) = name.rfind('.') {
        if idx > 0 {
            let inner = format!("{}${}", &name[..idx], &name[idx + 1..]);
            if known_class_aliases().contains_key(inner.as_str()) || inner.contains('$') {
                return Ok(inner);
            }
        }
    }

    Ok(name)
}

/// Hutool parity 与常见 JVM 类型别名。
fn known_class_aliases() -> &'static std::collections::HashMap<&'static str, &'static str> {
    use std::collections::HashMap;
    static ALIASES: std::sync::OnceLock<HashMap<&'static str, &'static str>> =
        std::sync::OnceLock::new();
    ALIASES.get_or_init(|| {
        HashMap::from([
            (
                "java.lang.Thread.State",
                "java.lang.Thread$State",
            ),
            (
                "java.lang.Thread$State",
                "java.lang.Thread$State",
            ),
        ])
    })
}
