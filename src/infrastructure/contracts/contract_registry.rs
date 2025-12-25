use std::collections::HashMap;

use crate::{
    infrastructure::{
        abi::abi_loader::AbiLoader,
        contracts::{ContractHandler, uniswap::UniswapV3Factory},
    },
    services::usecase::errors::AppError,
    utils,
};

pub struct ContractRegistry {
    registry: HashMap<String, String>,
    loader: AbiLoader,
}

impl ContractRegistry {
    pub fn new(registry: HashMap<String, String>, loader: AbiLoader) -> Self {
        Self { registry, loader }
    }

    pub fn get_processor(&self, address: [u8; 20]) -> Result<impl ContractHandler, AppError> {
        let log_address = utils::vec_to_hex(address.to_vec());
        let contract = self.registry.get(&log_address);

        if let Some(contract_name) = contract {
            match contract_name.as_str() {
                UniswapV3Factory::NAME => UniswapV3Factory::new(&log_address, self.loader.clone()),
                unsupported => Err(AppError::UnsupportedContract(unsupported.into())),
            }
        } else {
            Err(AppError::UnsupportedAddress(log_address))
        }
    }
}
