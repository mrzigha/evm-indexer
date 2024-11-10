use crate::error::Result;
use crate::chain::connection::ChainConnection;
use crate::db::models::EventLog;
use crate::decoder::abi::EventDecoder;
use backoff::backoff::Backoff;
use mongodb::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use web3::types::Log;
use backoff::ExponentialBackoff;
use futures::StreamExt;
use crate::error::Error;

pub struct EventListener {
    connection: Arc<RwLock<ChainConnection>>,
    decoder: EventDecoder,
    db: Database,
}

impl EventListener {
    pub fn new(
        connection: ChainConnection,
        decoder: EventDecoder,
        db: Database,
    ) -> Self {
        Self {
            connection: Arc::new(RwLock::new(connection)),
            decoder,
            db,
        }
    }

    pub async fn start(&self) -> Result<()> {
        loop {
            match self.listen_events().await {
                Ok(_) => {},
                Err(e) => {
                    tracing::error!("Error in event listener: {:?}", e);
                    let mut connection = self.connection.write().await;
                    connection.reconnect().await?;
                }
            }
        }
    }

    async fn listen_events(&self) -> Result<()> {
        let mut backoff = ExponentialBackoff {
            initial_interval: std::time::Duration::from_secs(1),
            max_interval: std::time::Duration::from_secs(30),
            max_elapsed_time: None,
            ..ExponentialBackoff::default()
        };

        loop {
            let mut connection = self.connection.write().await;
            match connection.subscribe_to_events().await {
                Ok(mut subscription) => {
                    drop(connection);
                    backoff.reset();
                    
                    tracing::info!("Starting to process events...");
                    while let Some(result) = subscription.next().await {

                        let connection = self.connection.read().await;
                        connection.state.metrics.record_event_received();

                        match result {
                            Ok(log) => {

                                if let Some(block_number) = log.block_number {
                                    connection.state.metrics.update_block_height(block_number.as_u64());
                                }

                                match self.process_event(log).await {
                                    Ok(event_name) => {
                                        connection.state.metrics.record_event_by_type(&event_name);
                                        connection.state.metrics.record_event_processed(&event_name);
                                    }
                                    Err(e) => {
                                        connection.state.metrics.record_event_decode_failure("decode_error");
                                        tracing::error!("Failed to process event: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Subscription stream error: {:?}", e);
                                break;
                            }
                        }
                    }

                    tracing::warn!("Subscription stream ended, attempting to resubscribe...");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                Err(e) => {
                    tracing::error!("Failed to create subscription: {:?}", e);
                    if let Some(duration) = backoff.next_backoff() {
                        tracing::info!("Waiting {:?} before retry", duration);
                        tokio::time::sleep(duration).await;
                    }
                }
            }
        }
    }

    async fn process_event(&self, log: Log) -> Result<String> {
        let start_time = std::time::Instant::now();
        let connection = self.connection.read().await;
        connection.state.metrics.record_event_received();

        tracing::info!("Processing event from transaction: {:?}", log.transaction_hash);

        let raw_log = ethabi::RawLog {
            topics: log.topics.clone(),
            data: log.data.0.clone(),
        };

        let (event_name, params) = match self.decoder.decode_log(raw_log) {
            Ok((name, p)) => {
                tracing::info!(
                    "Successfully decoded event: {}\nParameters: {:?}",
                    name,
                    p
                );
                connection.state.metrics.record_event_by_type(&name);
                (name, p)
            },
            Err(e) => {
                tracing::error!(
                    "Failed to decode event. Topics: {:?}, Data: {:?}, Error: {:?}",
                    log.topics,
                    log.data.0,
                    e
                );
                connection.state.metrics.record_event_decode_failure(&format!("{:?}", e));
                return Err(e);
            }
        };

        if let Some(block_number) = log.block_number {
            connection.state.metrics.update_block_height(block_number.as_u64());
        }

        let event_log = EventLog {
            chain_name: connection.config.name.clone(),
            event_name: event_name.clone(),
            block_number: log.block_number.unwrap_or_default().as_u64(),
            transaction_hash: format!("{:?}", log.transaction_hash.unwrap_or_default()),
            params,
            timestamp: mongodb::bson::DateTime::now(),
        };

        if connection.circuit_breaker.can_execute() {
            let event_log_clone = event_log.clone();
            match backoff::future::retry(ExponentialBackoff::default(), || async {
                match self.db.collection("events")
                    .insert_one(event_log_clone.clone(), None)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(backoff::Error::Permanent(Error::MongoError(e)))
                }
            }).await {
                Ok(_) => {
                    tracing::info!(
                        "Successfully stored event {} from tx {:?}",
                        event_name,
                        log.transaction_hash
                    );
                    connection.circuit_breaker.record_success();
                }
                Err(e) => {
                    tracing::error!("Failed to store event in MongoDB: {:?}", e);
                    return Err(Error::StorageError("Failed to store event in MongoDB".to_string()));
                }
            }
        }

        let duration = start_time.elapsed().as_secs_f64();
        let connection = self.connection.read().await;
        connection.state.metrics.observe_event_processing_time(&event_name, duration);

        Ok(event_name)
    }
}
