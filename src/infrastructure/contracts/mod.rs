pub mod alloy_contract_handler;
use crate::{infrastructure::abi::abi_loader::AbiLoader, services::usecase::errors::AppError};
use alloy::primitives::Log;

pub trait ContractHandler {
    const NAME: &str;

    fn event_signature_to_name(&self, signature: [u8; 32]) -> Result<String, AppError>;

    fn handle_event(
        &self,
        event_name: &str,
        log: &Log,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;

    fn new(address: &str, loader: AbiLoader) -> Result<Self, AppError>
    where
        Self: Sized;
}
