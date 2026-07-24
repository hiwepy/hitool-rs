//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_attribute::AnnotationAttribute;
use super::hierarchical::Hierarchical;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

use super::synthesized_annotation::SynthesizedAnnotation;

/// 属性值提供者。
pub trait AnnotationAttributeValueProvider {
    /// 按名称与类型获取属性值。
    fn get_attribute_value_typed(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue>;
}

impl<T: SynthesizedAnnotation> AnnotationAttributeValueProvider for T {
    fn get_attribute_value_typed(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue> {
        if self.has_attribute(attribute_name, attribute_type) {
            self.get_attribute_value(attribute_name)
        } else {
            None
        }
    }
}
