use alloy::{
    hex,
    json_abi::JsonAbi,
    primitives::{Address, keccak256},
    rpc::types::Log,
};
use async_trait::async_trait;

use crate::{
    infrastructure::{abi::abi_loader::AbiLoader, contracts::ContractHandler},
    services::{entities::evm_logs::EVMLogs, usecase::errors::AppError},
};

pub struct UniswapV3Factory {
    #[allow(dead_code)]
    pub address: Address,
    pub abi: JsonAbi,
}

impl UniswapV3Factory {
    pub fn new(address: &str, loader: AbiLoader) -> Result<Self, AppError> {
        let addr = address
            .parse::<Address>()
            .map_err(|_| AppError::InvalidAddress(address.into()))?;

        let abi = loader.load(Self::NAME)?;

        Ok(Self { address: addr, abi })
    }
}

#[async_trait]
impl ContractHandler for UniswapV3Factory {
    const NAME: &str = "uniswap_v3_factory";

    fn event_signature_to_name(&self, signature: [u8; 32]) -> Result<String, AppError> {
        let log_sig_hex = format!("0x{}", hex::encode(signature));

        let event = self.abi.events.iter().find(|(_name, params)| {
            let selector = keccak256(params[0].signature()).to_string();

            selector == log_sig_hex
        });

        if let Some((name, _)) = event {
            Ok(name.clone())
        } else {
            Err(AppError::MissingEvent(Self::NAME.into(), log_sig_hex))
        }
    }

    fn handle_event(
        &self,
        event_name: &str,
        _: &Log,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send {
        async move {
            match event_name {
                "PoolCreated" => {
                    println!("PoolCreated event");
                    Ok(())
                }
                unsupported => Err(AppError::MissingEventHandler(
                    Self::NAME.into(),
                    unsupported.into(),
                )),
            }
        }
    }

    fn process(
        &self,
        unprocessed_log: EVMLogs,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send
    where
        Self: Sync,
    {
        async move {
            let event_name = self.event_signature_to_name(unprocessed_log.event_signature)?;
            let log: Log = unprocessed_log.try_into()?;
            self.handle_event(&event_name, &log).await?;
            Ok(())
        }
    }
}
