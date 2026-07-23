//! 对齐: `cn.hutool.core.annotation.Link`

use std::sync::Arc;

use super::mirror::AnnotationMirror;
use super::relation_type::RelationType;

/// `@Link` 元注解类型名。
pub const LINK_TYPE: &str = "cn.hutool.core.annotation.Link";

/// 对齐 Java `@interface Link` 的运行时表示。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    annotation_type: String,
    attribute: String,
    relation_type: RelationType,
}

impl Link {
    /// 从注解镜像解析 Link。
    pub fn from_mirror(mirror: Arc<AnnotationMirror>) -> Self {
        let annotation_type = mirror
            .get_raw("annotation")
            .and_then(|v| v.as_str())
            .unwrap_or("java.lang.annotation.Annotation")
            .to_string();
        let attribute = mirror
            .get_raw("attribute")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let relation_type = mirror
            .get_raw("type")
            .and_then(|v| v.as_str())
            .map(parse_relation)
            .unwrap_or(RelationType::AliasFor);
        Self {
            annotation_type,
            attribute,
            relation_type,
        }
    }

    /// 目标注解类型。
    pub fn annotation_type(&self) -> &str {
        &self.annotation_type
    }

    /// 目标属性名。
    pub fn attribute(&self) -> &str {
        &self.attribute
    }

    /// 关系类型。
    pub fn relation_type(&self) -> RelationType {
        self.relation_type
    }
}

fn parse_relation(s: &str) -> RelationType {
    match s {
        "MIRROR_FOR" | "MirrorFor" | "MIRROR" => RelationType::MirrorFor,
        "FORCE_ALIAS_FOR" | "ForceAliasFor" => RelationType::ForceAliasFor,
        _ => RelationType::AliasFor,
    }
}
