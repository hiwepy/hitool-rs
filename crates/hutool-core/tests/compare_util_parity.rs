//! `CompareUtil` 对比验证测试 —— 对齐 Hutool `CompareUtilTest`
//!
//! 对齐: `cn.hutool.core.comparator.CompareUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/comparator/CompareUtilTest.java

use hutool_core::CompareUtil;

/// 对齐 Java: `CompareUtilTest.compareTest()` (行 11-18)
#[test]
fn compare_test() {
    let cmp = CompareUtil::compare::<&str>(None, Some("a"), true);
    assert!(
        cmp > 0,
        "compare(null, \"a\", nullGreater=true) 应 > 0, 实际 {cmp} (对齐 Java)"
    );

    let cmp = CompareUtil::compare::<&str>(None, Some("a"), false);
    assert!(
        cmp < 0,
        "compare(null, \"a\", nullGreater=false) 应 < 0, 实际 {cmp} (对齐 Java)"
    );
}

/// 对齐 Java: `CompareUtilTest.comparingPinyin()` (行 20-34)
#[test]
fn comparing_pinyin() {
    let mut list = vec!["成都", "北京", "上海", "深圳"];
    let ascending = ["北京", "成都", "上海", "深圳"];
    let descending = ["深圳", "上海", "成都", "北京"];

    list.sort_by(CompareUtil::comparing_pinyin(|s| *s));
    assert_eq!(list, ascending, "拼音正序 (对齐 Java)");

    list.sort_by(CompareUtil::comparing_pinyin_reverse(|s| *s, true));
    assert_eq!(list, descending, "拼音反序 (对齐 Java)");
}
