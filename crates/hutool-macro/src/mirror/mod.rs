//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

mod annotation_type_name;
mod value_kind;
mod annotation_value;
mod attribute_def;
mod annotation_schema;
mod annotation_mirror;
mod attribute_ref;
mod element_handle;

pub use annotation_type_name::AnnotationTypeName;
pub use value_kind::ValueKind;
pub use annotation_value::AnnotationValue;
pub use attribute_def::AttributeDef;
pub use annotation_schema::AnnotationSchema;
pub use annotation_mirror::AnnotationMirror;
pub use attribute_ref::AttributeRef;
pub use element_handle::ElementHandle;
pub use annotation_type_name::is_jdk_meta_annotation;
pub use annotation_type_name::is_not_jdk_meta_annotation;
pub use annotation_type_name::is_assignable;
