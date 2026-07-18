//! 对齐: `cn.hutool.core.util.RandomUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/RandomUtil.java

use rand::Rng;

/// 对齐 Java: `cn.hutool.core.util.RandomUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomUtil;

impl RandomUtil {
    /// 对齐 Java: `RandomUtil.randomInt()`
    pub fn random_int() -> i32 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomInt(int)`
    pub fn random_int_max(max: i32) -> i32 {
        rand::rng().random_range(0..max)
    }

    /// 对齐 Java: `RandomUtil.randomInt(int, int)`
    pub fn random_int_range(min: i32, max: i32) -> i32 {
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomLong()`
    pub fn random_long() -> i64 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomDouble()`
    pub fn random_double() -> f64 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomDouble(double, double)`
    pub fn random_double_range(min: f64, max: f64) -> f64 {
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomBoolean()`
    pub fn random_boolean() -> bool {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomString(int)`
    pub fn random_string(length: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::rng();
        (0..length)
            .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomString(String, int)`
    pub fn random_string_from(base: &str, length: usize) -> String {
        let chars: Vec<char> = base.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        (0..length)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomNumbers(int)`
    pub fn random_numbers(length: usize) -> String {
        let mut rng = rand::rng();
        (0..length)
            .map(|_| rng.random_range(0u8..10).to_string())
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomEle(List)`
    pub fn random_element<T: Clone>(items: &[T]) -> Option<T> {
        if items.is_empty() {
            return None;
        }
        let idx = rand::rng().random_range(0..items.len());
        Some(items[idx].clone())
    }

    /// 对齐 Java: `RandomUtil.randomEle(List, int)`
    pub fn random_elements<T: Clone>(items: &[T], count: usize) -> Vec<T> {
        if items.is_empty() || count == 0 {
            return Vec::new();
        }
        let mut rng = rand::rng();
        (0..count)
            .map(|_| items[rng.random_range(0..items.len())].clone())
            .collect()
    }

    /// 对齐 Java: `RandomUtil.weightRandom(Map)`
    pub fn weighted_random<T: Clone>(items: &[(T, u32)]) -> Option<T> {
        if items.is_empty() {
            return None;
        }
        let total_weight: u32 = items.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return None;
        }
        let mut rng = rand::rng();
        let mut target = rng.random_range(0..total_weight);
        for (item, weight) in items {
            if target < *weight {
                return Some(item.clone());
            }
            target -= weight;
        }
        items.last().map(|(item, _)| item.clone())
    }

    /// 对齐 Java: `RandomUtil.randomBytes(int)`
    pub fn random_bytes(length: usize) -> Vec<u8> {
        let mut rng = rand::rng();
        (0..length).map(|_| rng.random()).collect()
    }
}
