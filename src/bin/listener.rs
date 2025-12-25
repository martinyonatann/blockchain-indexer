use std::{sync::Arc, time::Duration};

use blockchain_indexer::{
    config::load_config,
    infrastructure::{blockchain::provider::EVMProvider, database::pgsql::new_database_connection},
    services::{
        delivery::event::evm_log_listener::EVMLogListener,
        repository::{
            EVMChainRepository, evm_chains::evm_chain_repository::EVMChainRepositoryImpl,
            evm_logs::evm_log_repository::EVMLogsRepositoryImpl,
            evm_sync_logs::evm_sync_logs::EVMSyncLogsRepositoryImpl,
        },
        usecase::index_logs::index_log_uc::IndexLogUCImpl,
    },
};
use tokio::task::JoinSet;
use tower::{Service, ServiceBuilder, ServiceExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let db_pool = new_database_connection(
        &config.database.database_url,
        config.database.max_connections,
    )
    .await?;

    let provider = EVMProvider::new(&config.listener.rpc_url).await?;
    let evm_chain_repo = EVMChainRepositoryImpl::new(db_pool.clone());
    let evm_log_repo = EVMLogsRepositoryImpl::new(db_pool.clone());
    let evm_sync_log_repo = EVMSyncLogsRepositoryImpl::new(db_pool.clone());
    let index_log_uc = Arc::new(IndexLogUCImpl::new(
        provider,
        evm_log_repo,
        evm_sync_log_repo,
        10,
    ));

    let evm_chain = evm_chain_repo.fetch_by_id(config.listener.chain_id).await?;
    let addresses: Vec<String> = config
        .listener
        .contract_addresses
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let mut futures = JoinSet::new();
    for address in addresses {
        let mut service = ServiceBuilder::new()
            .rate_limit(1, Duration::from_secs(evm_chain.block_time as u64))
            .service(EVMLogListener {
                usecase: Arc::clone(&index_log_uc),
                chain_id: config.listener.chain_id,
                address: address.to_string(),
            });

        let future = async move {
            loop {
                if service.ready().await.is_ok() {
                    match service.call(()).await {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("Failed to indexed: {:?}", err);
                        }
                    }
                }
            }
        };

        futures.spawn(future);
    }

    futures.join_all().await;

    Ok(())
}
