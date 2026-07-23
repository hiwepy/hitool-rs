//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// 注解类型名，对齐 Java `Class<? extends Annotation>` 的 fully-qualified name。
pub type AnnotationTypeName = &'static str;

/// 属性值类型种类，对齐 Java 注解属性返回类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueKind {
    Void,
    Bool,
    I32,
    I64,
    F64,
    String,
    Class,
    Annotation,
    Array,
}

/// 注解属性值。
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationValue {
    Unit,
    Bool(bool),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
    Class(String),
    Array(Vec<AnnotationValue>),
    Annotation(Arc<AnnotationMirror>),
}

impl AnnotationValue {
    /// 返回值的类型种类。
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Unit => ValueKind::Void,
            Self::Bool(_) => ValueKind::Bool,
            Self::I32(_) => ValueKind::I32,
            Self::I64(_) => ValueKind::I64,
            Self::F64(_) => ValueKind::F64,
            Self::String(_) => ValueKind::String,
            Self::Class(_) => ValueKind::Class,
            Self::Array(_) => ValueKind::Array,
            Self::Annotation(_) => ValueKind::Annotation,
        }
    }

    /// 作为字符串取值；非字符串类型返回 `None`。
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// 作为字符串数组取值。
    pub fn as_str_array(&self) -> Option<Vec<&str>> {
        match self {
            Self::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(item.as_str()?);
                }
                Some(out)
            }
            _ => None,
        }
    }
}

/// 属性定义，对齐 Java 注解接口中的 attribute method。
#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: &'static str,
    pub value_kind: ValueKind,
    pub default_value: AnnotationValue,
    /// 属性方法上的元注解（如 `@Alias`、`@MirrorFor`）。
    pub meta: Vec<Arc<AnnotationMirror>>,
}

impl AttributeDef {
    /// 创建字符串属性定义。
    pub fn string(name: &'static str, default: &str) -> Self {
        Self {
            name,
            value_kind: ValueKind::String,
            default_value: AnnotationValue::String(default.to_string()),
            meta: Vec::new(),
        }
    }

    /// 创建 Class 类型属性定义。
    pub fn class_type(name: &'static str, default: &str) -> Self {
        Self {
            name,
            value_kind: ValueKind::Class,
            default_value: AnnotationValue::Class(default.to_string()),
            meta: Vec::new(),
        }
    }

    /// 附加属性元注解。
    pub fn with_meta(mut self, meta: Arc<AnnotationMirror>) -> Self {
        self.meta.push(meta);
        self
    }
}

/// 注解类型 schema。
#[derive(Debug, Clone)]
pub struct AnnotationSchema {
    pub type_name: AnnotationTypeName,
    pub attributes: Vec<AttributeDef>,
    pub meta: Vec<Arc<AnnotationMirror>>,
    pub inherited: bool,
}

impl AnnotationSchema {
    /// 查找属性定义。
    pub fn attribute(&self, name: &str) -> Option<&AttributeDef> {
        self.attributes.iter().find(|a| a.name == name)
    }
}

/// 注解实例镜像。
#[derive(Clone)]
pub struct AnnotationMirror {
    pub type_name: AnnotationTypeName,
    values: HashMap<String, AnnotationValue>,
    synthesized: bool,
}

impl fmt::Debug for AnnotationMirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnnotationMirror")
            .field("type_name", &self.type_name)
            .field("values", &self.values)
            .field("synthesized", &self.synthesized)
            .finish()
    }
}

impl PartialEq for AnnotationMirror {
    fn eq(&self, other: &Self) -> bool {
        self.type_name == other.type_name && self.values == other.values
    }
}

impl AnnotationMirror {
    /// 构造注解镜像。
    pub fn new(type_name: AnnotationTypeName, values: HashMap<String, AnnotationValue>) -> Self {
        Self {
            type_name,
            values,
            synthesized: false,
        }
    }

    /// 标记为合成注解。
    pub fn mark_synthesized(mut self) -> Self {
        self.synthesized = true;
        self
    }

    /// 是否为合成注解。
    pub fn is_synthesized(&self) -> bool {
        self.synthesized
    }

    /// 注解类型名。
    pub fn annotation_type(&self) -> AnnotationTypeName {
        self.type_name
    }

    /// 读取属性值；缺失时按 schema 默认值填充。
    pub fn get_raw(&self, name: &str) -> Option<&AnnotationValue> {
        self.values.get(name)
    }

    /// 按 schema 解析属性值；缺失时跟随 `@Alias` 指向的属性。
    pub fn resolve_value(&self, schema: &AnnotationSchema, name: &str) -> AnnotationValue {
        if let Some(v) = self.values.get(name) {
            return v.clone();
        }
        // 跟随 AttributeDef 上的 @Alias 元注解（非 JVM 反射）
        if let Some(attr) = schema.attribute(name) {
            for meta in &attr.meta {
                if meta.annotation_type() == "cn.hutool.core.annotation.Alias" {
                    if let Some(AnnotationValue::String(target)) = meta.get_raw("value") {
                        if target != name {
                            return self.resolve_value(schema, target);
                        }
                    }
                }
            }
            return attr.default_value.clone();
        }
        AnnotationValue::Unit
    }

    /// 设置属性值。
    pub fn set_value(&mut self, name: impl Into<String>, value: AnnotationValue) {
        self.values.insert(name.into(), value);
    }

    /// 全部显式设置的属性。
    pub fn explicit_values(&self) -> &HashMap<String, AnnotationValue> {
        &self.values
    }
}

/// 属性引用，对齐 Java `Method`（注解 attribute method）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeRef {
    pub annotation_type: AnnotationTypeName,
    pub name: String,
}

impl AttributeRef {
    /// 创建属性引用。
    pub fn new(annotation_type: AnnotationTypeName, name: impl Into<String>) -> Self {
        Self {
            annotation_type,
            name: name.into(),
        }
    }
}

/// 被注解元素句柄，对齐 Java `AnnotatedElement` 身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementHandle(u64);

impl ElementHandle {
    /// 构造元素句柄。
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 内部 id。
    pub const fn id(self) -> u64 {
        self.0
    }
}

/// JDK 元注解类型集合。
pub fn is_jdk_meta_annotation(type_name: AnnotationTypeName) -> bool {
    matches!(
        type_name,
        "java.lang.annotation.Target"
            | "java.lang.annotation.Retention"
            | "java.lang.annotation.Inherited"
            | "java.lang.annotation.Documented"
            | "java.lang.SuppressWarnings"
            | "java.lang.Override"
            | "java.lang.Deprecated"
    )
}

/// 是否非 JDK 元注解。
pub fn is_not_jdk_meta_annotation(type_name: AnnotationTypeName) -> bool {
    !is_jdk_meta_annotation(type_name)
}

/// 判断值类型是否可赋值给期望类型（简化版 `ClassUtil.isAssignable`）。
pub fn is_assignable(expected: ValueKind, actual: &AnnotationValue) -> bool {
    match (expected, actual.kind()) {
        (ValueKind::Void, ValueKind::Void) => true,
        (ValueKind::String, ValueKind::String) => true,
        (ValueKind::I32, ValueKind::I32) => true,
        (ValueKind::I64, ValueKind::I64) => true,
        (ValueKind::F64, ValueKind::F64) => true,
        (ValueKind::Bool, ValueKind::Bool) => true,
        (ValueKind::Class, ValueKind::Class) => true,
        (ValueKind::Array, ValueKind::Array) => true,
        (ValueKind::Annotation, ValueKind::Annotation) => true,
        (ValueKind::I32, ValueKind::I64) => true,
        _ => false,
    }
}
