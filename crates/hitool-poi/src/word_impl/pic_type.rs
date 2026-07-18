//! Word picture type enumeration aligned with Hutool.
//!
//! 对齐: `cn.hutool.poi.word.PicType`
//! 来源: hutool-poi/src/main/java/cn/hutool/poi/word/PicType.java
//!
//! `PicType` 是 Hutool 定义的图片扩展名/Content-Type 映射枚举。

/// Word picture type enumeration.
///
/// 对齐 Java: `cn.hutool.poi.word.PicType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PicType {
    /// 对齐 Java: `PicType.JPEG`
    Jpeg,
    /// 对齐 Java: `PicType.PNG`
    Png,
    /// 对齐 Java: `PicType.GIF`
    Gif,
    /// 对齐 Java: `PicType.BMP`
    Bmp,
    /// 对齐 Java: `PicType.TIFF`
    Tiff,
    /// 对齐 Java: `PicType.WMF`
    Wmf,
    /// 对齐 Java: `PicType.SVG`
    Svg,
    /// 对齐 Java: `PicType.EMF`
    Emf,
}

impl PicType {
    /// 对齐 Java: `PicType.values()`
    pub fn values() -> &'static [PicType] {
        &[
            PicType::Jpeg, PicType::Png, PicType::Gif, PicType::Bmp,
            PicType::Tiff, PicType::Wmf, PicType::Svg, PicType::Emf,
        ]
    }
    /// 对齐 Java: `PicType.getValue()` (扩展名 / 文件后缀)
    pub fn get_value(&self) -> &'static str {
        match self {
            PicType::Jpeg => ".jpg",
            PicType::Png => ".png",
            PicType::Gif => ".gif",
            PicType::Bmp => ".bmp",
            PicType::Tiff => ".tif",
            PicType::Wmf => ".wmf",
            PicType::Svg => ".svg",
            PicType::Emf => ".emf",
        }
    }
}