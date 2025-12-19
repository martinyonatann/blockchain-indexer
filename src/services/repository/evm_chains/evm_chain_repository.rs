use async_trait::async_trait;
use sqlx::PgPool;

use crate::services::{
    entities::evm_chains::EvmChains, repository::interfaces::EVMChainRepository,
};

pub struct EVMChainRepositoryImpl {
    pool: PgPool,
}

impl EVMChainRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EVMChainRepository for EVMChainRepositoryImpl {
    async fn fetch_by_id(&self, id: u64) -> Result<EvmChains, sqlx::Error> {
        let query = r#"SELECT * FROM evm_chains WHERE id = $1"#;

        sqlx::query_as::<_, EvmChains>(query)
            .bind(id as i64)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_last_synced_block_number(
        &self,
        id: u64,
        block_number: u64,
    ) -> Result<EvmChains, sqlx::Error> {
        let query =
            r#"UPDATE evm_chains SET last_synced_block_number = $1 WHERE id = $2 RETURNING *"#;

        let chain = sqlx::query_as::<_, EvmChains>(query)
            .bind(block_number as i64)
            .bind(id as i64)
            .fetch_one(&self.pool)
            .await?;

        Ok(chain)
    }
}
