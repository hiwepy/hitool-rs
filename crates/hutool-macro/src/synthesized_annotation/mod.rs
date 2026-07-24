//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_attribute::AnnotationAttribute;
use super::hierarchical::Hierarchical;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

mod synthesized_annotation;
mod annotation_attribute_value_provider;

pub use synthesized_annotation::SynthesizedAnnotation;
pub use annotation_attribute_value_provider::AnnotationAttributeValueProvider;
