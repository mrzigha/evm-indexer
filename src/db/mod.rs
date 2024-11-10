use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use crate::config::DatabaseConfig;
use crate::error::Result;

pub mod models;

pub struct DatabaseConnection {
    pub client: Client,
    pub database: Database,
}

impl DatabaseConnection {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let username = std::env::var("EVM_INDEXER_DATABASE_USERNAME")
            .ok()
            .or(config.username.clone());
            
        let password = std::env::var("EVM_INDEXER_DATABASE_PASSWORD")
            .ok()
            .or(config.password.clone());
        let uri = match (username, password) {
            (Some(user), Some(pass)) => format!(
                "mongodb://{}:{}@{}:{}",
                user, pass, config.db_host, config.db_port
            ),
            _ => format!(
                "mongodb://{}:{}",
                config.db_host, config.db_port
            ),
        };
        let mut client_options = ClientOptions::parse(&uri).await?;
        client_options.app_name = Some("evm-indexer".to_string());

        let client = Client::with_options(client_options)?;

        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await?;

        let database = client.database(&config.db_name);
        
        tracing::info!("Successfully connected to MongoDB at {}:{}", config.db_host, config.db_port);
        
        Ok(Self {
            client,
            database,
        })
    }
}
