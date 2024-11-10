use chrono::{DateTime, Duration as ChronoDuration, Utc};
use parking_lot::RwLock;
use std::sync::Arc;
use crate::metrics::MetricsCollector;
use serde::{Deserialize, Deserializer};
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    #[serde(deserialize_with = "deserialize_duration_from_secs")]
    pub reset_timeout: Duration,
    #[serde(deserialize_with = "deserialize_duration_from_secs")]
    pub half_open_timeout: Duration,
}

#[derive(Clone, Debug)]
pub struct InternalCircuitBreakerConfig {
    pub failure_threshold: u32,
    pub reset_timeout: ChronoDuration,
    pub half_open_timeout: ChronoDuration,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CircuitState {
    Closed,
    Open(DateTime<Utc>),
    HalfOpen(DateTime<Utc>),
}

pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    config: InternalCircuitBreakerConfig,
    failures: Arc<RwLock<u32>>,
    metrics: MetricsCollector,
}


fn deserialize_duration_from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}

#[derive(Debug, Deserialize, Clone)]
pub struct HealthCheckConfig {
    pub interval_secs: u64,
    pub timeout_secs: u64,
    pub min_peers: u32,
    pub max_blocks_behind: u64,
}

impl CircuitBreaker {
    pub fn new(config: InternalCircuitBreakerConfig, metrics: MetricsCollector) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            config,
            failures: Arc::new(RwLock::new(0)),
            metrics,
        }
    }

    pub fn new_from_config(config: crate::config::CircuitBreakerConfig, metrics: MetricsCollector) -> Self {
        let internal_config = InternalCircuitBreakerConfig::from(config);
        Self::new(internal_config, metrics)
    }

    pub fn record_success(&self) {
        let mut state = self.state.write();
        match *state {
            CircuitState::HalfOpen(_) => {
                *state = CircuitState::Closed;
                *self.failures.write() = 0;
            }
            CircuitState::Closed => {
                *self.failures.write() = 0;
            }
            _ => {}
        }
    }

    pub fn record_failure(&self) -> bool {
        let mut state = self.state.write();
        let mut failures = self.failures.write();

        match *state {
            CircuitState::Closed => {
                *failures += 1;
                if *failures >= self.config.failure_threshold {
                    *state = CircuitState::Open(Utc::now());
                    self.metrics.record_circuit_breaker_trip();
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen(_) => {
                *state = CircuitState::Open(Utc::now());
                self.metrics.record_circuit_breaker_trip();
                true
            }
            CircuitState::Open(opened_at) => {
                if opened_at + self.config.reset_timeout <= Utc::now() {
                    *state = CircuitState::HalfOpen(Utc::now());
                    *failures = 0;
                    false
                } else {
                    true
                }
            }
        }
    }

    pub fn can_execute(&self) -> bool {
        let state = self.state.read();
        match *state {
            CircuitState::Closed => true,
            CircuitState::HalfOpen(attempt_at) => {
                (attempt_at + self.config.half_open_timeout) <= Utc::now()
            }
            CircuitState::Open(opened_at) => {
                (opened_at + self.config.reset_timeout) <= Utc::now()
            }
        }
    }
}

impl From<crate::config::CircuitBreakerConfig> for InternalCircuitBreakerConfig {
    fn from(config: crate::config::CircuitBreakerConfig) -> Self {
        Self {
            failure_threshold: config.failure_threshold,
            reset_timeout: ChronoDuration::seconds(Duration::from_secs(config.reset_timeout).as_secs() as i64),
            half_open_timeout: ChronoDuration::seconds(Duration::from_secs(config.half_open_timeout).as_secs() as i64),
        }
    }
}
