use thiserror::Error;

use alloy::{
    primitives::{Address, Bytes, FixedBytes},
    rpc::types::Log,
};

use sqlx::{
    prelude::FromRow,
    types::{BigDecimal, chrono},
};

#[derive(Debug, Error)]
pub enum EVMLogsError {
    #[error("Failed to create a valid log data")]
    InvalidLogData,

    #[error("Invalid block number: `{0}`")]
    InvalidBlockNumber(String),
}

#[derive(Debug, Clone, FromRow)]
pub struct EVMLogs {
    pub id: i32,
    pub block_number: BigDecimal,
    pub block_hash: [u8; 32],
    pub address: [u8; 20],
    pub transaction_hash: [u8; 32],
    pub data: Vec<u8>,
    pub event_signature: [u8; 32],
    pub topics: Vec<[u8; 32]>,
    pub transaction_index: i64,
    pub log_index: i64,
    pub removed: bool,
    pub created_at: chrono::NaiveDateTime,
}

// Only needed if read logs from DB and convert back to Log
impl TryInto<Log> for EVMLogs {
    type Error = EVMLogsError;

    fn try_into(self) -> Result<Log, Self::Error> {
        let transaction_hash = FixedBytes::<32>::from(self.transaction_hash);
        let contract_address = Address::from(self.address);
        let topics: Vec<FixedBytes<32>> = self.topics.iter().map(FixedBytes::<32>::from).collect();
        let data = Bytes::from(self.data);

        let inner = alloy::primitives::Log::new(contract_address, topics, data)
            .ok_or(EVMLogsError::InvalidLogData)?;

        let block_number = self
            .block_number
            .to_string()
            .parse::<u64>()
            .map_err(|_| EVMLogsError::InvalidBlockNumber(self.block_number.to_string()))?;

        Ok(Log {
            inner,
            block_number: Some(block_number),
            block_hash: None,
            block_timestamp: None,
            transaction_hash: Some(transaction_hash),
            transaction_index: None,
            log_index: None,
            removed: false,
        })
    }
}
