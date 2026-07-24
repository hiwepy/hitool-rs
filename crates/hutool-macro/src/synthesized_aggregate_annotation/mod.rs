//! 对齐: `cn.hutool.core.annotation.SynthesizedAggregateAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

mod synthesized_aggregate_annotation;
mod aggregate_annotation_ext;
mod synthesized_annotation_map;

pub use synthesized_aggregate_annotation::SynthesizedAggregateAnnotation;
pub use aggregate_annotation_ext::AggregateAnnotationExt;
pub use synthesized_annotation_map::SynthesizedAnnotationMap;
