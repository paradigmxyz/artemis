use ethers::types::H160;

use mev_share::{rpc::SendBundleRequest, sse};

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    MEVShareEvent(sse::Event),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitBundle(SendBundleRequest),
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
