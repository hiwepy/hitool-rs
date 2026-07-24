//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use super::mirror::ElementHandle;

/// 字段元素。
#[derive(Debug, Clone)]
pub struct FieldElement {
    pub handle: ElementHandle,
    pub name: String,
    pub declaring_type: ElementHandle,
    pub annotations: Vec<Arc<AnnotationMirror>>,
}
