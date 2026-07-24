//! 对齐: `cn.hutool.core.annotation.AnnotationUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/AnnotationUtil.java

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::combination_annotation_element::to_combination as combination_of;
use super::element::{global_registry, ElementHandle};
use super::generic_synthesized_aggregate_annotation::GenericSynthesizedAggregateAnnotation;
use super::mirror::{
    is_jdk_meta_annotation, is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName,
    AnnotationValue, ValueKind,
};
use super::scanner::annotation_scanner::{AnnotationScanner, Scanners};
use super::scanner::meta_annotation_scanner::MetaAnnotationScanner;

/// AnnotationScanner 扩展。
pub trait AnnotationScannerExt: AnnotationScanner {
    /// 扫描注解类型并返回列表。
    fn get_annotations_for_type(&self, annotation_type: AnnotationTypeName) -> Vec<Arc<AnnotationMirror>> {
        MetaAnnotationScanner::new(true).get_meta_annotations(annotation_type)
    }
}

impl<T: AnnotationScanner + ?Sized> AnnotationScannerExt for T {}
