use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct EVMSyncLogs {
    pub address: [u8; 20],
    pub last_synced_block_number: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
