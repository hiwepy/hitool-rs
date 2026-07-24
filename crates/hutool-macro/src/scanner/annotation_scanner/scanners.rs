//! 对齐: `cn.hutool.core.annotation.scanner.AnnotationScanner`

use std::sync::Arc;

use crate::element::{global_registry, AnnotatedElement, ElementHandle};
use crate::mirror::{is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName};

use super::annotation_scanner::AnnotationScanner;

/// 预置扫描器常量访问。
pub struct Scanners;

impl Scanners {
    /// NOTHING
    pub fn nothing() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::empty_annotation_scanner::EmptyAnnotationScanner)
    }

    /// DIRECTLY
    pub fn directly() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, false, false,
        ))
    }

    /// DIRECTLY_AND_META_ANNOTATION
    pub fn directly_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, false, false,
        ))
    }

    /// TYPE_HIERARCHY
    pub fn type_hierarchy() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, true, true,
        ))
    }

    /// TYPE_HIERARCHY_AND_META_ANNOTATION
    pub fn type_hierarchy_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, true, true,
        ))
    }
}
