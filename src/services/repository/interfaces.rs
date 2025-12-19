use crate::services::entities::evm_chains::EvmChains;
use crate::services::entities::evm_logs::EVMLogs;
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
