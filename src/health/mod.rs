use crate::config::RpcEndpoint;
use crate::metrics::MetricsCollector;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct HealthCheck {
    metrics: MetricsCollector,
    endpoint_states: Arc<DashMap<String, EndpointHealth>>,
}

#[derive(Clone, Debug)]
pub struct EndpointHealth {
    pub last_checked: Instant,
    pub is_healthy: bool,
    pub latency: Duration,
    pub block_height: u64,
    pub peer_count: u64,
}

impl HealthCheck {
    pub fn new(metrics: MetricsCollector) -> Self {
        Self {
            metrics,
            endpoint_states: Arc::new(DashMap::new()),
        }
    }

    pub async fn check_endpoint(&self, endpoint: &RpcEndpoint) -> bool {
        let start = Instant::now();
        let transport = match web3::transports::WebSocket::new(&endpoint.url).await {
            Ok(ws) => ws,
            Err(_) => {
                self.record_failure(&endpoint.url);
                return false;
            }
        };

        let web3 = web3::Web3::new(transport);
        let (block_number, peer_count) = match tokio::join!(
            web3.eth().block_number(),
            web3.net().peer_count(),
        ) {
            (Ok(block), Ok(peers)) => (block, peers),
            _ => {
                self.record_failure(&endpoint.url);
                return false;
            }
        };

        let duration = start.elapsed();
        self.record_success(&endpoint.url, duration, block_number.as_u64(), peer_count.as_u64());
        true
    }

    fn record_success(&self, url: &str, latency: Duration, block_height: u64, peer_count: u64) {
        self.metrics.record_latency(latency);
        self.endpoint_states.insert(
            url.to_string(),
            EndpointHealth {
                last_checked: Instant::now(),
                is_healthy: true,
                latency,
                block_height,
                peer_count,
            },
        );
    }

    fn record_failure(&self, url: &str) {
        self.metrics.record_failure();
        if let Some(mut health) = self.endpoint_states.get_mut(url) {
            health.is_healthy = false;
        }
    }

    pub fn get_best_endpoint(&self, endpoints: &[RpcEndpoint]) -> Option<RpcEndpoint> {
        endpoints
            .iter()
            .filter(|e| {
                self.endpoint_states
                    .get(&e.url)
                    .map(|h| h.is_healthy)
                    .unwrap_or(true)
            })
            .min_by_key(|e| {
                let health = self.endpoint_states.get(&e.url);
                (
                    e.priority,
                    health.map(|h| h.latency).unwrap_or(Duration::from_secs(60))
                )
            })
            .cloned()
    }
}
