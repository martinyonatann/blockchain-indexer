use alloy::rpc::types::Log;
use async_trait::async_trait;
use sqlx::{PgPool, types::BigDecimal};

use crate::services::{entities::evm_logs::EVMLogs, repository::EVMLogsRepository};

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
    async fn create_bulk(&self, logs: Vec<Log>) -> Result<(), sqlx::Error> {
        for log in logs {
            Self::create(self, log).await?;
        }

        Ok(())
    }

    async fn create(&self, log: Log) -> Result<EVMLogs, sqlx::Error> {
        let block_hash = log
            .block_hash
            .ok_or_else(|| sqlx::Error::Decode("Missing block hash".into()))?
            .to_vec();

        let block_number: BigDecimal = log
            .block_number
            .ok_or_else(|| sqlx::Error::Decode("Missing block number".into()))?
            .into();

        let transaction_index: i64 = log
            .transaction_index
            .ok_or_else(|| sqlx::Error::Decode("Missing transaction index".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Transaction index exceeds i64 range".into()))?;

        let log_index: i64 = log
            .log_index
            .ok_or_else(|| sqlx::Error::Decode("Missing log index".into()))?
            .try_into()
            .map_err(|_| sqlx::Error::Decode("Log index exceeds i64 range".into()))?;

        let transaction_hash = log
            .transaction_hash
            .ok_or_else(|| sqlx::Error::Decode("Missing transaction hash".into()))?
            .to_vec();

        let address = log.address().to_vec();
        let event_signature: &[u8] = log.topics()[0].as_slice();
        let topics: Vec<&[u8]> = log.topics().iter().map(|topic| topic.as_slice()).collect();
        let log_data: Vec<u8> = log.inner.data.data.to_vec();

        let query = r#"
                INSERT INTO evm_logs (
                    block_hash, block_number, address, transaction_hash, 
                    transaction_index, event_signature, topics, data, 
                    log_index, removed
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING *
            "#;

        sqlx::query_as::<_, EVMLogs>(query)
            .bind(block_hash)
            .bind(block_number)
            .bind(address)
            .bind(transaction_hash)
            .bind(transaction_index)
            .bind(event_signature)
            .bind(topics)
            .bind(log_data)
            .bind(log_index)
            .bind(log.removed)
            .fetch_one(&self.pool)
            .await
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
