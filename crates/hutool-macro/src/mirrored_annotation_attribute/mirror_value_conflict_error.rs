//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

use std::sync::Arc;

use super::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use super::annotation_attribute::AnnotationAttribute;
use super::mirror::AnnotationValue;
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

/// 镜像属性值冲突异常。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorValueConflictError {
    pub message: String,
}

impl std::fmt::Display for MirrorValueConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MirrorValueConflictError {}
