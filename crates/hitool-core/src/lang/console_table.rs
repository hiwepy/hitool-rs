//! 对齐: `cn.hutool.core.lang.ConsoleTable`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsoleTable.java
//!
//! Hutool 的 `ConsoleTable` Java 类型,等待完整实现。
//! 状态: 对齐桩(对象/方法/参数已对齐),等待 `hitool-core` 内部继续迁移。

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.lang.ConsoleTable`
#[derive(Debug, Clone, Default)]
pub struct ConsoleTable;

impl ConsoleTable {
    /// 对齐 Java: `ConsoleTable.create()`
    #[allow(clippy::too_many_arguments)]
    pub fn create() -> Result<ConsoleTable> {
        Err(CoreError::PendingEngine("ConsoleTable::create (waiting for full impl)"))
    }
    /// 对齐 Java: `ConsoleTable.setSBCMode(boolean isSBCMode)`
    #[allow(clippy::too_many_arguments)]
    pub fn setSBCMode(bool isSBCMode) -> Result<ConsoleTable> {
        Err(CoreError::PendingEngine("ConsoleTable::setSBCMode (waiting for full impl)"))
    }
    /// 对齐 Java: `ConsoleTable.addHeader(String... titles)`
    #[allow(clippy::too_many_arguments)]
    pub fn addHeader(&str... titles) -> Result<ConsoleTable> {
        Err(CoreError::PendingEngine("ConsoleTable::addHeader (waiting for full impl)"))
    }
    /// 对齐 Java: `ConsoleTable.addBody(String... values)`
    #[allow(clippy::too_many_arguments)]
    pub fn addBody(&str... values) -> Result<ConsoleTable> {
        Err(CoreError::PendingEngine("ConsoleTable::addBody (waiting for full impl)"))
    }
    /// 对齐 Java: `ConsoleTable.print()`
    #[allow(clippy::too_many_arguments)]
    pub fn print() -> Result<()> {
        Err(CoreError::PendingEngine("ConsoleTable::print (waiting for full impl)"))
    }
}
