use crate::{
    infrastructure::blockchain::provider::{BlockchainProvider, create_log_filter},
    services::{
        repository::{EVMLogsRepository, EVMSyncLogsRepository},
        usecase::{IndexLogUC, errors::AppError},
    },
};
use async_trait::async_trait;

pub struct IndexLogUCImpl<P, LR, SR> {
    pub provider: P,
    pub log_repo: LR,
    pub sync_repo: SR,
    pub batch_size: u64,
}

impl<P, LR, SR> IndexLogUCImpl<P, LR, SR> {
    pub fn new(provider: P, log_repo: LR, sync_repo: SR, batch_size: u64) -> Self {
        Self {
            provider,
            log_repo,
            sync_repo,
            batch_size,
        }
    }
}

#[async_trait]
impl<P, LR, SR> IndexLogUC for IndexLogUCImpl<P, LR, SR>
where
    P: BlockchainProvider + Send + Sync,
    LR: EVMLogsRepository + Send + Sync,
    SR: EVMSyncLogsRepository + Send + Sync,
{
    async fn execute(&self, chain_id: u64, address: String) -> Result<(), AppError> {
        let sync_log = self
            .sync_repo
            .find_or_create_by_address(&address, chain_id)
            .await?;

        let latest_block = self.provider.get_block_number().await?;
        if latest_block == sync_log.last_synced_block_number as u64 {
            println!("Fully indexed address: {address}");
            return Ok(());
        }

        let from_block_number = match sync_log.last_synced_block_number as u64 {
            0 => latest_block,
            block_number => block_number + 1_u64,
        };

        let to_block_number = match sync_log.last_synced_block_number as u64 {
            0 => 0,
            block_number => std::cmp::min(block_number + 10_000_u64, latest_block),
        };

        let filter = create_log_filter(&address, from_block_number, to_block_number)?;

        let logs = self.provider.get_logs(&filter).await?;

        if !logs.is_empty() {
            self.log_repo.create_bulk(logs).await?;
        }

        let _ = self
            .sync_repo
            .update_last_synced_block_number(sync_log.address, to_block_number)
            .await
            .inspect_err(|error| eprintln!("Error updating last_synced_block_number {error}"));

        Ok(())
    }
}
