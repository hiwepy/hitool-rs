//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationAttributeProcessor`

use std::sync::Arc;

use super::mirror::{AnnotationValue, ValueKind};
use super::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotationAttributeProcessor`
pub trait SynthesizedAnnotationAttributeProcessor: Send + Sync {
    /// 从多个合成注解中获取属性值。
    fn get_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
        synthesized_annotations: &[Arc<dyn SynthesizedAnnotation>],
    ) -> Option<AnnotationValue>;
}
