//! 对齐: `cn.hutool.core.text.TextSimilarity`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/TextSimilarity.java
//!
//! 文本相似度工具(最长公共子序列、相似率)。

use crate::Result;

/// 对齐 Java: `TextSimilarity#`
#[derive(Debug, Clone, Copy, Default)]
pub struct TextSimilarity;

impl TextSimilarity {
    /// 对齐 Java: `TextSimilarity::similar#double (String strA, String strB)`
    pub fn similar(a: &str, b: &str) -> Result<f64> {
        let (new_a, new_b) = if a.chars().count() < b.chars().count() {
            (remove_sign(b), remove_sign(a))
        } else {
            (remove_sign(a), remove_sign(b))
        };
        let temp = new_a.chars().count().max(new_b.chars().count());
        if temp == 0 {
            return Ok(1.0);
        }
        let common = longest_common_substring_length(&new_a, &new_b);
        Ok(common as f64 / temp as f64)
    }

    /// 对齐 Java: `TextSimilarity::similar#String (String strA, String strB, int scale)`
    pub fn similar_scaled(a: &str, b: &str, scale: i32) -> Result<String> {
        let degree = Self::similar(a, b)?;
        let pct = degree * 100.0;
        let factor = 10f64.powi(scale);
        let rounded = (pct * factor).round() / factor;
        Ok(format!("{:.*}%", scale as usize, rounded))
    }

    /// 对齐 Java: `TextSimilarity::longestCommonSubstring#String (String strA, String strB)`
    pub fn longest_common_substring(a: &str, b: &str) -> Result<String> {
        let chars_a: Vec<char> = a.chars().collect();
        let chars_b: Vec<char> = b.chars().collect();
        let matrix = generate_matrix(&chars_a, &chars_b);
        let mut m = chars_a.len();
        let mut n = chars_b.len();
        let mut result = vec!['\0'; matrix[m][n]];
        let mut current = result.len();
        while matrix[m][n] != 0 {
            if matrix[m][n] == matrix[m][n - 1] {
                n -= 1;
            } else if matrix[m][n] == matrix[m - 1][n] {
                m -= 1;
            } else {
                current -= 1;
                result[current] = chars_a[m - 1];
                n -= 1;
                m -= 1;
            }
        }
        Ok(result.into_iter().collect())
    }
}

fn remove_sign(str: &str) -> String {
    str.chars().filter(|c| is_valid_char(*c)).collect()
}

fn is_valid_char(c: char) -> bool {
    let u = c as u32;
    (0x4E00..=0x9FFF).contains(&u)
        || c.is_ascii_lowercase()
        || c.is_ascii_uppercase()
        || c.is_ascii_digit()
}

fn longest_common_substring_length(a: &str, b: &str) -> usize {
    let chars_a: Vec<char> = a.chars().collect();
    let chars_b: Vec<char> = b.chars().collect();
    let matrix = generate_matrix(&chars_a, &chars_b);
    matrix[chars_a.len()][chars_b.len()]
}

fn generate_matrix(a: &[char], b: &[char]) -> Vec<Vec<usize>> {
    let m = a.len();
    let n = b.len();
    let mut matrix = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
            } else {
                matrix[i][j] = matrix[i][j - 1].max(matrix[i - 1][j]);
            }
        }
    }
    matrix
}
