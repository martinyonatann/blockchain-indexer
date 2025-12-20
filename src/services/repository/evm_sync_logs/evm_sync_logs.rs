use async_trait::async_trait;
use sqlx::PgPool;

use crate::services::{entities::evm_sync_logs::EVMSyncLogs, repository::EVMSyncLogsRepository};

pub struct EVMSyncLogsRepositoryImpl {
    pool: PgPool,
}

impl EVMSyncLogsRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EVMSyncLogsRepository for EVMSyncLogsRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<EVMSyncLogs>, sqlx::Error> {
        sqlx::query_as::<_, EVMSyncLogs>(r#"SELECT * FROM evm_sync_logs"#)
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_address(&self, address: &str) -> Result<Option<EVMSyncLogs>, sqlx::Error> {
        let query = r#"SELECT * FROM evm_sync_logs WHERE address = $1::BYTEA"#;

        let result = sqlx::query_as::<_, EVMSyncLogs>(query)
            .bind(format!("\\x{address}"))
            .fetch_optional(&self.pool)
            .await;

        result
    }

    async fn create(
        &self,
        address: &str,
        chain_id: u64,
        last_synced_block_number: Option<i64>,
    ) -> Result<EVMSyncLogs, sqlx::Error> {
        let query = r#"
            INSERT INTO evm_sync_logs (address, chain_id, last_synced_block_number)
            VALUES ($1::BYTEA, $2, $3)
            RETURNING *
            "#;

        let result = sqlx::query_as::<_, EVMSyncLogs>(query)
            .bind(format!("\\x{address}"))
            .bind(chain_id as i64)
            .bind(last_synced_block_number.or(Some(0)))
            .fetch_one(&self.pool)
            .await;

        result
    }

    async fn find_or_create_by_address(
        &self,
        address: &str,
        chain_id: u64,
    ) -> Result<EVMSyncLogs, sqlx::error::Error> {
        let record = Self::find_by_address(self, address).await?;
        if let Some(log_record) = record {
            return Ok(log_record);
        }

        let new_record = Self::create(self, address, chain_id, None).await?;
        Ok(new_record)
    }

    async fn update_last_synced_block_number(
        &self,
        address: [u8; 20],
        block_number: u64,
    ) -> Result<EVMSyncLogs, sqlx::Error> {
        let query = r#"UPDATE evm_sync_logs SET last_synced_block_number = $1 WHERE address = $2 RETURNING *"#;

        let result = sqlx::query_as::<_, EVMSyncLogs>(query)
            .bind(block_number as i64)
            .bind(address)
            .fetch_one(&self.pool)
            .await;
        result
    }
}
