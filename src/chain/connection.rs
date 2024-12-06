use crate::config::{ChainConfig, RpcEndpoint, RpcType};
use crate::metrics::MetricsCollector;
use crate::circuit_breaker::CircuitBreaker;
use crate::chain::ChainState;
use crate::error::{Error, Result};
use web3::transports::{WebSocket, Http};
use web3::Web3;
use std::sync::Arc;
use tokio::sync::RwLock;
use web3::types::{Log, BlockNumber, FilterBuilder, H160};
use std::time::Duration;
use std::str::FromStr;
use futures::Stream;

type EventStream = dyn Stream<Item = web3::Result<Log>> + Send + 'static;

#[derive(Clone)]
pub enum Transport {
    WebSocket(Web3<WebSocket>),
    Http(Web3<Http>),
}

impl Transport {
    pub async fn new(endpoint: &RpcEndpoint) -> Result<Self> {
        match endpoint.rpc_type {
            RpcType::WebSocket => {
                let transport = WebSocket::new(&endpoint.url).await.map_err(Error::Web3Error)?;
                Ok(Transport::WebSocket(Web3::new(transport)))
            },
            RpcType::Http => {
                let transport = Http::new(&endpoint.url).map_err(Error::Web3Error)?;
                Ok(Transport::Http(Web3::new(transport)))
            },
        }
    }
}

pub struct ChainConnection {
    transport: Option<Transport>,
    pub config: ChainConfig,
    current_endpoint: Arc<RwLock<Option<RpcEndpoint>>>,
    pub state: Arc<ChainState>,
    pub circuit_breaker: CircuitBreaker,
    polling_interval: Duration,
}

impl ChainConnection {
    pub async fn new(
        config: ChainConfig,
        metrics: MetricsCollector,
    ) -> Result<Self> {
        let state = Arc::new(ChainState::new(metrics.clone()));
        
        let circuit_breaker = CircuitBreaker::new_from_config(
            config.rpcs[0].circuit_breaker.clone(),
            metrics.clone(),
        );

        let mut connection = Self {
            transport: None,
            config,
            current_endpoint: Arc::new(RwLock::new(None)),
            state,
            circuit_breaker,
            polling_interval: Duration::from_secs(2),
        };

        connection.connect().await?;
        Ok(connection)
    }

    pub async fn subscribe_to_events(&mut self) -> Result<Box<EventStream>> {
        self.ensure_connection().await?;
        
        let transport = self.transport.as_ref().ok_or(Error::NotConnected)?;
        let contract = H160::from_str(&self.config.contract_address)
            .map_err(|_| Error::InvalidAddress)?;
        
        Ok(match transport {
            Transport::WebSocket(web3) => {
                let current_block = web3.eth().block_number().await?;
                let filter = FilterBuilder::default()
                    .address(vec![contract])
                    .from_block(BlockNumber::Number(current_block))
                    .build();
                
                let stream = web3.eth_subscribe().subscribe_logs(filter).await?;
                Box::new(stream)
            },
            Transport::Http(web3) => {
                let _filter = FilterBuilder::default()
                    .address(vec![contract])
                    .build();

                let web3 = web3.clone();
                let interval = self.polling_interval;
                
                let stream = async_stream::stream! {
                    let mut last_block = web3.eth().block_number().await?;
                    
                    loop {
                        tokio::time::sleep(interval).await;
                        
                        let current_block = web3.eth().block_number().await?;
                        if current_block > last_block {
                            let filter = FilterBuilder::default()
                                .address(vec![contract])
                                .from_block(BlockNumber::Number(last_block + 1))
                                .to_block(BlockNumber::Number(current_block))
                                .build();
                            
                            match web3.eth().logs(filter).await {
                                Ok(logs) => {
                                    for log in logs {
                                        yield Ok(log);
                                    }
                                }
                                Err(e) => yield Err(e),
                            }
                            
                            last_block = current_block;
                        }
                    }
                };
                
                Box::new(stream)
            }
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let max_retries = 3;
        let retry_delay = Duration::from_secs(5);
    
        for endpoint in &self.config.rpcs {
            let mut attempts = 0;
            
            while attempts < max_retries {
                tracing::info!("Attempting to connect to {}", endpoint.url);
                
                match Transport::new(endpoint).await {
                    Ok(transport) => {
                        self.transport = Some(transport);
                        *self.current_endpoint.write().await = Some(endpoint.clone());
                        self.state.metrics.set_connection_status(true);
                        tracing::info!("Successfully connected to {}", endpoint.url);
                        return Ok(());
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Failed to connect to {} (attempt {}/{}): {}",
                            endpoint.url,
                            attempts + 1,
                            max_retries,
                            e
                        );
                        self.state.metrics.record_failure();
                        attempts += 1;
                        
                        if attempts < max_retries {
                            tokio::time::sleep(retry_delay).await;
                        }
                    }
                }
            }
        }
        
        tracing::error!("Failed to connect to any RPC endpoint after all retries");
        Err(Error::NoHealthyEndpoints)
    }

    pub async fn ensure_connection(&mut self) -> Result<()> {
        if self.transport.is_none() {
            self.connect().await?;
        }

        match &self.transport {
            Some(transport) => {
                let block_check = match transport {
                    Transport::WebSocket(web3) => web3.eth().block_number().await,
                    Transport::Http(web3) => web3.eth().block_number().await,
                };

                match block_check {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        tracing::warn!("Connection check failed, attempting reconnect");
                        self.connect().await
                    }
                }
            }
            None => self.connect().await,
        }
    }
}