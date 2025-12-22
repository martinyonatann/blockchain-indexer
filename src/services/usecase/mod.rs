use crate::services::{
    dtos::index_engine::{FeeAmountEnabledRequest, OwnerChangedRequest, PoolCreatedRequest},
    usecase::errors::AppError,
};

pub mod errors;
pub mod index_engine;
pub mod index_logs;

#[async_trait::async_trait]
pub trait IndexLogUC: Send + Sync {
    async fn execute(&self, chain_id: u64, address: String) -> Result<(), AppError>;
}

#[async_trait::async_trait]
pub trait IndexEngineUC: Send + Sync {
    async fn on_pool_created(&self, data: PoolCreatedRequest) -> Result<(), AppError>;
    async fn on_owner_changed(&self, data: OwnerChangedRequest) -> Result<(), AppError>;
    async fn on_fee_amount_enabled(&self, data: FeeAmountEnabledRequest) -> Result<(), AppError>;
}
