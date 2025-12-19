use async_trait::async_trait;
use sqlx::PgPool;

use crate::services::{entities::evm_logs::EVMLogs, repository::interfaces::EVMLogsRepository};

pub struct EVMLogsRepositoryImpl {
    pool: PgPool,
}

impl EVMLogsRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EVMLogsRepository for EVMLogsRepositoryImpl {
    async fn create(&self, _unused: EVMLogs) -> Result<EVMLogs, sqlx::Error> {
        todo!()
    }

    async fn list(&self, page_size: i64) -> Result<Vec<EVMLogs>, sqlx::Error> {
        sqlx::query_as::<_, EVMLogs>("SELECT * FROM evm_logs LIMIT $1")
            .bind(page_size)
            .fetch_all(&self.pool)
            .await
    }

    async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query(r#"DELETE FROM evm_logs WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn count(&self) -> Result<Option<i64>, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM evm_logs"#)
            .fetch_one(&self.pool)
            .await?;

        if count == 0 {
            return Ok(None);
        }

        Ok(Some(count))
    }
}
