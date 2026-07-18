//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本提供基本算术、比较、最值操作的 idiomatic 实现。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.NumberUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberUtil;

impl NumberUtil {
    // ── 算术操作 ──

    /// 对齐 Java: `NumberUtil.add(double, double)`
    pub fn add(v1: f64, v2: f64) -> f64 {
        v1 + v2
    }

    /// 对齐 Java: `NumberUtil.sub(double, double)`
    pub fn sub(v1: f64, v2: f64) -> f64 {
        v1 - v2
    }

    /// 对齐 Java: `NumberUtil.mul(double, double)`
    pub fn mul(v1: f64, v2: f64) -> f64 {
        v1 * v2
    }

    /// 对齐 Java: `NumberUtil.div(double, double)`
    pub fn div(v1: f64, v2: f64) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 / v2)
    }

    /// 对齐 Java: `NumberUtil.div(double, double, int)`
    pub fn div_with_scale(v1: f64, v2: f64, scale: u32) -> Result<f64> {
        let result = Self::div(v1, v2)?;
        let factor = 10f64.powi(scale as i32);
        Ok((result * factor).round() / factor)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `NumberUtil.compare(double, double)`
    pub fn compare_f64(x: f64, y: f64) -> i32 {
        x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal) as i32
    }

    /// 对齐 Java: `NumberUtil.compare(int, int)`
    pub fn compare_i32(x: i32, y: i32) -> i32 {
        x.cmp(&y) as i32
    }

    /// 对齐 Java: `NumberUtil.compare(long, long)`
    pub fn compare_i64(x: i64, y: i64) -> i32 {
        x.cmp(&y) as i32
    }

    /// 对齐 Java: `NumberUtil.compare(char, char)`
    pub fn compare_char(x: char, y: char) -> i32 {
        x.cmp(&y) as i32
    }

    // ── 相等判断 ──

    /// 对齐 Java: `NumberUtil.equals(double, double)`
    pub fn equals_f64(num1: f64, num2: f64) -> bool {
        (num1 - num2).abs() < f64::EPSILON
    }

    /// 对齐 Java: `NumberUtil.equals(float, float)`
    pub fn equals_f32(num1: f32, num2: f32) -> bool {
        (num1 - num2).abs() < f32::EPSILON
    }

    /// 对齐 Java: `NumberUtil.equals(long, long)`
    pub fn equals_i64(num1: i64, num2: i64) -> bool {
        num1 == num2
    }

    // ── 最值操作 ──

    /// 对齐 Java: `NumberUtil.min(int[])`
    pub fn min_i32(values: &[i32]) -> Result<i32> {
        values
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.min(long[])`
    pub fn min_i64(values: &[i64]) -> Result<i64> {
        values
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.min(double[])`
    pub fn min_f64(values: &[f64]) -> Result<f64> {
        values
            .iter()
            .copied()
            .reduce(f64::min)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(int[])`
    pub fn max_i32(values: &[i32]) -> Result<i32> {
        values
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(long[])`
    pub fn max_i64(values: &[i64]) -> Result<i64> {
        values
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(double[])`
    pub fn max_f64(values: &[f64]) -> Result<f64> {
        values
            .iter()
            .copied()
            .reduce(f64::max)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    // ── 数值判断 ──

    /// 对齐 Java: `NumberUtil.isNumber(CharSequence)`
    pub fn is_number(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.parse::<f64>().is_ok()
    }

    /// 对齐 Java: `NumberUtil.isInteger(CharSequence)`
    pub fn is_integer(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.parse::<i64>().is_ok()
    }

    // ── 范围判断 ──

    /// 对齐 Java: `NumberUtil.isInRange(long, long, long)`
    pub fn is_in_range_i64(value: i64, min: i64, max: i64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `NumberUtil.isInRange(double, double, double)`
    pub fn is_in_range_f64(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    // ── 转换操作 ──

    /// 对齐 Java: `NumberUtil.parseInt(CharSequence, int)`
    pub fn parse_int(s: &str, default_value: i32) -> i32 {
        s.parse::<i32>().unwrap_or(default_value)
    }

    /// 对齐 Java: `NumberUtil.parseLong(CharSequence, long)`
    pub fn parse_long(s: &str, default_value: i64) -> i64 {
        s.parse::<i64>().unwrap_or(default_value)
    }

    /// 对齐 Java: `NumberUtil.parseDouble(CharSequence, double)`
    pub fn parse_double(s: &str, default_value: f64) -> f64 {
        s.parse::<f64>().unwrap_or(default_value)
    }

    // ── 计数操作 ──

    /// 对齐 Java: `NumberUtil.count(int, int)`
    pub fn count(start: i32, end: i32, step: i32) -> Result<Vec<i32>> {
        if step == 0 {
            return Err(CoreError::InvalidArgument {
                name: "step",
                reason: "step cannot be zero",
            });
        }
        let mut result = Vec::new();
        if step > 0 {
            let mut current = start;
            while current <= end {
                result.push(current);
                current += step;
            }
        } else {
            let mut current = start;
            while current >= end {
                result.push(current);
                current += step;
            }
        }
        Ok(result)
    }

    /// 对齐 Java: `NumberUtil.appendRange(int, int)`
    pub fn range(start: i32, end: i32) -> Vec<i32> {
        (start..end).collect()
    }

    /// 对齐 Java: `NumberUtil.factorial(long)`
    pub fn factorial(n: u64) -> Result<u64> {
        if n > 20 {
            return Err(CoreError::InvalidArgument {
                name: "n",
                reason: "factorial overflow for n > 20",
            });
        }
        Ok((1..=n).product())
    }

    /// 对齐 Java: `NumberUtil.gcd(long, long)`
    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// 对齐 Java: `NumberUtil.lcm(long, long)`
    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a.abs() / Self::gcd(a, b)) * b.abs()
    }
}
