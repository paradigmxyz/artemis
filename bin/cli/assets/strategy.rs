use async_trait::async_trait;
use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Strategy;
use ethers::providers::Middleware;

use super::types::{Action, Config, Event};

#[derive(Debug, Clone)]
pub struct StrategyName<M> {
    /// Ethers client.
    client: Arc<M>,
}

impl<M: Middleware + 'static> StrategyName<M> {
    pub fn new(client: Arc<M>, config: Config) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for StrategyName<M> {
    // state sync for strategy
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    // Process incoming events
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {}
    }
}

impl<M: Middleware + 'static> StrategyName<M> {
    // add struct strategy methods here
}
