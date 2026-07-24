//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use super::mirror::ElementHandle;

/// 元素种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Type,
    Method,
    Field,
}
