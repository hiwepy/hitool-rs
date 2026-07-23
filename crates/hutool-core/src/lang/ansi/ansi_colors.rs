//! 对齐: `cn.hutool.core.lang.ansi.AnsiColors`

/// 色深
#[derive(Debug, Clone, Copy)]
pub enum BitDepth {
    /// 4-bit
    Four,
    /// 8-bit
    Eight,
}

/// 最近色结果
#[derive(Debug, Clone, Copy)]
pub struct ClosestColor {
    /// ANSI 码
    pub code: u8,
}

impl ClosestColor {
    /// 转为前景/背景码
    pub fn to_ansi_code(self, fore: bool) -> u8 {
        if fore {
            self.code
        } else {
            self.code.saturating_add(10)
        }
    }
}

/// 对齐 Java: `AnsiColors`
pub struct AnsiColors {
    depth: BitDepth,
}

impl AnsiColors {
    /// 构造
    pub fn new(depth: BitDepth) -> Self {
        Self { depth }
    }

    /// 找最近颜色（简化：灰度映射）
    pub fn find_closest(&self, r: u8, g: u8, b: u8) -> ClosestColor {
        let gray = ((r as u16 + g as u16 + b as u16) / 3) as u8;
        let code = match self.depth {
            BitDepth::Four => 30 + (gray / 36).min(7),
            BitDepth::Eight => 16 + (gray / 11).min(23),
        };
        ClosestColor { code }
    }
}
