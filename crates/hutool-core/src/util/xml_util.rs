//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! 真正实现位于 crate 根模块；此模块保留 Java 包结构兼容路径。

pub use crate::{
    transform_xml, visit_xml, NamespaceMode, XmlChild, XmlDocument, XmlEventReader, XmlEventWriter,
    XmlNode, XmlParseOptions, XmlTransformAction, XmlUtil,
};

#[cfg(feature = "xml-serde")]
pub use crate::XmlSerde;
