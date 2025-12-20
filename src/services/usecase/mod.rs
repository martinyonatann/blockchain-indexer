use crate::services::usecase::errors::AppError;

pub mod errors;
pub mod index_logs;

#[async_trait::async_trait]
pub trait IndexLogUC: Send + Sync {
    async fn execute(&self, chain_id: u64, address: String) -> Result<(), AppError>;
}
