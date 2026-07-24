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

mod thread_local_connection;
mod grouped_connection;

pub use thread_local_connection::ThreadLocalConnection;
pub use grouped_connection::GroupedConnection;
