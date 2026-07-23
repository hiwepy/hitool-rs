//! 对齐: `cn.hutool.core.math.MathUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/MathUtil.java

use super::arrangement::Arrangement;
use super::combination::Combination;
use super::money::Money;

/// 数学工具 —— 对齐 Java `MathUtil`。
#[derive(Debug, Clone, Copy, Default)]
pub struct MathUtil;

impl MathUtil {
    /// 对齐 Java: `MathUtil.arrangementCount(int n, int m)`
    pub fn arrangement_count(n: i32, m: i32) -> i64 {
        Arrangement::count(n, m)
    }

    /// 对齐 Java: `MathUtil.arrangementCount(int n)`
    pub fn arrangement_count_n(n: i32) -> i64 {
        Arrangement::count_n(n)
    }

    /// 对齐 Java: `MathUtil.arrangementSelect(String[], int m)`
    pub fn arrangement_select(datas: &[String], m: i32) -> Vec<Vec<String>> {
        Arrangement::new(datas.iter().cloned()).select(m)
    }

    /// 对齐 Java: `MathUtil.arrangementSelect(String[])` —— 全排列。
    pub fn arrangement_select_all(datas: &[String]) -> Vec<Vec<String>> {
        Arrangement::new(datas.iter().cloned()).select_full()
    }

    /// 对齐 Java: `MathUtil.combinationCount(int n, int m)`
    pub fn combination_count(n: i32, m: i32) -> i64 {
        Combination::count(n, m)
    }

    /// 对齐 Java: `MathUtil.combinationSelect(String[], int m)`
    pub fn combination_select(datas: &[String], m: i32) -> Vec<Vec<String>> {
        Combination::new(datas.iter().cloned()).select(m)
    }

    /// 对齐 Java: `MathUtil.yuanToCent(double yuan)`
    pub fn yuan_to_cent(yuan: f64) -> i64 {
        Money::from_yuan_f64(yuan).get_cent()
    }

    /// 对齐 Java: `MathUtil.centToYuan(long cent)`
    pub fn cent_to_yuan(cent: i64) -> f64 {
        let yuan = cent / 100;
        let cent_part = (cent % 100) as i32;
        Money::from_yuan_cent(yuan, cent_part)
            .get_amount()
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
    }
}
