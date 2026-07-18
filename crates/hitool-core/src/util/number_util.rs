//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数桩;具体实现
//! 等待各 `hitool-rs` 引擎完成后回填,所有桩在调用时统一返回
//! `CoreError::PendingEngine`。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(dead_code, unused_variables, clippy::too_many_arguments, non_snake_case)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.NumberUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberUtil;

impl NumberUtil {
    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (float v1, float v2)`
    pub fn add(v1: f32, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (float v1, double v2)`
    pub fn add_2(v1: f32, v2: f64) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, float v2)`
    pub fn add_3(v1: f64, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, double v2)`
    pub fn add_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 + v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (long v1, double v2)`
    pub fn add_5(v1: i64, v2: f64) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, long v2)`
    pub fn add_6(v1: f64, v2: i64) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (long v1, long v2)`
    pub fn add_7(v1: i64, v2: i64) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (Double v1, Double v2)`
    pub fn add_8(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (Number v1, Number v2)`
    pub fn add_9(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (Number... values)`
    pub fn add_10(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (String... values)`
    pub fn add_11(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (BigDecimal... values)`
    pub fn add_12(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (float v1, float v2)`
    pub fn sub(v1: f32, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (float v1, double v2)`
    pub fn sub_2(v1: f32, v2: f64) -> Result<f64> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (double v1, float v2)`
    pub fn sub_3(v1: f64, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (double v1, double v2)`
    pub fn sub_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 - v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (Double v1, Double v2)`
    pub fn sub_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (Number v1, Number v2)`
    pub fn sub_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (Number... values)`
    pub fn sub_7(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (String... values)`
    pub fn sub_8(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (BigDecimal... values)`
    pub fn sub_9(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (float v1, float v2)`
    pub fn mul(v1: f32, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (float v1, double v2)`
    pub fn mul_2(v1: f32, v2: f64) -> Result<f64> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (double v1, float v2)`
    pub fn mul_3(v1: f64, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (double v1, double v2)`
    pub fn mul_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 * v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (Double v1, Double v2)`
    pub fn mul_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (Number v1, Number v2)`
    pub fn mul_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (Number... values)`
    pub fn mul_7(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (String v1, String v2)`
    pub fn mul_8(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (String... values)`
    pub fn mul_9(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (BigDecimal... values)`
    pub fn mul_10(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2)`
    pub fn div(v1: f32, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2)`
    pub fn div_2(v1: f32, v2: f64) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2)`
    pub fn div_3(v1: f64, v2: f32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2)`
    pub fn div_4(v1: f64, v2: f64) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument { name: "v2", reason: "division by zero" });
        }
        Ok(v1 / v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2)`
    pub fn div_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2)`
    pub fn div_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2)`
    pub fn div_7(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2, int scale)`
    pub fn div_8(v1: f32, v2: f32, scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2, int scale)`
    pub fn div_9(v1: f32, v2: f64, scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2, int scale)`
    pub fn div_10(v1: f64, v2: f32, scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2, int scale)`
    pub fn div_11(v1: f64, v2: f64, scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2, int scale)`
    pub fn div_12(_v1: *const (), _v2: *const (), scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2, int scale)`
    pub fn div_13(_v1: *const (), _v2: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2, int scale)`
    pub fn div_14(_v1: *const (), _v2: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2, int scale, RoundingMode roundingMode)`
    pub fn div_15(v1: f32, v2: f32, scale: i32, _roundingMode: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2, int scale, RoundingMode roundingMode)`
    pub fn div_16(v1: f32, v2: f64, scale: i32, _roundingMode: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2, int scale, RoundingMode roundingMode)`
    pub fn div_17(v1: f64, v2: f32, scale: i32, _roundingMode: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2, int scale, RoundingMode roundingMode)`
    pub fn div_18(v1: f64, v2: f64, scale: i32, _roundingMode: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2, int scale, RoundingMode roundingMode)`
    pub fn div_19(_v1: *const (), _v2: *const (), scale: i32, _roundingMode: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2, int scale, RoundingMode roundingMode)`
    pub fn div_20(_v1: *const (), _v2: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2, int scale, RoundingMode roundingMode)`
    pub fn div_21(_v1: *const (), _v2: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (BigDecimal v1, BigDecimal v2, int scale, RoundingMode roundingMode)`
    pub fn div_22(_v1: *const (), _v2: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::ceilDiv#int (int v1, int v2)`
    pub fn ceilDiv(v1: i32, v2: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("ceilDiv"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (double v, int scale)`
    pub fn round(v: f64, scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (double v, int scale)`
    pub fn roundStr(v: f64, scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (String numberStr, int scale)`
    pub fn round_2(_numberStr: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (BigDecimal number, int scale)`
    pub fn round_3(_number: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (String numberStr, int scale)`
    pub fn roundStr_2(_numberStr: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (double v, int scale, RoundingMode roundingMode)`
    pub fn round_4(v: f64, scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (double v, int scale, RoundingMode roundingMode)`
    pub fn roundStr_3(v: f64, scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (String numberStr, int scale, RoundingMode roundingMode)`
    pub fn round_5(_numberStr: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (BigDecimal number, int scale, RoundingMode roundingMode)`
    pub fn round_6(_number: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("round"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (String numberStr, int scale, RoundingMode roundingMode)`
    pub fn roundStr_4(_numberStr: *const (), scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundHalfEven#BigDecimal (Number number, int scale)`
    pub fn roundHalfEven(_number: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundHalfEven"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundHalfEven#BigDecimal (BigDecimal value, int scale)`
    pub fn roundHalfEven_2(_value: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundHalfEven"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundDown#BigDecimal (Number number, int scale)`
    pub fn roundDown(_number: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundDown"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundDown#BigDecimal (BigDecimal value, int scale)`
    pub fn roundDown_2(_value: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundDown"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, double value)`
    pub fn decimalFormat(_pattern: *const (), value: f64) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, long value)`
    pub fn decimalFormat_2(_pattern: *const (), value: i64) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, Object value)`
    pub fn decimalFormat_3(_pattern: *const (), _value: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, Object value, RoundingMode roundingMode)`
    pub fn decimalFormat_4(_pattern: *const (), _value: *const (), _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormatMoney#String (double value)`
    pub fn decimalFormatMoney(value: f64) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormatMoney"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::formatPercent#String (double number, int scale)`
    pub fn formatPercent(number: f64, scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("formatPercent"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isNumber#boolean (CharSequence str)`
    pub fn isNumber(_str: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isInteger#boolean (String s)`
    pub fn isInteger(_s: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isInteger"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLong#boolean (String s)`
    pub fn isLong(_s: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isLong"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isDouble#boolean (String s)`
    pub fn isDouble(_s: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isDouble"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isPrimes#boolean (int n)`
    pub fn isPrimes(n: i32) -> Result<bool> {
        Err(CoreError::PendingEngine("isPrimes"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateRandomNumber#int[] (int begin, int end, int size)`
    pub fn generateRandomNumber(begin: i32, end: i32, size: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("generateRandomNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateRandomNumber#int[] (int begin, int end, int size, int[] seed)`
    pub fn generateRandomNumber_2(begin: i32, end: i32, size: i32, seed: Vec<i32>) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("generateRandomNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateBySet#Integer[] (int begin, int end, int size)`
    pub fn generateBySet(begin: i32, end: i32, size: i32) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("generateBySet"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int stop)`
    pub fn range(stop: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int start, int stop)`
    pub fn range_2(start: i32, stop: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int start, int stop, int step)`
    pub fn range_3(start: i32, stop: i32, step: i32) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("range"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::appendRange#Collection<Integer> (int start, int stop, Collection<Integer> values)`
    pub fn appendRange(start: i32, stop: i32, values: Vec<OPAQUE>) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("appendRange"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::appendRange#Collection<Integer> (int start, int stop, int step, Collection<Integer> values)`
    pub fn appendRange_2(start: i32, stop: i32, step: i32, values: Vec<OPAQUE>) -> Result<Vec<OPAQUE>> {
        Err(CoreError::PendingEngine("appendRange"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#BigInteger (BigInteger n)`
    pub fn factorial(_n: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#BigInteger (BigInteger start, BigInteger end)`
    pub fn factorial_2(_start: *const (), _end: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#long (long start, long end)`
    pub fn factorial_3(start: i64, end: i64) -> Result<i64> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#long (long n)`
    pub fn factorial_4(n: i64) -> Result<i64> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sqrt#long (long x)`
    pub fn sqrt(x: i64) -> Result<i64> {
        Err(CoreError::PendingEngine("sqrt"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::processMultiple#int (int selectNum, int minNum)`
    pub fn processMultiple(selectNum: i32, minNum: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("processMultiple"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::divisor#int (int m, int n)`
    pub fn divisor(m: i32, n: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("divisor"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::multiple#int (int m, int n)`
    pub fn multiple(m: i32, n: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("multiple"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::getBinaryStr#String (Number number)`
    pub fn getBinaryStr(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("getBinaryStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::binaryToInt#int (String binaryStr)`
    pub fn binaryToInt(_binaryStr: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("binaryToInt"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::binaryToLong#long (String binaryStr)`
    pub fn binaryToLong(_binaryStr: *const ()) -> Result<i64> {
        Err(CoreError::PendingEngine("binaryToLong"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (char x, char y)`
    pub fn compare(x: char, y: char) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (double x, double y)`
    pub fn compare_2(x: f64, y: f64) -> Result<i32> {
        Ok(x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (int x, int y)`
    pub fn compare_3(x: i32, y: i32) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (long x, long y)`
    pub fn compare_4(x: i64, y: i64) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (short x, short y)`
    pub fn compare_5(x: i16, y: i16) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (byte x, byte y)`
    pub fn compare_6(x: i8, y: i8) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isGreater#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isGreater(_bigNum1: *const (), _bigNum2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isGreater"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isGreaterOrEqual#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isGreaterOrEqual(_bigNum1: *const (), _bigNum2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isGreaterOrEqual"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLess#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isLess(_bigNum1: *const (), _bigNum2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isLess"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLessOrEqual#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isLessOrEqual(_bigNum1: *const (), _bigNum2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isLessOrEqual"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isIn#boolean (final BigDecimal value, final BigDecimal minInclude, final BigDecimal maxInclude)`
    pub fn isIn(_value: *const (), _minInclude: *const (), _maxInclude: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isIn"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (double num1, double num2)`
    pub fn equals(num1: f64, num2: f64) -> Result<bool> {
        Ok((num1 - num2).abs() < f64::EPSILON)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (float num1, float num2)`
    pub fn equals_2(num1: f32, num2: f32) -> Result<bool> {
        Ok((num1 - num2).abs() < f32::EPSILON)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (long num1, long num2)`
    pub fn equals_3(num1: i64, num2: i64) -> Result<bool> {
        Ok(num1 == num2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (final Number number1, final Number number2)`
    pub fn equals_4(_number1: *const (), _number2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("equals"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn equals_5(_bigNum1: *const (), _bigNum2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("equals"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (char c1, char c2, boolean ignoreCase)`
    pub fn equals_6(c1: char, c2: char, ignoreCase: bool) -> Result<bool> {
        Err(CoreError::PendingEngine("equals"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#T (T[] numberArray)`
    pub fn min(numberArray: Vec<T>) -> Result<T> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#long (long... numberArray)`
    pub fn min_2(numberArray: &[i64]) -> Result<i64> {
        numberArray.iter().copied().min().ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#int (int... numberArray)`
    pub fn min_3(numberArray: &[i32]) -> Result<i32> {
        numberArray.iter().copied().min().ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#short (short... numberArray)`
    pub fn min_4(numberArray: &[i16]) -> Result<i16> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#double (double... numberArray)`
    pub fn min_5(numberArray: &[f64]) -> Result<f64> {
        numberArray.iter().copied().reduce(f64::min).ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#float (float... numberArray)`
    pub fn min_6(numberArray: &[f32]) -> Result<f32> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#BigDecimal (BigDecimal... numberArray)`
    pub fn min_7(numberArray: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("min"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#T (T[] numberArray)`
    pub fn max(numberArray: Vec<T>) -> Result<T> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#long (long... numberArray)`
    pub fn max_2(numberArray: &[i64]) -> Result<i64> {
        numberArray.iter().copied().max().ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#int (int... numberArray)`
    pub fn max_3(numberArray: &[i32]) -> Result<i32> {
        numberArray.iter().copied().max().ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#short (short... numberArray)`
    pub fn max_4(numberArray: &[i16]) -> Result<i16> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#double (double... numberArray)`
    pub fn max_5(numberArray: &[f64]) -> Result<f64> {
        numberArray.iter().copied().reduce(f64::max).ok_or_else(|| CoreError::InvalidArgument { name: "numberArray", reason: "empty array" })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#float (float... numberArray)`
    pub fn max_6(numberArray: &[f32]) -> Result<f32> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#BigDecimal (BigDecimal... numberArray)`
    pub fn max_7(numberArray: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("max"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number, String defaultValue)`
    pub fn toStr(_number: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number)`
    pub fn toStr_2(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number, boolean isStripTrailingZeros)`
    pub fn toStr_3(_number: *const (), isStripTrailingZeros: bool) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (BigDecimal bigDecimal)`
    pub fn toStr_4(_bigDecimal: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (BigDecimal bigDecimal, boolean isStripTrailingZeros)`
    pub fn toStr_5(_bigDecimal: *const (), isStripTrailingZeros: bool) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigDecimal#BigDecimal (Number number)`
    pub fn toBigDecimal(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigDecimal"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigDecimal#BigDecimal (String numberStr)`
    pub fn toBigDecimal_2(_numberStr: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigDecimal"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigInteger#BigInteger (Number number)`
    pub fn toBigInteger(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigInteger"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigInteger#BigInteger (String number)`
    pub fn toBigInteger_2(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigInteger"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::count#int (int total, int part)`
    pub fn count(total: i32, part: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("count"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::null2Zero#BigDecimal (BigDecimal decimal)`
    pub fn null2Zero(_decimal: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("null2Zero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::zero2One#int (int value)`
    pub fn zero2One(value: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("zero2One"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#int (Integer number)`
    pub fn nullToZero(_number: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#long (Long number)`
    pub fn nullToZero_2(_number: *const ()) -> Result<i64> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#double (Double number)`
    pub fn nullToZero_3(_number: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#float (Float number)`
    pub fn nullToZero_4(_number: *const ()) -> Result<f32> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#short (Short number)`
    pub fn nullToZero_5(_number: *const ()) -> Result<i16> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#byte (Byte number)`
    pub fn nullToZero_6(_number: *const ()) -> Result<i8> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#BigDecimal (BigDecimal number)`
    pub fn nullToZero_7(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#BigInteger (BigInteger number)`
    pub fn nullToZero_8(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::newBigInteger#BigInteger (String str)`
    pub fn newBigInteger(_str: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("newBigInteger"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isBeside#boolean (long number1, long number2)`
    pub fn isBeside(number1: i64, number2: i64) -> Result<bool> {
        Err(CoreError::PendingEngine("isBeside"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isBeside#boolean (int number1, int number2)`
    pub fn isBeside_2(number1: i32, number2: i32) -> Result<bool> {
        Err(CoreError::PendingEngine("isBeside"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::partValue#int (int total, int partCount)`
    pub fn partValue(total: i32, partCount: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("partValue"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::partValue#int (int total, int partCount, boolean isPlusOneWhenHasRem)`
    pub fn partValue_2(total: i32, partCount: i32, isPlusOneWhenHasRem: bool) -> Result<i32> {
        Err(CoreError::PendingEngine("partValue"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (Number number, int n)`
    pub fn pow(_number: *const (), n: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("pow"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (BigDecimal number, int n)`
    pub fn pow_2(_number: *const (), n: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("pow"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (BigDecimal number, int n, int scale, RoundingMode roundingMode)`
    pub fn pow_3(_number: *const (), n: i32, scale: i32, _roundingMode: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("pow"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isPowerOfTwo#boolean (long n)`
    pub fn isPowerOfTwo(n: i64) -> Result<bool> {
        Err(CoreError::PendingEngine("isPowerOfTwo"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseInt#int (String number)`
    pub fn parseInt(_number: *const ()) -> Result<i32> {
        Err(CoreError::PendingEngine("parseInt"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseLong#long (String number)`
    pub fn parseLong(_number: *const ()) -> Result<i64> {
        Err(CoreError::PendingEngine("parseLong"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseFloat#float (String number)`
    pub fn parseFloat(_number: *const ()) -> Result<f32> {
        Err(CoreError::PendingEngine("parseFloat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseDouble#double (String number)`
    pub fn parseDouble(_number: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("parseDouble"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseNumber#Number (String numberStr)`
    pub fn parseNumber(_numberStr: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseInt#Integer (String numberStr, Integer defaultValue)`
    pub fn parseInt_2(_numberStr: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseInt"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseLong#Long (String numberStr, Long defaultValue)`
    pub fn parseLong_2(_numberStr: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseLong"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseFloat#Float (String numberStr, Float defaultValue)`
    pub fn parseFloat_2(_numberStr: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseFloat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseDouble#Double (String numberStr, Double defaultValue)`
    pub fn parseDouble_2(_numberStr: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseDouble"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseNumber#Number (String numberStr, Number defaultValue)`
    pub fn parseNumber_2(_numberStr: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("parseNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBytes#byte[] (int value)`
    pub fn toBytes(value: i32) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("toBytes"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toInt#int (byte[] bytes)`
    pub fn toInt(bytes: Vec<i8>) -> Result<i32> {
        Err(CoreError::PendingEngine("toInt"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toUnsignedByteArray#byte[] (BigInteger value)`
    pub fn toUnsignedByteArray(_value: *const ()) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("toUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toUnsignedByteArray#byte[] (int length, BigInteger value)`
    pub fn toUnsignedByteArray_2(length: i32, _value: *const ()) -> Result<Vec<i8>> {
        Err(CoreError::PendingEngine("toUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::fromUnsignedByteArray#BigInteger (byte[] buf)`
    pub fn fromUnsignedByteArray(buf: Vec<i8>) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("fromUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::fromUnsignedByteArray#BigInteger (byte[] buf, int off, int length)`
    pub fn fromUnsignedByteArray_2(buf: Vec<i8>, off: i32, length: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("fromUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValidNumber#boolean (Number number)`
    pub fn isValidNumber(_number: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isValidNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValid#boolean (double number)`
    pub fn isValid(number: f64) -> Result<bool> {
        Err(CoreError::PendingEngine("isValid"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValid#boolean (float number)`
    pub fn isValid_2(number: f32) -> Result<bool> {
        Err(CoreError::PendingEngine("isValid"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::calculate#double (String expression)`
    pub fn calculate(_expression: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("calculate"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toDouble#double (Number value)`
    pub fn toDouble(_value: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("toDouble"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isOdd#boolean (int num)`
    pub fn isOdd(num: i32) -> Result<bool> {
        Err(CoreError::PendingEngine("isOdd"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isEven#boolean (int num)`
    pub fn isEven(num: i32) -> Result<bool> {
        Err(CoreError::PendingEngine("isEven"))
    }
}
