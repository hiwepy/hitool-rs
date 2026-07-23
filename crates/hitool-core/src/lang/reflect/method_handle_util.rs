//! 对齐: `cn.hutool.core.lang.reflect.MethodHandleUtil`

/// 对齐 Java: `MethodHandleUtil` — 用函数指针近似
pub struct MethodHandleUtil;

impl MethodHandleUtil {
    /// 调用无参函数
    pub fn invoke<R, F: Fn() -> R>(f: F) -> R {
        f()
    }

    /// 调用单参函数
    pub fn invoke1<A, R, F: Fn(A) -> R>(f: F, a: A) -> R {
        f(a)
    }
}
