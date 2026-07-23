//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::hierarchical::default_hierarchical_cmp;
use super::mirror::{AnnotationValue, ValueKind};
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;

/// 对齐 Java 类: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`
pub struct CacheableSynthesizedAnnotationAttributeProcessor {
    cache: RwLock<HashMap<(String, ValueKind), AnnotationValue>>,
}

impl Default for CacheableSynthesizedAnnotationAttributeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheableSynthesizedAnnotationAttributeProcessor {
    /// 创建带缓存的处理器。
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }
}

impl SynthesizedAnnotationAttributeProcessor for CacheableSynthesizedAnnotationAttributeProcessor {
    fn get_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
        synthesized_annotations: &[Arc<dyn SynthesizedAnnotation>],
    ) -> Option<AnnotationValue> {
        let key = (attribute_name.to_string(), attribute_type);
        if let Some(v) = self.cache.read().unwrap().get(&key) {
            return Some(v.clone());
        }
        let mut candidates: Vec<&Arc<dyn SynthesizedAnnotation>> = synthesized_annotations
            .iter()
            .filter(|ma| ma.has_attribute(attribute_name, attribute_type))
            .collect();
        candidates.sort_by(|a, b| default_hierarchical_cmp(a.as_ref(), b.as_ref()));
        let value = candidates
            .first()
            .and_then(|ma| ma.get_attribute_value(attribute_name));
        if let Some(v) = value.clone() {
            self.cache.write().unwrap().insert(key, v);
        }
        value
    }
}

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
