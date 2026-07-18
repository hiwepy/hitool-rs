//! `cn.hutool.core.annotation` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod abstract_annotation_synthesizer;
pub mod abstract_link_annotation_post_processor;
pub mod abstract_wrapped_annotation_attribute;
pub mod aggregate_annotation;
pub mod alias;
pub mod alias_annotation_post_processor;
pub mod alias_for;
pub mod alias_link_annotation_post_processor;
pub mod aliased_annotation_attribute;
pub mod annotation_attribute;
pub mod annotation_attribute_value_provider;
pub mod annotation_proxy;
pub mod annotation_synthesizer;
pub mod annotation_util;
pub mod cacheable_annotation_attribute;
pub mod cacheable_synthesized_annotation_attribute_processor;
pub mod combination_annotation_element;
pub mod force_alias_for;
pub mod force_aliased_annotation_attribute;
pub mod generic_synthesized_aggregate_annotation;
pub mod generic_synthesized_annotation;
pub mod hierarchical;
pub mod link;
pub mod mirror_for;
pub mod mirror_link_annotation_post_processor;
pub mod mirrored_annotation_attribute;
pub mod prop_ignore;
pub mod relation_type;
pub mod synthesized_aggregate_annotation;
pub mod synthesized_annotation;
pub mod synthesized_annotation_attribute_processor;
pub mod synthesized_annotation_post_processor;
pub mod synthesized_annotation_proxy;
pub mod synthesized_annotation_selector;
pub mod wrapped_annotation_attribute;
pub mod scanner;
