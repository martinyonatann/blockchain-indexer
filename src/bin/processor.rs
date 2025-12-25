use std::{collections::HashMap, time::Duration};

use blockchain_indexer::{
    config::load_config,
    infrastructure::{
        abi::abi_loader::AbiLoader, contracts::contract_registry::ContractRegistry,
        database::pgsql::new_database_connection,
    },
    services::{
        repository::{EVMLogsRepository, evm_logs::evm_log_repository::EVMLogsRepositoryImpl},
        usecase::index_engine::index_engine_uc::IndexEngineUCImpl,
    },
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let sleep_duration = Duration::from_secs(config.processor.poll_interval.parse::<u64>()?);
    let batch_size = config.processor.batch_size.parse::<u64>()?;
    let abi_loader = AbiLoader::new(config.processor.artifacts_base_path);
    let mut contract_name_by_address: HashMap<String, String> = HashMap::new();
    let contract = config.processor.contracts;
    contract.split(",").for_each(|contract| {
        let contract_details: Vec<&str> = contract
            .split(":")
            .map(|address_str| address_str.trim())
            .collect();

        let (contract_name, contract_address) = (contract_details[0], contract_details[1]);
        contract_name_by_address.insert(contract_address.to_lowercase(), contract_name.to_string());
    });

    let db_pool = new_database_connection(
        &config.database.database_url,
        config.database.max_connections,
    )
    .await?;

    let evm_logs_repo = EVMLogsRepositoryImpl::new(db_pool.clone());
    let contract_registry = ContractRegistry::new(contract_name_by_address, abi_loader);
    let index_engine_uc =
        IndexEngineUCImpl::new(evm_logs_repo.clone(), contract_registry, batch_size);

    'l: loop {
        let unprocessed_count = match evm_logs_repo.count().await {
            Ok(count) => count,
            Err(err) => {
                eprintln!(
                    "Error counting unprocessed logs: {err}. Sleeping for {} seconds...",
                    sleep_duration.as_secs()
                );

                sleep(sleep_duration).await;
                continue 'l;
            }
        };

        match unprocessed_count {
            Some(count) => {
                println!("Found {count} unprocessed logs. Starting processing...",);

                if let Err(err) = index_engine_uc.process_logs().await {
                    eprintln!("Error processing logs: {err}");
                }
            }
            None => {
                println!(
                    "No unprocessed logs found. Sleeping for {} seconds...",
                    sleep_duration.as_secs()
                );
                sleep(sleep_duration).await;
            }
        }
    }
}
