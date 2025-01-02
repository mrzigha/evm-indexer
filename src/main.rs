use evm_indexer::{
    chain::{connection::ChainConnection, event_listener::EventListener}, config::{Config, RpcType}, db::DatabaseConnection, decoder::{abi::EventDecoder, DecoderConfig}, health::HealthCheck, metrics::MetricsCollector, sync::historical::HistoricalSync, Error};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use std::{env, fs, path::Path, sync::Arc};
use warp::Filter;
use prometheus::Encoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv::dotenv().ok();

    let logs_path = env::var("EVM_INDEXER_LOG_PATH")
        .map_err(|_| Error::MissingEnvVar("EVM_INDEXER_LOG_PATH".to_string()))?;

    let logs_path = Path::new(&logs_path);
    if !logs_path.exists() {
        fs::create_dir_all(logs_path)?;
    }

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        logs_path,
        "evm-indexer",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let (stdout_non_blocking, _stdout_guard) = tracing_appender::non_blocking(std::io::stdout());

    tracing_subscriber::fmt()
    .with_env_filter("info,evm_indexer=debug")
    .json()
    .with_writer(non_blocking.and(stdout_non_blocking))
    .init();

    let config = Config::new()?;

    let db_connection = DatabaseConnection::new(&config.database).await?;

    let metrics_route = warp::path!("metrics").map(|| {
        let encoder = prometheus::TextEncoder::new();
        let metric_families = prometheus::default_registry().gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    });

    let metrics_addr = format!("{}:{}", config.general.metrics_laddr, config.general.metrics_port)
    .parse::<std::net::SocketAddr>()?;
    tokio::spawn(warp::serve(metrics_route).run(metrics_addr));

    let mut handles = Vec::new();
    for chain_config in config.chains {
        let db = db_connection.database.clone();
        let metrics = MetricsCollector::new(&chain_config.name, &chain_config.rpcs[0].url);
        let health_checker = Arc::new(HealthCheck::new(metrics.clone()));
        tracing::debug!("Loading ABI...");
        let decoder_config = DecoderConfig::new()?;
        let contract = decoder_config.load_contract().await?;
        let decoder = EventDecoder::new(contract);
        let historical_config = chain_config.clone();
        let historical_decoder = decoder.clone();
        let historical_db = db.clone();
        let historical_metrics = metrics.clone();

        let http_endpoint = chain_config.rpcs.iter()
        .find(|e| matches!(e.rpc_type, RpcType::Http));

        if let Some(http_endpoint) = http_endpoint {
            let transport = web3::transports::Http::new(&http_endpoint.url)?;
            let web3 = web3::Web3::new(transport);
            let current_block = web3.eth().block_number().await?.as_u64();
            tracing::info!(
                "Starting historical sync for {} from block {} to {}",
                historical_config.name,
                historical_config.starting_block.unwrap_or(0),
                current_block
            );
            
            let historical_sync = HistoricalSync::new(
                web3,
                historical_config.name,
                historical_config.contract_address,
                historical_decoder,
                historical_db,
                historical_metrics,
                1000,
            );
        
            if let Some(starting_block) = historical_config.starting_block {
                let sync_handle = tokio::spawn(async move {
                    if let Err(e) = historical_sync.sync_to_block(starting_block, current_block).await {
                        tracing::error!("Historical sync error: {:?}", e);
                    } else {
                        tracing::info!("Historical sync completed up to block {}", current_block);
                    }
                });
                handles.push(sync_handle);
            }
        }

        let chain_name = chain_config.name.clone();

        let connection = ChainConnection::new(
            chain_config.clone(),
            metrics.clone(),
        ).await?;

        let listener = EventListener::new(connection, decoder, db);

        let health_config = chain_config.clone();
        let health_clone = health_checker.clone();
        tokio::spawn(async move {
            loop {
                for endpoint in &health_config.rpcs {
                    let _ = health_clone.check_endpoint(endpoint).await;
                    tokio::time::sleep(tokio::time::Duration::from_secs(
                        endpoint.health_check.interval_secs
                    )).await;
                }
            }
        });

        let handle = tokio::spawn(async move {
            if let Err(e) = listener.start().await {
                tracing::error!("Chain {} error: {:?}", chain_name, e);
            }
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    Ok(())
}
