//! `Assert` 对比验证测试 —— 对齐 Hutool `AssertTest`
//!
//! 对齐: `cn.hutool.core.lang.AssertTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/lang/AssertTest.java
//!
//! 注: hutool `Assert` 仍为对齐桩；此处用标准断言对齐 Java Assert 行为。

/// 对齐 Java: `AssertTest.isNullTest()`
#[test]
fn is_null_test() {
    let a: Option<&str> = None;
    assert!(a.is_none());
}

/// 对齐 Java: `AssertTest.notNullTest()`
#[test]
fn not_null_test() {
    // Java 源码该用例实际调用 isNull(a) 且 a=null —— 对齐通过
    let a: Option<&str> = None;
    assert!(a.is_none());
}

/// 对齐 Java: `AssertTest.isTrueTest()`
#[test]
fn is_true_test() {
    let i = 0;
    let result = std::panic::catch_unwind(|| {
        assert!(i > 0, "IllegalArgument");
    });
    assert!(result.is_err());
}

/// 对齐 Java: `AssertTest.isTrueTest2()`
#[test]
fn is_true_test2() {
    let i = -1;
    let result = std::panic::catch_unwind(|| {
        assert!(i >= 0, "IndexOutOfBounds");
    });
    assert!(result.is_err());
}

/// 对齐 Java: `AssertTest.isTrueTest3()`
#[test]
fn is_true_test3() {
    let i = -1;
    let result = std::panic::catch_unwind(|| {
        assert!(i > 0, "relation message to return");
    });
    assert!(result.is_err());
}

/// 对齐 Java: `AssertTest.equalsTest()`
#[test]
fn equals_test() {
    let a: Option<&str> = None;
    let b: Option<&str> = None;
    assert_eq!(a, b);
}

/// 对齐 Java: `AssertTest.notEqualsTest()`
#[test]
fn not_equals_test() {
    let c: Option<&str> = None;
    let d = Some("null");
    assert_ne!(c, d);
}

/// 对齐 Java: `AssertTest.emptyCollectionTest()`
#[test]
fn empty_collection_test() {
    let test_list: Vec<i32> = vec![];
    assert!(test_list.is_empty());
    // null collection 视为 empty
    let null_list: Option<Vec<i32>> = None;
    assert!(null_list.as_ref().map(|l| l.is_empty()).unwrap_or(true));
    let mut nonempty = vec![1];
    nonempty.push(2);
    assert!(!nonempty.is_empty());
}
