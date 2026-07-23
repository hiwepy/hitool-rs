//! 对齐: `cn.hutool.core.annotation.WrappedAnnotationAttribute`

use std::sync::Arc;

use super::annotation_attribute::AnnotationAttribute;

/// 对齐 Java interface: `cn.hutool.core.annotation.WrappedAnnotationAttribute`
pub trait WrappedAnnotationAttribute: AnnotationAttribute {
    /// 获取原始属性。
    fn get_original(&self) -> Arc<dyn AnnotationAttribute>;

    /// 获取关联属性。
    fn get_linked(&self) -> Arc<dyn AnnotationAttribute>;
}
