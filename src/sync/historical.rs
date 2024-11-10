use crate::error::{Error, Result};
use crate::db::models::EventLog;
use crate::decoder::abi::EventDecoder;
use crate::metrics::MetricsCollector;
use mongodb::Database;
use web3::{
    types::{BlockNumber, FilterBuilder, Log, H160},
    Transport, Web3,
};
use std::str::FromStr;
use mongodb::bson::{doc, Bson};

pub struct HistoricalSync<T: Transport> {
    web3: Web3<T>,
    chain_name: String,
    contract_address: String,
    decoder: EventDecoder,
    db: Database,
    metrics: MetricsCollector,
    batch_size: u64,
}

impl<T: Transport> HistoricalSync<T> {
    pub fn new(
        web3: Web3<T>,
        chain_name: String,
        contract_address: String,
        decoder: EventDecoder,
        db: Database,
        metrics: MetricsCollector,
        batch_size: u64,
    ) -> Self {
        Self {
            web3,
            chain_name,
            contract_address,
            decoder,
            db,
            metrics,
            batch_size,
        }
    }

    pub async fn sync_to_block(&self, from_block: u64, to_block: u64) -> Result<()> {
        let mut current_block = from_block;

        while current_block < to_block {
            let end_block = std::cmp::min(current_block + self.batch_size, to_block);
            
            tracing::info!(
                "Processing historical blocks {} to {}",
                current_block,
                end_block
            );

            let logs = self.fetch_logs_batch(current_block, end_block).await?;
            self.process_logs(logs).await?;

            current_block = end_block + 1;
            self.metrics.update_block_height(current_block);
        }

        Ok(())
    }

    async fn fetch_logs_batch(&self, from_block: u64, to_block: u64) -> Result<Vec<Log>> {
        let contract = H160::from_str(&self.contract_address)
            .map_err(|_| Error::InvalidAddress)?;

        let filter = FilterBuilder::default()
            .address(vec![contract])
            .from_block(BlockNumber::Number(from_block.into()))
            .to_block(BlockNumber::Number(to_block.into()))
            .build();

        self.web3
            .eth()
            .logs(filter)
            .await
            .map_err(Error::Web3Error)
    }

    async fn process_logs(&self, logs: Vec<Log>) -> Result<()> {
        for log in logs {
            let exists = self
                .db
                .collection::<EventLog>("events")
                .find_one(
                    doc! {
                        "transaction_hash": format!("{:?}", log.transaction_hash.unwrap_or_default()),
                        "block_number": Bson::Int64(log.block_number.unwrap_or_default().as_u64() as i64)
                    },
                    None,
                )
                .await?
                .is_some();

            if !exists {
                self.metrics.record_event_received();
                
                let raw_log = ethabi::RawLog {
                    topics: log.topics.clone(),
                    data: log.data.0.clone(),
                };

                match self.decoder.decode_log(raw_log) {
                    Ok((event_name, params)) => {
                        let event_log = EventLog {
                            chain_name:  self.chain_name.clone(),
                            event_name: event_name.clone(),
                            block_number: log.block_number.unwrap_or_default().as_u64(),
                            transaction_hash: format!("{:?}", log.transaction_hash.unwrap_or_default()),
                            params,
                            timestamp: mongodb::bson::DateTime::now(),
                        };

                        self.db
                            .collection("events")
                            .insert_one(event_log, None)
                            .await?;

                        self.metrics.record_event_by_type(&event_name);
                        self.metrics.record_event_processed(&event_name);
                    }
                    Err(e) => {
                        self.metrics.record_event_decode_failure("decode_error");
                        tracing::error!("Failed to decode historical on {} event: {:?}", self.chain_name.clone(), e);
                    }
                }
            }
        }

        Ok(())
    }
}
