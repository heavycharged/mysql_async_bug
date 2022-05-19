use std::sync::Arc;
use anyhow::anyhow;
use mysql_async::{Pool, prelude::*};

pub struct AppState {
    mysql_conn_pool: Pool,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        Self {
            mysql_conn_pool: pool,
        }
    }

    pub async fn select_number(&self) -> anyhow::Result<u32> {
        let conn = self.mysql_conn_pool.get_conn().await?;

        let value: Option<u32> = "SELECT 42".first(conn).await?;

        value.ok_or_else(|| anyhow!("cannot query static number"))
    }

    /// This functions shutdowns the mysql connection pool.
    pub async fn shutdown(self: Arc<Self>) -> anyhow::Result<()> {
        if let Ok(v) = Arc::try_unwrap(self) {
            return Ok(v.mysql_conn_pool.disconnect().await?);
        }
        panic!("cannot unwrap arc value, someone still has access to the value")
    }
}