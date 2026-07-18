//! 对齐: `cn.hutool.core.lang.Console`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Console.java
//!
//! Hutool 的 `Console` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.Console`
#[derive(Debug, Clone, Default)]
pub struct Console;

impl Console {
    /// 对齐 Java: `Console.log()`
    #[allow(clippy::too_many_arguments)]
    pub fn log() -> Result<()> {
        Err(CoreError::PendingEngine("Console::log (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.table(ConsoleTable consoleTable)`
    #[allow(clippy::too_many_arguments)]
    pub fn table(ConsoleTable consoleTable) -> Result<()> {
        Err(CoreError::PendingEngine("Console::table (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.print(Object obj)`
    #[allow(clippy::too_many_arguments)]
    pub fn print(Object obj) -> Result<()> {
        Err(CoreError::PendingEngine("Console::print (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.printProgress(char showChar, int len)`
    #[allow(clippy::too_many_arguments)]
    pub fn printProgress(char showChar, i32 len) -> Result<()> {
        Err(CoreError::PendingEngine("Console::printProgress (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.error()`
    #[allow(clippy::too_many_arguments)]
    pub fn error() -> Result<()> {
        Err(CoreError::PendingEngine("Console::error (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.scanner()`
    #[allow(clippy::too_many_arguments)]
    pub fn scanner() -> Result<Scanner> {
        Err(CoreError::PendingEngine("Console::scanner (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.input()`
    #[allow(clippy::too_many_arguments)]
    pub fn input() -> Result<String> {
        Err(CoreError::PendingEngine("Console::input (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.where()`
    #[allow(clippy::too_many_arguments)]
    pub fn where() -> Result<String> {
        Err(CoreError::PendingEngine("Console::where (waiting for full impl)"))
    }
    /// 对齐 Java: `Console.lineNumber()`
    #[allow(clippy::too_many_arguments)]
    pub fn lineNumber() -> Result<i32> {
        Err(CoreError::PendingEngine("Console::lineNumber (waiting for full impl)"))
    }
}
