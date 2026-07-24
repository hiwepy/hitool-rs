//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::hierarchical::default_hierarchical_cmp;
use super::mirror::{AnnotationValue, ValueKind};
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;

mod cacheable_synthesized_annotation_attribute_processor;
mod test_value_synthesized_annotation;

pub use cacheable_synthesized_annotation_attribute_processor::CacheableSynthesizedAnnotationAttributeProcessor;
pub use test_value_synthesized_annotation::TestValueSynthesizedAnnotation;
