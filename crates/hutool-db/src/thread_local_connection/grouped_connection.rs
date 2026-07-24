//! ThreadLocalConnection facade，对齐 hutool 的 `cn.hutool.db.ThreadLocalConnection`。
//!
//! 基于 `thread_local!` 提供 thread-local Connection 管理。
//! 具体 Connection 类型（Java JDBC Connection）是 unsafe-to-copy，Rust 用 trait object 替代。
//!
//! **注意**：由于 Rust 借用规则限制，`get()` 返回 `bool`/`has()` 而非 `Box<dyn Any>`。
//! 用户应使用 `with_connection(ds_name, |conn| { ... })` 闭包模式访问。

use std::any::Any;
use std::collections::HashMap;

use crate::DbResult;
use crate::db::DbRuntimeError;

use super::thread_local_connection::ThreadLocalConnection;

/// 分组连接集合，对齐 `cn.hutool.db.ThreadLocalConnection.GroupedConnection`。
///
/// 每个 GroupedConnection 关联一组 DataSource → Connection 映射。
pub struct GroupedConnection {
    pub(crate) connections: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl GroupedConnection {
    /// 对齐 `GroupedConnection()`
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /// 对齐 `GroupedConnection.get(DataSource ds)` — 返回是否存在
    pub fn has(&self, ds_name: &str) -> bool {
        self.connections.contains_key(ds_name)
    }

    /// 对齐 `GroupedConnection.close(DataSource ds)`
    pub fn close(&mut self, ds_name: &str) -> DbResult<()> {
        self.connections.remove(ds_name);
        Ok(())
    }

    /// 对齐 `GroupedConnection.isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }
}

impl Default for GroupedConnection {
    fn default() -> Self {
        Self::new()
    }
}
