//! 对齐: `cn.hutool.core.lang.hash.Hash32`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/hash/Hash32.java

/// 对齐 Java: `Hash32<T>`
pub trait Hash32<T: ?Sized> {
    /// 对齐 Java: `hash32(T)`
    fn hash32(&self, key: &T) -> i32;
}
