//! 对齐: `cn.hutool.core.math.BitStatusUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/BitStatusUtil.java
//!
//! 通过位运算表示状态；参数必须 ≥0 且为偶数。

/// 对齐 Java 类: `cn.hutool.core.math.BitStatusUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct BitStatusUtil;

impl BitStatusUtil {
    /// 对齐 Java: `add(int states, int stat)`
    #[must_use]
    pub fn add(states: i32, stat: i32) -> i32 {
        Self::check(&[states, stat]);
        states | stat
    }

    /// 对齐 Java: `has(int states, int stat)`
    #[must_use]
    pub fn has(states: i32, stat: i32) -> bool {
        Self::check(&[states, stat]);
        (states & stat) == stat
    }

    /// 对齐 Java: `remove(int states, int stat)`
    #[must_use]
    pub fn remove(states: i32, stat: i32) -> i32 {
        Self::check(&[states, stat]);
        if Self::has(states, stat) {
            states ^ stat
        } else {
            states
        }
    }

    /// 对齐 Java: `clear()`
    #[must_use]
    pub fn clear() -> i32 {
        0
    }

    /// 对齐 Java: `check(int... args)` —— 必须 ≥0 且为偶数。
    fn check(args: &[i32]) {
        for &arg in args {
            assert!(arg >= 0, "{arg} 必须大于等于0");
            assert!(arg & 1 == 0, "{arg} 不是偶数");
        }
    }
}
