//! 对齐: `cn.hutool.core.annotation.SynthesizedAggregateAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

/// 合成注解映射视图。
pub type SynthesizedAnnotationMap =
