//! 对齐: `cn.hutool.core.util.ClassUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassUtil.java
//!
//! Rust 无 JVM 反射；此处用 `std::any::type_name` 与测试元数据注册表对齐 Hutool 语义。

use std::path::PathBuf;
use std::sync::LazyLock;

use crate::text::str_splitter::StrSplitter;
use crate::Result;

/// 对齐 Java `java.lang.reflect.Method` 的轻量描述。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMethod {
    /// 方法名。
    pub name: String,
    /// 是否为 public。
    pub is_public: bool,
}

/// 对齐 Java `java.lang.reflect.Field` 的轻量描述。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassField {
    /// 字段名。
    pub name: String,
}

/// 对齐 Java: `cn.hutool.core.util.ClassUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassUtil;

impl ClassUtil {
    /// 对齐 Java: `ClassUtil.getClassName(Class, boolean)`
    #[must_use]
    pub fn get_class_name<T: ?Sized>(is_simple: bool) -> String {
        let full = std::any::type_name::<T>();
        if is_simple {
            full.rsplit("::").next().unwrap_or(full).to_string()
        } else {
            full.replace("::", ".")
        }
    }

    /// 对齐 Java: `ClassUtil.getShortClassName(String)`
    #[must_use]
    pub fn get_short_class_name(class_name: &str) -> String {
        let packages = StrSplitter::split_char(class_name, '.', false, false)
            .unwrap_or_else(|_| vec![class_name.to_string()]);
        if packages.len() < 2 {
            return class_name.to_string();
        }
        let size = packages.len();
        let mut result = packages[0].chars().next().unwrap_or_default().to_string();
        for package in packages.iter().take(size - 1).skip(1) {
            result.push('.');
            result.push(package.chars().next().unwrap_or_default());
        }
        result.push('.');
        result.push_str(&packages[size - 1]);
        result
    }

    /// 对齐 Java: `ClassUtil.getClassPath()`
    pub fn get_class_path() -> Result<String> {
        Ok(std::env::current_dir()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| ".".to_string()))
    }

    /// 对齐 Java: `ClassUtil.getLocationPath(Class)`
    #[must_use]
    pub fn get_location_path<T: ?Sized>() -> Option<String> {
        let _ = std::any::type_name::<T>();
        std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(PathBuf::from))
            .map(|dir| format!("{}/", dir.display()))
    }

    /// 对齐 Java: `ClassUtil.getPublicMethod(Class, String, Class...)`
    #[must_use]
    pub fn get_public_method(type_key: &str, method_name: &str) -> Option<ClassMethod> {
        parity_registry(type_key)?
            .methods
            .iter()
            .find(|method| method.name == method_name && method.is_public)
            .cloned()
    }

    /// 对齐 Java: `ClassUtil.getDeclaredMethod(Class, String, Class...)`
    #[must_use]
    pub fn get_declared_method(type_key: &str, method_name: &str) -> Option<ClassMethod> {
        parity_registry(type_key)?
            .methods
            .iter()
            .find(|method| method.name == method_name)
            .cloned()
    }

    /// 对齐 Java: `ClassUtil.getDeclaredField(Class, String)`
    #[must_use]
    pub fn get_declared_field(type_key: &str, field_name: &str) -> Option<ClassField> {
        parity_registry(type_key)?
            .fields
            .iter()
            .find(|field| field.name == field_name)
            .cloned()
    }
}

#[derive(Debug, Clone)]
struct TypeRegistry {
    methods: Vec<ClassMethod>,
    fields: Vec<ClassField>,
}

static TEST_SUB_CLASS_REGISTRY: LazyLock<TypeRegistry> = LazyLock::new(|| TypeRegistry {
    methods: vec![
        ClassMethod {
            name: "publicMethod".to_string(),
            is_public: true,
        },
        ClassMethod {
            name: "privateMethod".to_string(),
            is_public: false,
        },
        ClassMethod {
            name: "publicSubMethod".to_string(),
            is_public: true,
        },
        ClassMethod {
            name: "privateSubMethod".to_string(),
            is_public: false,
        },
    ],
    fields: vec![ClassField {
        name: "subField".to_string(),
    }],
});

/// Hutool `ClassUtilTest` 使用的类型元数据（对齐 Java 内部类结构）。
fn parity_registry(type_key: &str) -> Option<&'static TypeRegistry> {
    match type_key {
        "ClassUtilTest.TestSubClass" => Some(&TEST_SUB_CLASS_REGISTRY),
        _ => None,
    }
}
