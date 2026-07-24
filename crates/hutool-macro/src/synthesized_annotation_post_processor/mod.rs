//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`

use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::synthesized_annotation::SynthesizedAnnotation;

mod synthesized_annotation_post_processor;
mod post_processors;

pub use synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;
pub use post_processors::PostProcessors;
