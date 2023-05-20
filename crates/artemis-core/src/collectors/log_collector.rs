use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::PubsubClient,
    types::{Bytes, Filter, H256},
};
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blockchain event logs based on a [Filter](Filter),
/// and generates a stream of [events](NewLog) which contain the indexed topics and data.
pub struct LogCollector<M> {
    provider: Arc<M>,
    filter: Filter,
}

/// A new log event, containing up to four indexed topics and the data.
#[derive(Debug, Clone)]
pub struct NewLog {
    pub topic0: H256,
    pub topic1: Option<H256>,
    pub topic2: Option<H256>,
    pub topic3: Option<H256>,
    pub data: Bytes,
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
            let topic0 = log.topics.get(0).copied();
            let topic1 = log.topics.get(1).copied();
            let topic2 = log.topics.get(2).copied();
            let topic3 = log.topics.get(3).copied();
            let data = log.data;
            match (topic0, topic1, topic2, topic3) {
                (Some(topic0), Some(topic1), Some(topic2), Some(topic3)) => Some(NewLog {
                    topic0,
                    topic1: Some(topic1),
                    topic2: Some(topic2),
                    topic3: Some(topic3),
                    data,
                }),
                (Some(topic0), Some(topic1), Some(topic2), None) => Some(NewLog {
                    topic0,
                    topic1: Some(topic1),
                    topic2: Some(topic2),
                    topic3: None,
                    data,
                }),
                (Some(topic0), Some(topic1), None, None) => Some(NewLog {
                    topic0,
                    topic1: Some(topic1),
                    topic2: None,
                    topic3: None,
                    data,
                }),
                (Some(topic0), None, None, None) => Some(NewLog {
                    topic0,
                    topic1: None,
                    topic2: None,
                    topic3: None,
                    data,
                }),
                _ => None,
            }
        });
        Ok(Box::pin(stream))
    }
}
