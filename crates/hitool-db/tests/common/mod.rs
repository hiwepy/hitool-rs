//! 集成测试共享 fixture —— SQLite 内存库 + Hutool `user` 表 seed。

use hitool_db::{memory_pool, Db};

/// 返回已 seed `user` 表的 `Db`（`sqlite::memory:`）。
pub async fn test_db() -> Db {
    Db::new(memory_pool().await.expect("memory pool"))
}
