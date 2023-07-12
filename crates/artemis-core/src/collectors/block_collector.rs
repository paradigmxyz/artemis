use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::PubsubClient,
    types::{Block, TxHash},
};
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the entire block
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub block: Block<TxHash>,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let stream = self.provider.subscribe_blocks().await?;
        let stream = stream.filter_map(|block| Some(NewBlock { block }));
        Ok(Box::pin(stream))
    }
}
