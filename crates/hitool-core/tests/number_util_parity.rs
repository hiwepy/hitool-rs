//! number_util parity tests
//! 对齐: hutool-core NumberUtilTest

use hitool_core::NumberUtil;

// ── 算术操作 ──

#[test]
fn add_basic() {
    assert_eq!(NumberUtil::add(1.5, 2.5), 4.0);
}

#[test]
fn sub_basic() {
    assert_eq!(NumberUtil::sub(5.0, 3.0), 2.0);
}

#[test]
fn mul_basic() {
    assert_eq!(NumberUtil::mul(3.0, 4.0), 12.0);
}

#[test]
fn div_basic() {
    assert_eq!(NumberUtil::div(10.0, 2.0).unwrap(), 5.0);
}

#[test]
fn div_by_zero() {
    assert!(NumberUtil::div(1.0, 0.0).is_err());
}

#[test]
fn div_with_scale() {
    assert_eq!(NumberUtil::div_with_scale(10.0, 3.0, 2).unwrap(), 3.33);
}

// ── 比较操作 ──

#[test]
fn compare_f64_basic() {
    assert_eq!(NumberUtil::compare_f64(1.0, 2.0), -1);
    assert_eq!(NumberUtil::compare_f64(2.0, 2.0), 0);
    assert_eq!(NumberUtil::compare_f64(3.0, 2.0), 1);
}

#[test]
fn compare_i32_basic() {
    assert_eq!(NumberUtil::compare_i32(1, 2), -1);
    assert_eq!(NumberUtil::compare_i32(2, 2), 0);
    assert_eq!(NumberUtil::compare_i32(3, 2), 1);
}

#[test]
fn compare_i64_basic() {
    assert_eq!(NumberUtil::compare_i64(100, 200), -1);
}

#[test]
fn compare_char_basic() {
    assert_eq!(NumberUtil::compare_char('a', 'b'), -1);
    assert_eq!(NumberUtil::compare_char('a', 'a'), 0);
    assert_eq!(NumberUtil::compare_char('b', 'a'), 1);
}

// ── 相等判断 ──

#[test]
fn equals_f64_basic() {
    assert!(NumberUtil::equals_f64(1.0, 1.0));
    assert!(!NumberUtil::equals_f64(1.0, 2.0));
}

#[test]
fn equals_f32_basic() {
    assert!(NumberUtil::equals_f32(1.0f32, 1.0f32));
    assert!(!NumberUtil::equals_f32(1.0f32, 2.0f32));
}

#[test]
fn equals_i64_basic() {
    assert!(NumberUtil::equals_i64(42, 42));
    assert!(!NumberUtil::equals_i64(42, 43));
}

// ── 最值操作 ──

#[test]
fn min_i32_basic() {
    assert_eq!(NumberUtil::min_i32(&[3, 1, 4, 1, 5]).unwrap(), 1);
}

#[test]
fn min_i64_basic() {
    assert_eq!(NumberUtil::min_i64(&[3, 1, 4, 1, 5]).unwrap(), 1);
}

#[test]
fn min_f64_basic() {
    assert_eq!(NumberUtil::min_f64(&[3.0, 1.0, 4.0]).unwrap(), 1.0);
}

#[test]
fn min_empty() {
    assert!(NumberUtil::min_i32(&[]).is_err());
}

#[test]
fn max_i32_basic() {
    assert_eq!(NumberUtil::max_i32(&[3, 1, 4, 1, 5]).unwrap(), 5);
}

#[test]
fn max_i64_basic() {
    assert_eq!(NumberUtil::max_i64(&[3, 1, 4, 1, 5]).unwrap(), 5);
}

#[test]
fn max_f64_basic() {
    assert_eq!(NumberUtil::max_f64(&[3.0, 1.0, 4.0]).unwrap(), 4.0);
}

#[test]
fn max_empty() {
    assert!(NumberUtil::max_i32(&[]).is_err());
}

// ── 数值判断 ──

#[test]
fn is_number_valid() {
    assert!(NumberUtil::is_number("123"));
    assert!(NumberUtil::is_number("3.14"));
    assert!(NumberUtil::is_number("-1"));
}

#[test]
fn is_number_invalid() {
    assert!(!NumberUtil::is_number("abc"));
    assert!(!NumberUtil::is_number(""));
}

#[test]
fn is_integer_valid() {
    assert!(NumberUtil::is_integer("123"));
    assert!(NumberUtil::is_integer("-1"));
}

#[test]
fn is_integer_invalid() {
    assert!(!NumberUtil::is_integer("3.14"));
    assert!(!NumberUtil::is_integer("abc"));
}

// ── 范围判断 ──

#[test]
fn is_in_range_i64_basic() {
    assert!(NumberUtil::is_in_range_i64(5, 1, 10));
    assert!(!NumberUtil::is_in_range_i64(0, 1, 10));
    assert!(NumberUtil::is_in_range_i64(1, 1, 10));
    assert!(NumberUtil::is_in_range_i64(10, 1, 10));
}

#[test]
fn is_in_range_f64_basic() {
    assert!(NumberUtil::is_in_range_f64(5.0, 1.0, 10.0));
    assert!(!NumberUtil::is_in_range_f64(0.0, 1.0, 10.0));
}

// ── 转换操作 ──

#[test]
fn parse_int_valid() {
    assert_eq!(NumberUtil::parse_int("42", 0), 42);
}

#[test]
fn parse_int_invalid() {
    assert_eq!(NumberUtil::parse_int("abc", 0), 0);
}

#[test]
fn parse_long_valid() {
    assert_eq!(NumberUtil::parse_long("1234567890", 0), 1234567890);
}

#[test]
fn parse_double_valid() {
    assert_eq!(NumberUtil::parse_double("3.14", 0.0), 3.14);
}

// ── 计数操作 ──

#[test]
fn count_basic() {
    assert_eq!(NumberUtil::count(1, 5, 1).unwrap(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn count_step() {
    assert_eq!(NumberUtil::count(0, 10, 2).unwrap(), vec![0, 2, 4, 6, 8, 10]);
}

#[test]
fn count_zero_step() {
    assert!(NumberUtil::count(1, 5, 0).is_err());
}

#[test]
fn range_basic() {
    assert_eq!(NumberUtil::range(0, 5), vec![0, 1, 2, 3, 4]);
}

// ── 数学操作 ──

#[test]
fn factorial_basic() {
    assert_eq!(NumberUtil::factorial(5).unwrap(), 120);
    assert_eq!(NumberUtil::factorial(0).unwrap(), 1);
}

#[test]
fn factorial_overflow() {
    assert!(NumberUtil::factorial(21).is_err());
}

#[test]
fn gcd_basic() {
    assert_eq!(NumberUtil::gcd(12, 8), 4);
    assert_eq!(NumberUtil::gcd(7, 5), 1);
}

#[test]
fn lcm_basic() {
    assert_eq!(NumberUtil::lcm(4, 6), 12);
    assert_eq!(NumberUtil::lcm(3, 5), 15);
}
