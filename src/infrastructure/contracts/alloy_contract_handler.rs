use alloy::{
    hex,
    json_abi::JsonAbi,
    primitives::{Address, Log, keccak256},
};

use crate::{
    infrastructure::{abi::abi_loader::AbiLoader, contracts::ContractHandler},
    services::usecase::errors::AppError,
};

pub struct AlloyContractHandler {
    pub address: Address,
    pub abi: JsonAbi,
}

impl ContractHandler for AlloyContractHandler {
    const NAME: &str = "uniswap_v3_factory";

    fn new(address: &str, loader: AbiLoader) -> Result<Self, AppError> {
        let addr = address
            .parse::<Address>()
            .map_err(|_| AppError::InvalidAddress(address.into()))?;

        let abi = loader.load(Self::NAME)?;

        Ok(Self { address: addr, abi })
    }

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
}
