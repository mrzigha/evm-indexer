pub mod config;
pub mod db;
pub mod chain;
pub mod decoder;
pub mod metrics;
pub mod health;
pub mod error;
pub mod circuit_breaker;
pub mod sync;

pub use config::Config;
pub use db::DatabaseConnection;
pub use decoder::abi::EventDecoder;
pub use chain::{
    connection::ChainConnection,
    event_listener::EventListener,
};
pub use metrics::MetricsCollector;
pub use health::HealthCheck;
pub use error::Error;
pub use circuit_breaker::CircuitBreaker;

pub type Result<T> = std::result::Result<T, Error>;
