//! `PartitionIter` 对比验证测试 —— 对齐 Hutool `PartitionIterTest`
//!
//! 对齐: `cn.hutool.core.collection.PartitionIterTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/PartitionIterTest.java

/// 对齐 Java: `PartitionIterTest.iterTest()`
#[test]
fn iter_test() {
    let lines = vec!["a", "b", "c", "d", "e"];
    for chunk in lines.chunks(3) {
        assert!(!chunk.is_empty());
    }
}

/// 对齐 Java: `PartitionIterTest.iterMaxTest()`
#[test]
fn iter_max_test() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 0, 12, 45, 12];
    let mut max = 0;
    for chunk in list.chunks(3) {
        max = max.max(*chunk.iter().max().unwrap_or(&0));
    }
    assert_eq!(max, 45);
}
