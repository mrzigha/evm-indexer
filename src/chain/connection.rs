use crate::error::Result;
use crate::config::{ChainConfig, RpcEndpoint};
use crate::metrics::MetricsCollector;
use crate::circuit_breaker::CircuitBreaker;
use web3::transports::WebSocket;
use web3::Web3;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::chain::ChainState;
use web3::types::{H160, Log};
use std::str::FromStr;
use web3::api::SubscriptionStream;
use crate::error::Error;

pub struct ChainConnection {
    web3: Option<Web3<WebSocket>>,
    pub config: ChainConfig,
    current_endpoint: Arc<RwLock<Option<RpcEndpoint>>>,
    pub state: Arc<ChainState>,
    pub circuit_breaker: CircuitBreaker,
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
            web3: None,
            config,
            current_endpoint: Arc::new(RwLock::new(None)),
            state,
            circuit_breaker,
        };

        connection.connect().await?;
        Ok(connection)
    }

    async fn connect(&mut self) -> Result<()> {
        let max_retries = 3;
        let retry_delay = std::time::Duration::from_secs(5);
    
        for endpoint in &self.config.rpcs {
            let mut attempts = 0;
            
            while attempts < max_retries {
                tracing::info!("Attempting to connect to {}", endpoint.url);
                
                match WebSocket::new(&endpoint.url).await {
                    Ok(transport) => {
                        self.web3 = Some(Web3::new(transport));
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
        Err(crate::error::Error::NoHealthyEndpoints)
    }

    pub async fn ensure_connection(&mut self) -> Result<()> {
        if self.web3.is_none() {
            self.connect().await?;
        }

        if let Some(web3) = &self.web3 {
            match web3.eth().block_number().await {
                Ok(_) => Ok(()),
                Err(_) => {
                    tracing::warn!("Connection check failed, attempting reconnect");
                    self.reconnect().await
                }
            }
        } else {
            self.connect().await
        }
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        self.web3 = None;
        self.state.metrics.set_connection_status(false);
        self.connect().await
    }

    pub async fn subscribe_to_events(&mut self) -> Result<SubscriptionStream<WebSocket, Log>> {
        const SUBSCRIPTION_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(5);
        const MAX_SUBSCRIPTION_ATTEMPTS: u32 = 3;
    
        let mut attempts = 0;
        while attempts < MAX_SUBSCRIPTION_ATTEMPTS {
            self.ensure_connection().await?;
            
            let web3 = self.web3.as_ref().ok_or(Error::NotConnected)?;
            let contract = H160::from_str(&self.config.contract_address)
                .map_err(|_| Error::InvalidAddress)?;
            
            let current_block = web3.eth().block_number().await?;
            
            let filter = web3::types::FilterBuilder::default()
                .address(vec![contract])
                .from_block(web3::types::BlockNumber::Number(current_block))
                .build();
                
            match web3.eth_subscribe()
                .subscribe_logs(filter)
                .await 
            {
                Ok(subscription) => {
                    tracing::info!(
                        "Successfully subscribed to events for contract {} from block {} (live monitoring)",
                        self.config.contract_address,
                        current_block
                    );
                    return Ok(subscription);
                },
                Err(e) => {
                    attempts += 1;
                    tracing::warn!(
                        "Failed to subscribe to events (attempt {}/{}): {:?}",
                        attempts,
                        MAX_SUBSCRIPTION_ATTEMPTS,
                        e
                    );
                    
                    if attempts < MAX_SUBSCRIPTION_ATTEMPTS {
                        tokio::time::sleep(SUBSCRIPTION_RETRY_DELAY).await;
                        self.reconnect().await?;
                    } else {
                        return Err(Error::Web3Error(e));
                    }
                }
            }
        }
        
        Err(Error::NoHealthyEndpoints)
    }
}
