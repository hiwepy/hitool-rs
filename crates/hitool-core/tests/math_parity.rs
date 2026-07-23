//! `cn.hutool.core.math` 子包对比验证测试
//! 来源: hutool-core/src/test/java/cn/hutool/core/math/

use hitool_core::math::{Arrangement, Calculator, Combination, MathUtil, Money};

/// 对齐 Java: `cn.hutool.core.math.CalculatorTest.conversationTest()`
#[test]
fn calculator_conversation_test() {
    assert!((Calculator::conversion("77 * 12") - 924.0).abs() < 0.001);
}

/// 对齐 Java: `cn.hutool.core.math.CombinationTest.countTest()`
#[test]
fn combination_count_test() {
    assert_eq!(Combination::count(5, 2), 10);
}

/// 对齐 Java: `cn.hutool.core.math.ArrangementTest.arrangementTest()`
#[test]
fn arrangement_test() {
    assert_eq!(Arrangement::count(4, 2), 12);
}

/// 对齐 Java: `cn.hutool.core.math.MoneyTest.yuanToCentTest()`
#[test]
fn money_yuan_to_cent_test() {
    assert_eq!(Money::from_yuan_str("1234.56").get_cent(), 123_456);
    assert_eq!(MathUtil::yuan_to_cent(1234.56), 123_456);
}
