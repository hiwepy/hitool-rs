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

mod annotation_util;
mod annotation_scanner_ext;

pub use annotation_util::AnnotationUtil;
pub use annotation_scanner_ext::AnnotationScannerExt;
pub use annotation_util::mirror_string;
pub use annotation_util::mirror_class_name;
pub use annotation_util::value_kind_of_name;
