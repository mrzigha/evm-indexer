pub mod connection;
pub mod event_listener;

use crate::metrics::MetricsCollector;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ChainState {
    pub last_processed_block: Arc<RwLock<u64>>,
    pub metrics: MetricsCollector,
}

impl ChainState {
    pub fn new(metrics: MetricsCollector) -> Self {
        Self {
            last_processed_block: Arc::new(RwLock::new(0)),
            metrics,
        }
    }

    pub async fn update_block(&self, block: u64) {
        let mut last_block = self.last_processed_block.write().await;
        *last_block = block;
        self.metrics.update_block_height(block);
    }
}
