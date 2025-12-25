use thiserror::Error;

use crate::services::entities::evm_logs::EVMLogsError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid ChainID: `{0}`")]
    InvalidChainID(String),

    #[error("Missing Contract Abi File : `{0}`")]
    MissingContractAbiFile(String),

    #[error("Invalid Abi File : `{0}`")]
    InvalidAbiFile(String),

    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Missing event in contract `{0}` for signature `{1}`")]
    MissingEvent(String, String),

    #[error("Missing event handler `{0}` for contract `{1}`")]
    MissingEventHandler(String, String),

    #[error("unsuported contract `{0}`")]
    UnsupportedContract(String),

    #[error("unsuported address `{0}`")]
    UnsupportedAddress(String),
}

impl From<alloy::transports::TransportError> for AppError {
    fn from(err: alloy::transports::TransportError) -> Self {
        AppError::ProviderError(err.to_string())
    }
}

impl From<EVMLogsError> for AppError {
    fn from(err: EVMLogsError) -> Self {
        match err {
            EVMLogsError::InvalidLogData => AppError::InternalServerError,
            EVMLogsError::InvalidBlockNumber(bn) => AppError::InvalidAddress(bn),
        }
    }
}
