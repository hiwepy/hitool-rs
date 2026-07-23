//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`

use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`
pub trait SynthesizedAnnotationPostProcessor: Send + Sync {
    /// 处理顺序。
    fn order(&self) -> i32;

    /// 处理合成注解。
    fn process(
        &self,
        synthesized_annotation: Arc<dyn SynthesizedAnnotation>,
        synthesizer: &dyn AnnotationSynthesizer,
    );
}

/// 预置后置处理器工厂。
pub struct PostProcessors;

impl PostProcessors {
    /// Alias 处理器。
    pub fn alias_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(super::alias_annotation_post_processor::AliasAnnotationPostProcessor)
    }

    /// MirrorLink 处理器。
    pub fn mirror_link_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(super::mirror_link_annotation_post_processor::MirrorLinkAnnotationPostProcessor)
    }

    /// AliasLink 处理器。
    pub fn alias_link_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(super::alias_link_annotation_post_processor::AliasLinkAnnotationPostProcessor)
    }
}
