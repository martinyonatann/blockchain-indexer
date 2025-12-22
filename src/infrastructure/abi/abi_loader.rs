use std::fs;

use alloy::json_abi::JsonAbi;

use crate::services::usecase::errors::AppError;
pub struct AbiLoader {
    artifacts_base_path: String,
}

impl AbiLoader {
    pub fn load(&self, contract: &str) -> Result<JsonAbi, AppError> {
        let path = format!("{}/{}.json", self.artifacts_base_path, contract);

        let bytes = fs::read(&path).map_err(|_| AppError::MissingContractAbiFile(path.into()))?;

        let abi = serde_json::from_slice(&bytes)
            .map_err(|_| AppError::InvalidAbiFile(contract.into()))?;

        Ok(abi)
    }

    pub fn new(artifacts_base_path: String) -> Self {
        Self {
            artifacts_base_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;

    use crate::{infrastructure::abi::abi_loader::AbiLoader, services::usecase::errors::AppError};

    const VALID_ABI: &str = r#"
       [
           {
               "type": "function",
               "name": "balanceOf",
               "inputs": [
                   { "name": "owner", "type": "address" }
               ],
               "outputs": [
                   { "name": "balance", "type": "uint256" }
               ]
           }
       ]
       "#;

    #[test]
    fn load_valid_abi_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        let contract_name = "erc20";
        let filepath = path.join(format!("{}.json", contract_name));
        fs::write(&filepath, VALID_ABI).unwrap();

        let loader = AbiLoader::new(path.to_string_lossy().to_string());

        let result = loader.load(contract_name);

        assert!(result.is_ok());
        let abi = result.unwrap();
        assert_eq!(abi.items().len(), 1);

        assert!(abi.functions.contains_key("balanceOf"));
        let func = &abi.functions["balanceOf"][0];
        assert_eq!(func.name, "balanceOf");
        assert_eq!(func.inputs[0].name, "owner");
        assert_eq!(func.outputs[0].name, "balance");
    }

    #[test]
    fn missing_abi_file_returns_error() {
        let dir = tempdir().unwrap();
        let loader = AbiLoader::new(dir.path().to_string_lossy().to_string());

        let result = loader.load("non_existent_contract");
        match result {
            Err(AppError::MissingContractAbiFile(path)) => {
                assert!(path.contains("non_existent_contract.json"));
            }

            _ => panic!("Expected MissingContractAbiFile error"),
        }
    }

    #[test]
    fn invalid_json_returns_invalid_abi_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();

        // Write invalid JSON
        let contract_name = "broken";
        let filepath = path.join(format!("{}.json", contract_name));
        fs::write(&filepath, "{ not valid json }").unwrap();

        let loader = AbiLoader::new(path.to_string_lossy().to_string());

        let result = loader.load(contract_name);

        match result {
            Err(AppError::InvalidAbiFile(name)) => {
                assert_eq!(name, contract_name);
            }
            _ => panic!("Expected InvalidAbiFile error"),
        }
    }
}
