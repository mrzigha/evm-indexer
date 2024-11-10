pub mod abi;

use std::{env, path::Path, sync::Arc};
use ethabi::Contract;
use crate::error::Result;

#[derive(Clone)]
pub struct DecoderConfig {
    pub abi_path: String,
}

impl DecoderConfig {
    pub fn new() -> Result<Self> {
        let abi_path = env::var("EVM_INDEXER_ABI_PATH")
            .map_err(|_| crate::error::Error::MissingEnvVar("EVM_INDEXER_ABI_PATH".to_string()))?;

        if !Path::new(&abi_path).exists() {
            return Err(crate::error::Error::AbiFileNotFound(abi_path));
        }

        Ok(Self { abi_path })
    }
    
    pub async fn load_contract(&self) -> Result<Arc<Contract>> {
        let abi_file = tokio::fs::read_to_string(&self.abi_path).await?;
        let contract = Contract::load(abi_file.as_bytes())?;
        Ok(Arc::new(contract))
    }
}
