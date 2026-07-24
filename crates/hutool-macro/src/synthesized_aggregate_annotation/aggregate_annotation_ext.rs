//! 对齐: `cn.hutool.core.annotation.SynthesizedAggregateAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

use super::synthesized_aggregate_annotation::SynthesizedAggregateAnnotation;

/// 聚合注解便捷 trait 别名。
pub trait AggregateAnnotationExt: SynthesizedAggregateAnnotation {
    /// 获取属性值（默认实现委托 attribute processor）。
    fn aggregate_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue> {
        self.get_attribute_value(attribute_name, attribute_type)
    }
}

impl<T: SynthesizedAggregateAnnotation> AggregateAnnotationExt for T {}
