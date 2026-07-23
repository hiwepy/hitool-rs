//! 事务 Session —— 对齐 Hutool `cn.hutool.db.Session`。

use crate::db::{Db, DbResult};
use crate::entity::Entity;

/// 对齐 Hutool `Session`（基于 Db 的简化事务封装）。
#[derive(Clone)]
pub struct Session {
    db: Db,
}

impl Session {
    /// 对齐 Java: `Session.create(String)`.
    #[must_use]
    pub fn create(db: Db) -> Self {
        Self { db }
    }

    /// 对齐 Java: `Session.update`.
    pub async fn update(&self, set: &Entity, where_entity: &Entity) -> DbResult<u64> {
        self.db.update(set, where_entity).await
    }

    /// 对齐 Java: `Session.tx`.
    pub async fn tx<F, Fut>(&self, func: F) -> DbResult<()>
    where
        F: FnOnce(Session) -> Fut,
        Fut: std::future::Future<Output = DbResult<()>>,
    {
        self.db
            .tx(|db| async move { func(Session::create(db)).await })
            .await
    }
}
