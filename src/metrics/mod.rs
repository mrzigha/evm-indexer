use prometheus::{
    IntCounterVec, IntGaugeVec, HistogramVec,
    opts, register_int_counter_vec, register_int_gauge_vec, register_histogram_vec,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref ENDPOINT_FAILURES: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_endpoint_failures", "Number of RPC endpoint failures"),
        &["chain", "endpoint"]
    ).unwrap();

    static ref ENDPOINT_LATENCY: HistogramVec = register_histogram_vec!(
        "indexer_endpoint_latency",
        "RPC endpoint latency in seconds",
        &["chain", "endpoint"],
        vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    ).unwrap();

    static ref ACTIVE_CONNECTIONS: IntGaugeVec = register_int_gauge_vec!(
        opts!("indexer_active_connections", "Number of active RPC connections"),
        &["chain", "endpoint"]
    ).unwrap();

    static ref EVENTS_PROCESSED: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_events_processed", "Number of events processed"),
        &["chain", "event_type"]
    ).unwrap();

    static ref LAST_BLOCK_HEIGHT: IntGaugeVec = register_int_gauge_vec!(
        opts!("indexer_last_block_height", "Last processed block height"),
        &["chain"]
    ).unwrap();

    static ref CIRCUIT_BREAKER_TRIPS: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_circuit_breaker_trips", "Number of circuit breaker trips"),
        &["chain", "endpoint"]
    ).unwrap();

    static ref EVENTS_RECEIVED: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_events_received", "Total number of events received before decoding"),
        &["chain"]
    ).unwrap();

    static ref EVENTS_DECODE_FAILURES: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_events_decode_failures", "Number of event decode failures"),
        &["chain", "reason"]
    ).unwrap();

    static ref EVENTS_BY_TYPE: IntCounterVec = register_int_counter_vec!(
        opts!("indexer_events_by_type", "Number of events by type"),
        &["chain", "event_type"]
    ).unwrap();

    static ref EVENT_PROCESSING_DURATION: HistogramVec = register_histogram_vec!(
        "indexer_event_processing_duration",
        "Time taken to process events in seconds",
        &["chain", "event_type"],
        vec![0.001, 0.01, 0.1, 0.5, 1.0, 2.0, 5.0]
    ).unwrap();
}

#[derive(Clone)]
pub struct MetricsCollector {
    chain_name: String,
    endpoint_url: String,
}

impl MetricsCollector {
    pub fn new(chain_name: &str, endpoint_url: &str) -> Self {
        Self {
            chain_name: chain_name.to_string(),
            endpoint_url: endpoint_url.to_string(),
        }
    }

    pub fn record_failure(&self) {
        ENDPOINT_FAILURES
            .with_label_values(&[&self.chain_name, &self.endpoint_url])
            .inc();
    }

    pub fn record_latency(&self, duration: std::time::Duration) {
        ENDPOINT_LATENCY
            .with_label_values(&[&self.chain_name, &self.endpoint_url])
            .observe(duration.as_secs_f64());
    }

    pub fn set_connection_status(&self, is_connected: bool) {
        ACTIVE_CONNECTIONS
            .with_label_values(&[&self.chain_name, &self.endpoint_url])
            .set(if is_connected { 1 } else { 0 });
    }

    pub fn record_event_processed(&self, event_type: &str) {
        EVENTS_PROCESSED
            .with_label_values(&[&self.chain_name, event_type])
            .inc();
    }

    pub fn update_block_height(&self, height: u64) {
        LAST_BLOCK_HEIGHT
            .with_label_values(&[&self.chain_name])
            .set(height as i64);
    }

    pub fn record_circuit_breaker_trip(&self) {
        CIRCUIT_BREAKER_TRIPS
            .with_label_values(&[&self.chain_name, &self.endpoint_url])
            .inc();
    }

    pub fn record_event_received(&self) {
        EVENTS_RECEIVED
            .with_label_values(&[&self.chain_name])
            .inc();
    }

    pub fn record_event_decode_failure(&self, reason: &str) {
        EVENTS_DECODE_FAILURES
            .with_label_values(&[&self.chain_name, reason])
            .inc();
    }

    pub fn record_event_by_type(&self, event_type: &str) {
        EVENTS_BY_TYPE
            .with_label_values(&[&self.chain_name, event_type])
            .inc();
    }

    pub fn observe_event_processing_time(&self, event_type: &str, duration: f64) {
        EVENT_PROCESSING_DURATION
            .with_label_values(&[&self.chain_name, event_type])
            .observe(duration);
    }
}
