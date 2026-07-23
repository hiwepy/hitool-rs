//! `FilterIter` 对比验证测试 —— 对齐 Hutool `FilterIterTest`
//!
//! 对齐: `cn.hutool.core.collection.FilterIterTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/FilterIterTest.java

/// 对齐 Java: `FilterIterTest.checkFilterIter()`
#[test]
fn check_filter_iter() {
    let it = ["1", "2"].into_iter();
    // filter=null → 全部通过
    let filtered: Vec<_> = it.collect();
    assert_eq!(filtered.len(), 2);

    let it = ["1", "2"].into_iter();
    let filtered: Vec<_> = it.filter(|k| *k == "1").collect();
    assert_eq!(filtered.len(), 1);
}
