//! 对齐: `cn.hutool.core.io.FastByteArrayOutputStream`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FastByteArrayOutputStream.java
//!
//! 基于 [`super::fast_byte_buffer::FastByteBuffer`] 的可增长输出缓冲。

use super::fast_byte_buffer::FastByteBuffer;
use std::io::{self, Write};

/// 对齐 Java 类: `cn.hutool.core.io.FastByteArrayOutputStream`
#[derive(Debug, Clone)]
pub struct FastByteArrayOutputStream {
    buffer: FastByteBuffer,
}

impl Default for FastByteArrayOutputStream {
    fn default() -> Self {
        Self::new()
    }
}

impl FastByteArrayOutputStream {
    /// 对齐 Java: `FastByteArrayOutputStream()`
    pub fn new() -> Self {
        Self {
            buffer: FastByteBuffer::new(),
        }
    }

    /// 对齐 Java: `FastByteArrayOutputStream(int size)`
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buffer: FastByteBuffer::with_chunk(size),
        }
    }

    /// 对齐 Java: `write(int)`
    pub fn write_u8(&mut self, b: u8) {
        self.buffer.append_u8(b);
    }

    /// 对齐 Java: `write(byte[], int, int)`
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.buffer.append(data);
    }

    /// 对齐 Java: `size()`
    pub fn size(&self) -> usize {
        self.buffer.size()
    }

    /// 对齐 Java: `close()` — no-op（内存缓冲）。
    pub fn close(&self) {}

    /// 对齐 Java: `reset()`
    pub fn reset(&mut self) {
        self.buffer.reset();
    }

    /// 对齐 Java: `writeTo(OutputStream)`
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.buffer.as_slice())
    }

    /// 对齐 Java: `toByteArray()`
    pub fn to_byte_array(&self) -> Vec<u8> {
        self.buffer.to_array()
    }

    /// 对齐 Java: `toString()` / `toString(Charset)` — UTF-8。
    pub fn to_utf8_string(&self) -> String {
        String::from_utf8_lossy(self.buffer.as_slice()).into_owned()
    }
}

impl Write for FastByteArrayOutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.append(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
