use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::PubsubClient,
    types::{H256, U64, U256, Bytes},
};
use std::sync::Arc;
use tokio_stream::StreamExt;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: H256,
    pub number: U64,
    pub gas_used: U256,
    pub timestamp: U256,
    pub extra_data: Bytes,
    pub base_fee_per_gas: U256
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
        let stream = stream.filter_map(|block| match block.hash {
            Some(hash) => {
                let gas_used = block.gas_used;
                let timestamp = block.timestamp;
                let extra_data = block.extra_data;
                let base_fee_per_gas = match block.base_fee_per_gas { Some(base_fee_per_gas) => base_fee_per_gas, None => U256::from(0)};

                block.number.map(|number| NewBlock { hash, number, gas_used, timestamp, extra_data, base_fee_per_gas})
            },
            None => None,
        });
        Ok(Box::pin(stream))
    }
}
