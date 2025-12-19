use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct EvmChains {
    pub id: i64,
    pub name: String,
    pub last_synced_block_number: i64,
    pub block_time: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
