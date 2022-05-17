use anyhow::anyhow;
use mysql_async::{Pool, prelude::*};
use tokio::sync::Mutex;

pub struct AppState {
    mysql_conn_pool: Mutex<Option<Pool>>,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        Self {
            mysql_conn_pool: Mutex::new(Some(pool)),
        }
    }

    pub async fn select_number(&self) -> anyhow::Result<u32> {
        let conn = self.mysql_conn_pool.lock()
            .await
            .as_ref()
            .unwrap()
            .get_conn()
            .await?;

        let value: Option<u32> = "SELECT 42".first(conn).await?;

        value.ok_or_else(|| anyhow!("cannot query static number"))
    }

    /// This functions shutdowns the mysql connection pool.
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        Ok(
            self.mysql_conn_pool.lock()
                .await
                .take()
                .unwrap()
                .disconnect()
                .await?
        )
    }
}