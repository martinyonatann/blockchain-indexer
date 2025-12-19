use crate::services::entities::evm_chains::EvmChains;
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
