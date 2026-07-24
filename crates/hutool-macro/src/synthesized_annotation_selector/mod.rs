//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use super::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use super::synthesized_annotation::SynthesizedAnnotation;

mod synthesized_annotation_selector;
mod selectors;
mod test_synthesized_annotation;

pub use synthesized_annotation_selector::SynthesizedAnnotationSelector;
pub use selectors::Selectors;
pub use test_synthesized_annotation::TestSynthesizedAnnotation;
