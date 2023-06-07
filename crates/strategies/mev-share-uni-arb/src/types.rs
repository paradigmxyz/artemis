use artemis_core::{
    collectors::{block_collector::NewBlock, opensea_order_collector::OpenseaOrder},
    executors::{mempool_executor::SubmitTxToMempool, mev_share_executor::Bundles},
};
use ethers::types::{Chain, H160, H256};

use mev_share_rs::{sse, EventClient};

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    MEVShareEvent(sse::Event),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitBundles(Bundles),
}

#[derive(Debug, serde::Deserialize)]
pub struct PoolRecord {
    pub token_address: H160,
    pub uni_pool_address: H160,
    pub sushi_pool_address: H160,
}

#[derive(Debug, serde::Deserialize)]
pub struct V2V3PoolRecord {
    pub token_address: H160,
    pub v3_pool: H160,
    pub v2_pool: H160,
    pub weth_token0: bool,
}
