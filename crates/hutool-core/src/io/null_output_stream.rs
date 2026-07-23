//! 对齐: `cn.hutool.core.io.NullOutputStream`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/NullOutputStream.java
//!
//! 丢弃全部写出数据（`/dev/null` 语义）。

use std::io::{self, Write};

/// 对齐 Java 类: `cn.hutool.core.io.NullOutputStream`
#[derive(Debug, Clone, Copy, Default)]
pub struct NullOutputStream;

impl NullOutputStream {
    /// 构造丢弃流。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `write(int)` — 忽略。
    pub fn write_u8(&self, _b: u8) {}

    /// 对齐 Java: `write(byte[])` — 忽略。
    pub fn write_bytes(&self, _data: &[u8]) {}

    /// 对齐 Java: `write(byte[], int, int)` — 忽略。
    pub fn write_range(&self, _data: &[u8], _off: usize, _len: usize) {}
}

impl Write for NullOutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
