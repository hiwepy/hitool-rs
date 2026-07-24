//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

use std::sync::Arc;

use super::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use super::annotation_attribute::AnnotationAttribute;
use super::mirror::AnnotationValue;
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

mod mirror_value_conflict_error;
mod mirrored_annotation_attribute;

pub use mirror_value_conflict_error::MirrorValueConflictError;
pub use mirrored_annotation_attribute::MirroredAnnotationAttribute;
