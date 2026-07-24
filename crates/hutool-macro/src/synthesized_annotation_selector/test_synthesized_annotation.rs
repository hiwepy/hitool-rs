//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use super::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use super::synthesized_annotation::SynthesizedAnnotation;

/// 测试用合成注解桩。
pub struct TestSynthesizedAnnotation {
    vertical_distance: i32,
    horizontal_distance: i32,
    id: u64,
}

impl TestSynthesizedAnnotation {
    /// 创建测试桩。
    pub fn new(vertical_distance: i32, horizontal_distance: i32, id: u64) -> Arc<Self> {
        Arc::new(Self {
            vertical_distance,
            horizontal_distance,
            id,
        })
    }
}

impl Hierarchical for TestSynthesizedAnnotation {
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

impl PartialEq for TestSynthesizedAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl SynthesizedAnnotation for TestSynthesizedAnnotation {
    fn get_annotation(&self) -> Arc<super::mirror::AnnotationMirror> {
        Arc::new(super::mirror::AnnotationMirror::new("test.Test", Default::default()))
    }
    fn has_attribute(&self, _attribute_name: &str, _return_type: super::mirror::ValueKind) -> bool {
        false
    }
    fn get_attributes(&self) -> std::collections::HashMap<String, Arc<dyn super::annotation_attribute::AnnotationAttribute>> {
        Default::default()
    }
    fn set_attribute(&self, _attribute_name: &str, _attribute: Arc<dyn super::annotation_attribute::AnnotationAttribute>) {}
    fn replace_attribute(
        &self,
        _attribute_name: &str,
        _operator: Box<dyn Fn(Arc<dyn super::annotation_attribute::AnnotationAttribute>) -> Arc<dyn super::annotation_attribute::AnnotationAttribute> + Send + Sync>,
    ) {
    }
    fn get_attribute_value(&self, _attribute_name: &str) -> Option<super::mirror::AnnotationValue> {
        None
    }
    fn annotation_type(&self) -> super::mirror::AnnotationTypeName {
        "test.Test"
    }
}
