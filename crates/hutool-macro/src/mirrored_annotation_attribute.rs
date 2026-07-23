//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

use std::sync::Arc;

use super::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use super::annotation_attribute::AnnotationAttribute;
use super::mirror::AnnotationValue;
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

/// 镜像属性值冲突异常。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorValueConflictError {
    pub message: String,
}

impl std::fmt::Display for MirrorValueConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MirrorValueConflictError {}

/// 对齐 Java 类: `cn.hutool.core.annotation.MirroredAnnotationAttribute`
pub struct MirroredAnnotationAttribute {
    inner: Arc<AbstractWrappedAnnotationAttribute>,
}

impl MirroredAnnotationAttribute {
    /// 构造镜像属性。
    pub fn new(original: Arc<dyn AnnotationAttribute>, linked: Arc<dyn AnnotationAttribute>) -> Arc<Self> {
        Arc::new(Self {
            inner: AbstractWrappedAnnotationAttribute::new(
                original,
                linked,
                mirror_value,
                |original, linked| {
                    original.is_value_equivalent_to_default_value()
                        && linked.is_value_equivalent_to_default_value()
                },
            ),
        })
    }

    /// 读取镜像属性值，冲突时返回 Err。
    pub fn try_get_value(&self) -> Result<AnnotationValue, MirrorValueConflictError> {
        mirror_value_result(&self.inner.get_original(), &self.inner.get_linked())
    }
}

fn mirror_value(original: &Arc<dyn AnnotationAttribute>, linked: &Arc<dyn AnnotationAttribute>) -> AnnotationValue {
    mirror_value_result(original, linked).unwrap_or_else(|e| panic!("{}", e.message))
}

fn mirror_value_result(
    original: &Arc<dyn AnnotationAttribute>,
    linked: &Arc<dyn AnnotationAttribute>,
) -> Result<AnnotationValue, MirrorValueConflictError> {
    let origin_default = original.is_value_equivalent_to_default_value();
    let target_default = linked.is_value_equivalent_to_default_value();
    let origin_value = original.get_value();
    let target_value = linked.get_value();

    if origin_default == target_default {
        if origin_value != target_value {
            return Err(MirrorValueConflictError {
                message: format!(
                    "mirror values differ: {:?} <==> {:?}",
                    origin_value, target_value
                ),
            });
        }
        return Ok(origin_value);
    }
    Ok(if origin_default {
        target_value
    } else {
        origin_value
    })
}

impl AnnotationAttribute for MirroredAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "MirroredAnnotationAttribute"
    }
    fn get_annotation(&self) -> Arc<super::mirror::AnnotationMirror> {
        self.inner.get_annotation()
    }
    fn get_attribute(&self) -> super::mirror::AttributeRef {
        self.inner.get_attribute()
    }
    fn get_value(&self) -> AnnotationValue {
        self.inner.get_value()
    }
    fn is_value_equivalent_to_default_value(&self) -> bool {
        self.inner.is_value_equivalent_to_default_value()
    }
    fn get_attribute_type(&self) -> super::mirror::ValueKind {
        self.inner.get_attribute_type()
    }
    fn get_meta_annotation(
        &self,
        type_name: super::mirror::AnnotationTypeName,
    ) -> Option<Arc<super::mirror::AnnotationMirror>> {
        self.inner.get_meta_annotation(type_name)
    }
    fn is_wrapped(&self) -> bool {
        true
    }

    fn as_wrapped(&self) -> Option<&dyn WrappedAnnotationAttribute> {
        Some(self)
    }
}

impl WrappedAnnotationAttribute for MirroredAnnotationAttribute {
    fn get_original(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_original()
    }
    fn get_linked(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_linked()
    }
}
