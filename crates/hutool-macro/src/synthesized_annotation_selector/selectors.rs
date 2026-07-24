//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use super::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use super::synthesized_annotation::SynthesizedAnnotation;

use super::synthesized_annotation_selector::SynthesizedAnnotationSelector;

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

fn wrap(selector: Arc<dyn HierarchicalSelector>) -> Arc<dyn SynthesizedAnnotationSelector> {
    Arc::new(SelectorAdapter { inner: selector })
}
