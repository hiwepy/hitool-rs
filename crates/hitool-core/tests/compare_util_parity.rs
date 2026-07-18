//! `CompareUtil` 对比验证测试 —— 对齐 Hutool `CompareUtilTest`
//!
//! 对齐: `cn.hutool.core.comparator.CompareUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/comparator/CompareUtilTest.java
//!
//! # 当前状态
//!
//! hitool `comparator::compare_util` 仍是空对齐桩。Java `CompareUtil.compare`
//! 的 null 优先/后置语义可用 Rust `Option::cmp` 直接表达。
//! `comparingPinyin`(中文拼音排序)依赖拼音字典,Rust 版本暂无对应实现,
//! 标注为 `#[ignore]`。

use std::cmp::Ordering;

/// 模拟 Java `CompareUtil.compare(c1, c2, isNullGreater)` 语义。
///
/// - `c1`/`c2` 均为 `Some` 时按 `T: Ord` 比较
/// - 含 `None` 时:`isNullGreater=true` 则 None 视为更大;`false` 则视为更小
fn compare_opt<T: Ord>(c1: Option<T>, c2: Option<T>, null_greater: bool) -> i32 {
    match (c1, c2) {
        (Some(a), Some(b)) => match a.cmp(&b) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        },
        (Some(_), None) => {
            // c2 为 null
            if null_greater { -1 } else { 1 }
        }
        (None, Some(_)) => {
            // c1 为 null
            if null_greater { 1 } else { -1 }
        }
        (None, None) => 0,
    }
}

/// 对齐 Java: `CompareUtilTest.compareTest()` (行 11-18)
///
/// ```java
/// int compare = CompareUtil.compare(null, "a", true);
/// assertTrue(compare > 0);
/// compare = CompareUtil.compare(null, "a", false);
/// assertTrue(compare < 0);
/// ```
#[test]
fn compare_test() {
    // null, "a", isNullGreater=true → null 视为更大 → compare > 0
    let cmp = compare_opt::<&str>(None, Some("a"), true);
    assert!(
        cmp > 0,
        "compare(null, \"a\", nullGreater=true) 应 > 0, 实际 {cmp} (对齐 Java)"
    );

    // null, "a", isNullGreater=false → null 视为更小 → compare < 0
    let cmp = compare_opt::<&str>(None, Some("a"), false);
    assert!(
        cmp < 0,
        "compare(null, \"a\", nullGreater=false) 应 < 0, 实际 {cmp} (对齐 Java)"
    );
}

/// 对齐 Java: `CompareUtilTest.comparingPinyin()` (行 20-34)
///
/// 中文按拼音排序:
/// - 正序: ["北京", "成都", "上海", "深圳"](BJ, CD, SH, SZ)
/// - 反序: ["深圳", "上海", "成都", "北京"]
///
/// **状态**: hitool `CompareUtil::comparing_pinyin` 未实现(需拼音字典库),
/// 标注 `#[ignore]`,待引入拼音依赖后启用。
#[test]
#[ignore = "等待 hitool_core::comparator::compare_util 实现 comparing_pinyin (需拼音字典)"]
fn comparing_pinyin() {
    // 期望正序:["北京", "成都", "上海", "深圳"]
    // 期望反序:["深圳", "上海", "成都", "北京"]
}