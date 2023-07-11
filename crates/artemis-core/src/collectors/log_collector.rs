use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::PubsubClient,
    types::{Filter, Log},
};
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blockchain event logs based on a [Filter](Filter),
/// and generates a stream of [events](NewLog) which contains the underlying [log](Log).
pub struct LogCollector<M> {
    provider: Arc<M>,
    filter: Filter,
}

/// A new log event, containing the underlying [log](Log).
#[derive(Debug, Clone)]
pub struct NewLog {
    pub log: Log,
}

impl<M> LogCollector<M> {
    pub fn new(provider: Arc<M>, filter: Filter) -> Self {
        Self { provider, filter }
    }
}

/// Implementation of the [Collector](Collector) trait for the [LogCollector](LogCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new logs.
#[async_trait]
impl<M> Collector<NewLog> for LogCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewLog>> {
        let stream = self.provider.subscribe_logs(&self.filter).await?;
        let stream = stream.filter_map(|log| {
            Some(NewLog { log })
        });
        Ok(Box::pin(stream))
    }
}
