use async_trait::async_trait;
use futures::stream::{self, StreamExt};

use crate::services::{
    dtos::index_engine::{FeeAmountEnabledRequest, OwnerChangedRequest, PoolCreatedRequest},
    entities::evm_logs::EVMLogs,
    repository::EVMLogsRepository,
    usecase::{IndexEngineUC, errors::AppError},
};

use crate::infrastructure::contracts::{ContractHandler, contract_registry::ContractRegistry};

pub struct IndexEngineUCImpl<RL: EVMLogsRepository> {
    evm_log_repo: RL,
    contract_registry: ContractRegistry,
    batch_size: u64,
}

impl<RL: EVMLogsRepository> IndexEngineUCImpl<RL> {
    pub fn new(evm_log_repo: RL, contract_registry: ContractRegistry, batch_size: u64) -> Self {
        Self {
            evm_log_repo,
            contract_registry,
            batch_size,
        }
    }

    pub async fn process_logs(&self) -> Result<(), AppError> {
        let logs: Vec<EVMLogs> = self.evm_log_repo.list(self.batch_size as i64).await?;

        if logs.is_empty() {
            return Ok(());
        }

        println!("Processing batch of {} logs in parallel", logs.len());

        // Process up to 10 logs concurrently
        let results: Vec<_> = stream::iter(logs)
            .map(|log| async move {
                let log_id = log.id;
                match self.process_and_delete_log(log).await {
                    Ok(_) => (log_id, true),
                    Err(e) => {
                        eprintln!(" [{}] Error: {}", log_id, e);
                        (log_id, false)
                    }
                }
            })
            .buffer_unordered(10)
            .collect()
            .await;

        let processed = results.iter().filter(|(_, success)| *success).count();
        let failed = results.len() - processed;

        println!("Batch complete: {} processed, {} failed", processed, failed);
        Ok(())
    }

    async fn process_and_delete_log(&self, log: EVMLogs) -> Result<(), AppError> {
        let log_id = log.id;
        let processor = self.contract_registry.get_processor(log.address)?;

        processor.process(log).await?;
        self.evm_log_repo.delete(log_id).await?;

        Ok(())
    }
}

#[async_trait]
impl<RL> IndexEngineUC for IndexEngineUCImpl<RL>
where
    RL: EVMLogsRepository + Send + Sync,
{
    async fn on_pool_created(&self, data: PoolCreatedRequest) -> Result<(), AppError> {
        todo!()
    }
    async fn on_owner_changed(&self, data: OwnerChangedRequest) -> Result<(), AppError> {
        todo!()
    }
    async fn on_fee_amount_enabled(&self, data: FeeAmountEnabledRequest) -> Result<(), AppError> {
        todo!()
    }
}
