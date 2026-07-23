//! `cn.hutool.core.exceptions` 子包对比验证测试 (2 个测试文件, 7 个 @Test)
//! 来源: hutool-core/src/test/java/cn/hutool/core/exceptions/

use hitool_core::exceptions::{CheckedUtil, ExceptionUtil};

// ===== ExceptionUtilTest (4 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.exceptions.ExceptionUtilTest.getRootTest()`
#[test]
fn exception_util_get_root_test() {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "root");
    let root = ExceptionUtil::get_root_cause(Some(&err)).expect("root");
    assert!(root.to_string().contains("root"));
}

// ===== CheckedUtilTest (3 个 @Test) =====
/// 对齐 Java: `cn.hutool.core.exceptions.CheckedUtilTest.functionTest()`
#[test]
fn checked_util_function_test() {
    let f = CheckedUtil::uncheck0(|| 42_i32);
    assert_eq!(f.call(), 42);
}
