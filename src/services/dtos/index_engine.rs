use crate::services::usecase::errors::AppError;

pub struct PoolCreatedRequest {
    pub pool: String,
    pub token0: String,
    pub token1: String,
    pub fee: u32,
}

pub struct OwnerChangedRequest {
    pub previous: String,
    pub new_owner: String,
}

pub struct FeeAmountEnabledRequest {
    pub fee: u32,
    pub tick_spacing: i32,
}

#[derive(Debug)]
pub enum BatchResult {
    NoLogsFound,
    BatchProcessed {
        total: u64,
        processed: usize,
        errors: Vec<(i64, AppError)>,
    },
}
