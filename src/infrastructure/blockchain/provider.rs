use std::str::FromStr;

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockNumberOrTag, Filter, Log},
};
use async_trait::async_trait;

use crate::services::usecase::errors::AppError;

#[async_trait]
pub trait BlockchainProvider: Send + Sync {
    async fn get_block_number(&self) -> Result<u64, AppError>;
    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>, AppError>;
}

pub struct EVMProvider {
    provider: Box<dyn Provider>,
}

impl EVMProvider {
    pub async fn new(rpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;
        Ok(Self {
            provider: Box::new(provider),
        })
    }
}

#[async_trait]
impl BlockchainProvider for EVMProvider {
    async fn get_block_number(&self) -> Result<u64, AppError> {
        self.provider
            .get_block_number()
            .await
            .map_err(|e| AppError::RpcError(e.to_string()))
    }

    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>, AppError> {
        self.provider
            .get_logs(filter)
            .await
            .map_err(|e| AppError::RpcError(e.to_string()))
    }
}

pub fn create_log_filter(
    address: &str,
    from_block: u64,
    to_block: u64,
) -> Result<Filter, AppError> {
    let addr = Address::from_str(address).map_err(|e| AppError::InvalidAddress(e.to_string()))?;

    Ok(Filter::new()
        .address(addr)
        .from_block(BlockNumberOrTag::Number(from_block))
        .to_block(BlockNumberOrTag::Number(to_block)))
}
