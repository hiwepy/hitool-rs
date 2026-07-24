//! 对齐: `cn.hutool.core.annotation.scanner.AnnotationScanner`

use std::sync::Arc;

use crate::element::{global_registry, AnnotatedElement, ElementHandle};
use crate::mirror::{is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName};

use super::scan_consumer::ScanConsumer;

/// 对齐 Java interface: `cn.hutool.core.annotation.scanner.AnnotationScanner`
pub trait AnnotationScanner: Send + Sync {
    /// 是否支持扫描该元素。
    fn support(&self, element: ElementHandle) -> bool {
        false
    }

    /// 是否支持扫描注解类型（元注解扫描）。
    fn support_type(&self, _annotation_type: AnnotationTypeName) -> bool {
        false
    }

    /// 扫描元素。
    fn scan<'a>(&self, consumer: &mut ScanConsumer<'a>, element: ElementHandle);

    /// 扫描注解类型元注解。
    fn scan_meta<'a>(&self, _annotation_type: AnnotationTypeName, _consumer: &mut ScanConsumer<'a>) {}

    /// 获取全部注解。
    fn get_annotations(&self, element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        let mut list = Vec::new();
        if self.support(element) {
            let list_ref = &mut list;
            let mut consumer: ScanConsumer<'_> =
                Box::new(move |_index, annotation| list_ref.push(annotation));
            self.scan(&mut consumer, element);
        }
        list
    }
}
