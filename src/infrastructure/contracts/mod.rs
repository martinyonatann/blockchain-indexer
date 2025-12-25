pub mod contract_registry;
pub mod uniswap;
use alloy::rpc::types::Log;

use crate::services::{entities::evm_logs::EVMLogs, usecase::errors::AppError};
pub trait ContractHandler: Send + Sync {
    const NAME: &str;

    fn event_signature_to_name(&self, signature: [u8; 32]) -> Result<String, AppError>;

    fn handle_event(
        &self,
        event_name: &str,
        log: &Log,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn process(
        &self,
        unprocessed_log: EVMLogs,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
