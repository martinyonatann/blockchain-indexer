use thiserror::Error;

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

    #[error("Missing event in contract `{0}` for signature `{1}`")]
    MissingEvent(String, String),

    #[error("Missing event handler `{0}` for contract `{1}`")]
    MissingEventHandler(String, String),
}

impl From<alloy::transports::TransportError> for AppError {
    fn from(err: alloy::transports::TransportError) -> Self {
        AppError::ProviderError(err.to_string())
    }
}
