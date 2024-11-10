use std::{env, path::Path};

use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize, Clone)]
pub enum RpcType {
    #[serde(rename = "ws")]
    WebSocket,
    #[serde(rename = "http")]
    Http,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcEndpoint {
    pub url: String,
    pub rpc_type: RpcType,
    pub priority: u8,
    pub health_check: HealthCheckConfig,
    pub circuit_breaker: CircuitBreakerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HealthCheckConfig {
    pub interval_secs: u64,
    pub timeout_secs: u64,
    pub min_peers: u32,
    pub max_blocks_behind: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub reset_timeout: u64,
    pub half_open_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChainConfig {
    pub name: String,
    pub contract_address: String,
    pub rpcs: Vec<RpcEndpoint>,
    pub starting_block: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub metrics_laddr: String,
    pub metrics_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub chains: Vec<ChainConfig>,
}

impl Config {
    pub fn new() -> Result<Self, crate::error::Error> {
        dotenv::dotenv().ok();

        let config_path = env::var("EVM_INDEXER_CONFIG_PATH")
            .map_err(|_| Error::MissingEnvVar("EVM_INDEXER_CONFIG_PATH".to_string()))?;

        if !Path::new(&config_path).exists() {
            return Err(Error::ConfigFileNotFound(config_path));
        }

        let builder = config::Config::builder()
            .add_source(config::File::with_name(&config_path))
            .add_source(config::Environment::with_prefix("EVM_INDEXER"));

        let config = builder.build()?;
        Ok(config.try_deserialize()?)
    }
}
