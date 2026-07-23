//! 对齐: `cn.hutool.core.io.file.LineSeparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/LineSeparator.java

/// 对齐 Java enum: `cn.hutool.core.io.file.LineSeparator`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineSeparator {
    /// 对齐 Java 枚举常量: `MAC` — `\r`
    Mac,
    /// 对齐 Java 枚举常量: `LINUX` — `\n`
    Linux,
    /// 对齐 Java 枚举常量: `WINDOWS` — `\r\n`
    Windows,
}

impl LineSeparator {
    /// 对齐 Java: `LineSeparator.getValue()`
    pub fn value(self) -> &'static str {
        match self {
            Self::Mac => "\r",
            Self::Linux => "\n",
            Self::Windows => "\r\n",
        }
    }
}
