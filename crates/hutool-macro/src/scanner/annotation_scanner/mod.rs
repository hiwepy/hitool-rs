//! 对齐: `cn.hutool.core.annotation.scanner.AnnotationScanner`

use std::sync::Arc;

use crate::element::{global_registry, AnnotatedElement, ElementHandle};
use crate::mirror::{is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName};

mod scan_consumer;
mod annotation_scanner;
mod scanners;

pub use scan_consumer::ScanConsumer;
pub use annotation_scanner::AnnotationScanner;
pub use scanners::Scanners;
pub use scan_consumer::accept_annotation;
pub use scan_consumer::declared_annotations;
pub use scan_consumer::element_exists;
pub use scan_consumer::element_kind;
