//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use super::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use super::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`
pub trait SynthesizedAnnotationSelector: Send + Sync {
    /// 比较两个合成注解并返回选中项。
    fn choose(
        &self,
        old_annotation: Arc<dyn SynthesizedAnnotation>,
        new_annotation: Arc<dyn SynthesizedAnnotation>,
    ) -> Arc<dyn SynthesizedAnnotation>;
}

struct SelectorAdapter {
    inner: Arc<dyn HierarchicalSelector>,
}

impl SynthesizedAnnotationSelector for SelectorAdapter {
    fn choose(
        &self,
        old_annotation: Arc<dyn SynthesizedAnnotation>,
        new_annotation: Arc<dyn SynthesizedAnnotation>,
    ) -> Arc<dyn SynthesizedAnnotation> {
        match self.inner.choose(old_annotation.as_ref(), new_annotation.as_ref()) {
            ChooseSide::Next => new_annotation,
            ChooseSide::Prev => old_annotation,
        }
    }
}

fn wrap(selector: Arc<dyn HierarchicalSelector>) -> Arc<dyn SynthesizedAnnotationSelector> {
    Arc::new(SelectorAdapter { inner: selector })
}

/// 预置选择器工厂。
pub struct Selectors;

impl Selectors {
    /// 更近且更旧优先。
    pub fn nearest_and_oldest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(NearestAndOldestPrioritySelector))
    }

    /// 更近且更新优先。
    pub fn nearest_and_newest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(NearestAndNewestPrioritySelector))
    }

    /// 更远且更旧优先。
    pub fn farthest_and_oldest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(FarthestAndOldestPrioritySelector))
    }

    /// 更远且更新优先。
    pub fn farthest_and_newest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(FarthestAndNewestPrioritySelector))
    }
}

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
