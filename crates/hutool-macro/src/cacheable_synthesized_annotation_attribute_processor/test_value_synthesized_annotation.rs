//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::hierarchical::default_hierarchical_cmp;
use super::mirror::{AnnotationValue, ValueKind};
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;

/// 测试用合成注解（带值 map）。
pub struct TestValueSynthesizedAnnotation {
    vertical_distance: i32,
    horizontal_distance: i32,
    values: HashMap<String, AnnotationValue>,
}

impl TestValueSynthesizedAnnotation {
    /// 创建测试合成注解。
    pub fn new(
        vertical_distance: i32,
        horizontal_distance: i32,
        values: HashMap<String, AnnotationValue>,
    ) -> Arc<Self> {
        Arc::new(Self {
            vertical_distance,
            horizontal_distance,
            values,
        })
    }
}

impl super::hierarchical::Hierarchical for TestValueSynthesizedAnnotation {
    fn get_root(&self) -> Option<&dyn std::any::Any> {
        None
    }
    fn get_vertical_distance(&self) -> i32 {
        self.vertical_distance
    }
    fn get_horizontal_distance(&self) -> i32 {
        self.horizontal_distance
    }
}

impl SynthesizedAnnotation for TestValueSynthesizedAnnotation {
    fn get_annotation(&self) -> Arc<super::mirror::AnnotationMirror> {
        Arc::new(super::mirror::AnnotationMirror::new("test.Test", Default::default()))
    }
    fn has_attribute(&self, attribute_name: &str, return_type: ValueKind) -> bool {
        self.values
            .get(attribute_name)
            .map(|v| super::mirror::is_assignable(return_type, v))
            .unwrap_or(false)
    }
    fn get_attributes(&self) -> HashMap<String, Arc<dyn super::annotation_attribute::AnnotationAttribute>> {
        Default::default()
    }
    fn set_attribute(&self, _attribute_name: &str, _attribute: Arc<dyn super::annotation_attribute::AnnotationAttribute>) {}
    fn replace_attribute(
        &self,
        _attribute_name: &str,
        _operator: Box<dyn Fn(Arc<dyn super::annotation_attribute::AnnotationAttribute>) -> Arc<dyn super::annotation_attribute::AnnotationAttribute> + Send + Sync>,
    ) {
    }
    fn get_attribute_value(&self, attribute_name: &str) -> Option<AnnotationValue> {
        self.values.get(attribute_name).cloned()
    }
    fn annotation_type(&self) -> super::mirror::AnnotationTypeName {
        "test.Test"
    }
}
