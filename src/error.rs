use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Web3 error: {0}")]
    Web3Error(#[from] web3::Error),

    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("ABI error: {0}")]
    AbiError(#[from] ethabi::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("No healthy RPC endpoints available")]
    NoHealthyEndpoints,

    #[error("Connection not established")]
    NotConnected,

    #[error("Circuit breaker open")]
    CircuitBreakerOpen,

    #[error("Unknown event")]
    UnknownEvent,

    #[error("Recovery failed")]
    RecoveryFailed,

    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Invalid address format")]
    InvalidAddress,

    #[error("FromHex error: {0}")]
    FromHex(#[from] rustc_hex::FromHexError),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Subscription error: {0}")]
    SubscriptionError(String),

    #[error("Database authentication failed")]
    DatabaseAuthError,

    #[error("Required environment variable not found: {0}")]
    MissingEnvVar(String),

    #[error("Configuration file not found at path: {0}")]
    ConfigFileNotFound(String),

    #[error("ABI file not found at path: {0}")]
    AbiFileNotFound(String),

    #[error("Log error: {0}")]
    LogError(String),

    #[error("Invalid RPC type specified")]
    InvalidRpcType,
}

pub type Result<T> = std::result::Result<T, Error>;
