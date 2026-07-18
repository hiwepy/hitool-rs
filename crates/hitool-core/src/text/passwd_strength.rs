//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检查与等级评估。

use crate::{CoreError, Result};

/// 对齐 Java: `PasswdStrength#PASSWD_LEVEL` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswdLevel {
    /// 弱
    Easy,
    /// 中
    Midium,
    /// 强
    Strong,
    /// 非常强
    VeryStrong,
    /// 极强
    ExtremelyStrong,
}

/// 对齐 Java: `PasswdStrength#CHAR_TYPE` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    /// 数字
    Num,
    /// 小写字母
    SmallLetter,
    /// 大写字母
    CapitalLetter,
    /// 其他字符
    OtherChar,
}

/// 对齐 Java: `PasswdStrength#`
#[derive(Debug, Clone, Copy, Default)]
pub struct PasswdStrength;

impl PasswdStrength {
    /// 对齐 Java: `PasswdStrength::check#int (String passwd)`
    pub fn check(_passwd: &str) -> Result<i32> {
        Err(CoreError::PendingEngine("PasswdStrength::check"))
    }

    /// 对齐 Java: `PasswdStrength::getLevel#PASSWD_LEVEL (String passwd)`
    pub fn get_level(_passwd: &str) -> Result<PasswdLevel> {
        Err(CoreError::PendingEngine("PasswdStrength::get_level"))
    }
}