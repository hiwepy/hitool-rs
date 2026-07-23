//! 对齐: `cn.hutool.core.io.BufferUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/BufferUtil.java
//!
//! Java `ByteBuffer` 工具的切片/Vec 惯用映射（不依赖 JVM NIO Channel）。

/// 对齐 Java 类: `cn.hutool.core.io.BufferUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct BufferUtil;

impl BufferUtil {
    /// 对齐 Java: `BufferUtil.copy(ByteBuffer, int, int)` — 切片拷贝。
    pub fn copy_range(src: &[u8], start: usize, end: usize) -> Vec<u8> {
        let end = end.min(src.len());
        let start = start.min(end);
        src[start..end].to_vec()
    }

    /// 对齐 Java: `BufferUtil.copy(ByteBuffer, ByteBuffer)` / length 重载。
    pub fn copy_into(src: &[u8], dest: &mut [u8]) -> usize {
        let n = src.len().min(dest.len());
        dest[..n].copy_from_slice(&src[..n]);
        n
    }

    /// 对齐 Java: `BufferUtil.copy(src, srcStart, dest, destStart, length)`
    pub fn copy_at(
        src: &[u8],
        src_start: usize,
        dest: &mut [u8],
        dest_start: usize,
        length: usize,
    ) -> usize {
        if src_start >= src.len() || dest_start >= dest.len() {
            return 0;
        }
        let n = length
            .min(src.len() - src_start)
            .min(dest.len() - dest_start);
        dest[dest_start..dest_start + n].copy_from_slice(&src[src_start..src_start + n]);
        n
    }

    /// 对齐 Java: `BufferUtil.readUtf8Str(ByteBuffer)`
    pub fn read_utf8_str(buffer: &[u8]) -> String {
        String::from_utf8_lossy(buffer).into_owned()
    }

    /// 对齐 Java: `BufferUtil.readStr(ByteBuffer, Charset)` — UTF-8。
    pub fn read_str(buffer: &[u8]) -> String {
        Self::read_utf8_str(buffer)
    }

    /// 对齐 Java: `BufferUtil.readBytes(ByteBuffer)` / maxLen 重载。
    pub fn read_bytes(buffer: &[u8]) -> Vec<u8> {
        buffer.to_vec()
    }

    /// 对齐 Java: `BufferUtil.readBytes(ByteBuffer, int)`
    pub fn read_bytes_max(buffer: &[u8], max_len: usize) -> Vec<u8> {
        buffer[..buffer.len().min(max_len)].to_vec()
    }

    /// 对齐 Java: `BufferUtil.readBytes(ByteBuffer, int start, int end)`
    pub fn read_bytes_range(buffer: &[u8], start: usize, end: usize) -> Vec<u8> {
        Self::copy_range(buffer, start, end)
    }

    /// 对齐 Java: `BufferUtil.lineEnd(ByteBuffer)` — `\n` 或 `\r\n` 结束位置。
    pub fn line_end(buffer: &[u8]) -> Option<usize> {
        Self::line_end_max(buffer, buffer.len())
    }

    /// 对齐 Java: `BufferUtil.lineEnd(ByteBuffer, int maxLength)`
    pub fn line_end_max(buffer: &[u8], max_length: usize) -> Option<usize> {
        let limit = buffer.len().min(max_length);
        for i in 0..limit {
            if buffer[i] == b'\n' {
                return Some(i);
            }
            if buffer[i] == b'\r' {
                if i + 1 < limit && buffer[i + 1] == b'\n' {
                    return Some(i + 1);
                }
                return Some(i);
            }
        }
        None
    }

    /// 对齐 Java: `BufferUtil.readLine(ByteBuffer, Charset)` — UTF-8 一行（不含换行）。
    pub fn read_line(buffer: &[u8]) -> Option<String> {
        let end = Self::line_end(buffer)?;
        let mut content_end = end;
        if content_end > 0 && buffer[content_end] == b'\n' && buffer[content_end - 1] == b'\r' {
            content_end -= 1;
        } else if buffer.get(end) == Some(&b'\n') || buffer.get(end) == Some(&b'\r') {
            // end 指向换行符本身
        }
        // end 是换行符位置；内容为 [0, end) 再剥尾部 \r
        let mut slice = &buffer[..end];
        if slice.last() == Some(&b'\r') {
            slice = &slice[..slice.len() - 1];
        }
        if slice.last() == Some(&b'\n') {
            slice = &slice[..slice.len() - 1];
        }
        let _ = content_end;
        Some(String::from_utf8_lossy(slice).into_owned())
    }

    /// 对齐 Java: `BufferUtil.create(byte[])` — 包装为拥有缓冲。
    pub fn create(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }

    /// 对齐 Java: `BufferUtil.create(int capacity)`
    pub fn create_capacity(capacity: usize) -> Vec<u8> {
        vec![0u8; capacity]
    }

    /// 对齐 Java: `BufferUtil.createUtf8(CharSequence)`
    pub fn create_utf8(text: &str) -> Vec<u8> {
        text.as_bytes().to_vec()
    }

    /// 对齐 Java: `BufferUtil.createCharBuffer(int)` — 字符缓冲容量占位。
    pub fn create_char_buffer(capacity: usize) -> String {
        String::with_capacity(capacity)
    }
}
