//! `cn.hutool.core.comparator` 子包对比验证测试 (6 个测试文件, 23 个 @Test)
//! 来源: hutool-core/src/test/java/cn/hutool/core/comparator/

use hutool_core::comparator::{
    IndexedComparator, PropertyComparator, VersionComparator, WindowsExplorerStringComparator,
};

// ===== VersionComparatorTest (12 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.comparator.VersionComparatorTest.versionComparatorTest1()`
#[test]
fn version_comparator_version_comparator_test1() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("1.2.1"), Some("1.12.1")) < 0);
    assert!(cmp.compare(Some("1.12.1"), Some("1.2.1")) > 0);
}

// ===== PropertyComparatorTest (2 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.comparator.PropertyComparatorTest.sortNullTest()`
#[test]
fn property_comparator_sort_null_test() {
    #[derive(Clone)]
    struct Row {
        name: Option<String>,
    }
    let cmp = PropertyComparator::new(|r: &Row| r.name.clone());
    let a = Row { name: None };
    let b = Row {
        name: Some("b".into()),
    };
    assert!(cmp.compare(&a, &b) > 0);
    assert!(cmp.compare(&b, &a) < 0);
}

// ===== IndexedComparatorTest (3 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.comparator.IndexedComparatorTest.sortTest()`
#[test]
fn indexed_comparator_sort_test() {
    let cmp = IndexedComparator::new(["a", "b", "c"]);
    assert!(cmp.compare(&"a", &"b") < 0);
    assert!(cmp.compare(&"c", &"a") > 0);
}

// ===== WindowsExplorerStringComparatorTest (2 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.comparator.WindowsExplorerStringComparatorTest.testCompare1()`
#[test]
fn windows_explorer_string_comparator_test_compare1() {
    let cmp = WindowsExplorerStringComparator::INSTANCE;
    assert!(cmp.compare("file1.txt", "file2.txt") < 0);
    assert!(cmp.compare("file10.txt", "file2.txt") > 0);
}
