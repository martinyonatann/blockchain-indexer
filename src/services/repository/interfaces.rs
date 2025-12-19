use crate::services::entities::evm_chains::EvmChains;
use crate::services::entities::evm_logs::EVMLogs;
use crate::services::entities::evm_sync_logs::EVMSyncLogs;
use async_trait::async_trait;

#[async_trait]
pub trait EVMChainRepository {
    async fn fetch_by_id(&self, id: u64) -> Result<EvmChains, sqlx::Error>;
    async fn update_last_synced_block_number(
        &self,
        id: u64,
        block_number: u64,
    ) -> Result<EvmChains, sqlx::Error>;
}

#[async_trait]
pub trait EVMLogsRepository {
    async fn create(&self, log: EVMLogs) -> Result<EVMLogs, sqlx::Error>;
    async fn list(&self, page_size: i64) -> Result<Vec<EVMLogs>, sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<(), sqlx::Error>;
    async fn count(&self) -> Result<Option<i64>, sqlx::Error>;
}

#[async_trait]
pub trait EVMSyncLogsRepository {
    async fn find_all(&self) -> Result<Vec<EVMSyncLogs>, sqlx::Error>;
    async fn find_by_address(&self, address: &str) -> Result<Option<EVMSyncLogs>, sqlx::Error>;
    async fn create(
        &self,
        address: &str,
        chain_id: u64,
        last_synced_block_number: Option<i64>,
    ) -> Result<EVMSyncLogs, sqlx::Error>;
    async fn find_or_create_by_address(
        &self,
        address: &str,
        chain_id: u64,
    ) -> Result<EVMSyncLogs, sqlx::error::Error>;
    async fn update_last_synced_block_number(
        &self,
        address: &str,
        block_number: u64,
    ) -> Result<EVMSyncLogs, sqlx::Error>;
}
