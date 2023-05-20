use async_trait::async_trait;

use ethers::{prelude::Middleware, providers::PubsubClient, types::Transaction};
use futures::StreamExt;
use std::sync::Arc;

use crate::types::{Collector, CollectorStream};
use anyhow::Result;

/// A collector that listens for new transactions in the mempool, and generates a stream of
/// [events](Transaction) which contain the transaction.
pub struct MempoolCollector<M> {
    provider: Arc<M>,
}

impl<M> MempoolCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [MempoolCollector](MempoolCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new transactions.
#[async_trait]
impl<M> Collector<Transaction> for MempoolCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Transaction>> {
        let stream = self.provider.subscribe_pending_txs().await?;
        let stream = stream.transactions_unordered(256);
        let stream = stream.filter_map(|res| async move { res.ok() });
        Ok(Box::pin(stream))
    }
}
