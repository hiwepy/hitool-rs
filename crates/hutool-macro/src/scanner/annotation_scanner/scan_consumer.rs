//! 对齐: `cn.hutool.core.annotation.scanner.AnnotationScanner`

use std::sync::Arc;

use crate::element::{global_registry, AnnotatedElement, ElementHandle};
use crate::mirror::{is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName};

/// 扫描回调。
pub type ScanConsumer<'a> = Box<dyn FnMut(i32, Arc<AnnotationMirror>) + 'a>;
