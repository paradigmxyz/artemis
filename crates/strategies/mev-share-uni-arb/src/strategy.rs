use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;

use anyhow::Result;
use artemis_core::types::Strategy;

use ethers::signers::Signer;

use ethers::providers::Middleware;
use ethers::types::{Address, H256};
use ethers::types::{H160, U256};
use mev_share::rpc::{BundleItem, Inclusion, SendBundleRequest};
use tracing::info;

use crate::types::V2V3PoolRecord;

use super::types::{Action, Event};

use mev_share_bindings::blind_arb::BlindArb;

/// Information about a uniswap v2 pool.
#[derive(Debug, Clone)]
pub struct V2PoolInfo {
    /// Address of the v2 pool.
    pub v2_pool: H160,
    /// Whether the pool has weth as token0.
    pub is_weth_token0: bool,
}

#[derive(Debug, Clone)]
pub struct MevShareUniArb<M, S> {
    /// Ethers client.
    client: Arc<M>,
    /// Maps uni v3 pool address to v2 pool information.
    pool_map: HashMap<H160, V2PoolInfo>,
    /// Signer for transactions.
    tx_signer: S,
    /// Arb contract.
    arb_contract: BlindArb<M>,
}

impl<M: Middleware + 'static, S: Signer> MevShareUniArb<M, S> {
    /// Create a new instance of the strategy.
    pub fn new(client: Arc<M>, signer: S, arb_contract_address: Address) -> Self {
        Self {
            client: client.clone(),
            pool_map: HashMap::new(),
            tx_signer: signer,
            arb_contract: BlindArb::new(arb_contract_address, client),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static, S: Signer + 'static> Strategy<Event, Action>
    for MevShareUniArb<M, S>
{
    /// Initialize the strategy. This is called once at startup, and loads
    /// pool information into memory.
    async fn sync_state(&mut self) -> Result<()> {
        // Read pool information from csv file.
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/v3_v2_pools.csv");
        let mut reader = csv::Reader::from_path(path)?;

        for record in reader.deserialize() {
            // Parse records into PoolRecord struct.
            let record: V2V3PoolRecord = record?;
            self.pool_map.insert(
                record.v3_pool,
                V2PoolInfo {
                    v2_pool: record.v2_pool,
                    is_weth_token0: record.weth_token0,
                },
            );
        }

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::MEVShareEvent(event) => {
                info!("Received mev share event: {:?}", event);
                // skip if event has no logs
                if event.logs.is_empty() {
                    return vec![];
                }
                let address = event.logs[0].address;
                // skip if address is not a v3 pool
                if !self.pool_map.contains_key(&address) {
                    return vec![];
                }
                // if it's a v3 pool we care about, submit bundles
                info!(
                    "Found a v3 pool match at address {:?}, submitting bundles",
                    address
                );
                self.generate_bundles(address, event.hash)
                    .await
                    .into_iter()
                    .map(Action::SubmitBundle)
                    .collect()
            }
        }
    }
}

impl<M: Middleware + 'static, S: Signer + 'static> MevShareUniArb<M, S> {
    /// Generate a series of bundles of varying sizes to submit to the matchmaker.
    pub async fn generate_bundles(
        &self,
        v3_address: H160,
        tx_hash: H256,
    ) -> Vec<SendBundleRequest> {
        let mut bundles = Vec::new();
        let v2_info = self.pool_map.get(&v3_address).unwrap();

        // The sizes of the backruns we want to submit.
        // TODO: Run some analysis to figure out likely sizes.
        let sizes = vec![
            U256::from(100000_u128),
            U256::from(1000000_u128),
            U256::from(10000000_u128),
            U256::from(100000000_u128),
            U256::from(1000000000_u128),
            U256::from(10000000000_u128),
            U256::from(100000000000_u128),
            U256::from(1000000000000_u128),
            U256::from(10000000000000_u128),
            U256::from(100000000000000_u128),
            U256::from(1000000000000000_u128),
            U256::from(10000000000000000_u128),
            U256::from(100000000000000000_u128),
            U256::from(1000000000000000000_u128),
        ];

        // Set parameters for the backruns.
        let payment_percentage = U256::from(0);
        let bid_gas_price = self.client.get_gas_price().await.unwrap();
        let block_num = self.client.get_block_number().await.unwrap();

        for size in sizes {
            let arb_tx = {
                // Construct arb tx based on whether the v2 pool has weth as token0.
                let mut inner = match v2_info.is_weth_token0 {
                    true => {
                        self.arb_contract
                            .execute_arb_weth_token_0(
                                v2_info.v2_pool,
                                v3_address,
                                size,
                                payment_percentage,
                            )
                            .tx
                    }
                    false => {
                        self.arb_contract
                            .execute_arb_weth_token_1(
                                v2_info.v2_pool,
                                v3_address,
                                size,
                                payment_percentage,
                            )
                            .tx
                    }
                };
                // Set gas parameters (this is a bit hacky)
                inner.set_gas(400000);
                inner.set_gas_price(bid_gas_price);
                let fill = self.client.fill_transaction(&mut inner, None).await;

                match fill {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error filling tx: {}", e);
                        continue;
                    }
                }

                inner
            };
            info!("generated arb tx: {:?}", arb_tx);

            // Sign tx and construct bundle
            let signature = self.tx_signer.sign_transaction(&arb_tx).await.unwrap();
            let bytes = arb_tx.rlp_signed(&signature);
            let txs = vec![
                BundleItem::Hash { hash: tx_hash },
                BundleItem::Tx {
                    tx: bytes,
                    can_revert: false,
                },
            ];
            let bundle = SendBundleRequest {
                bundle_body: txs,
                inclusion: Inclusion {
                    block: block_num.add(1),
                    // set a large validity window to ensure builder gets a chance to include bundle.
                    max_block: Some(block_num.add(30)),
                },
                ..Default::default()
            };
            info!("submitting bundle: {:?}", bundle);
            bundles.push(bundle);
        }
        bundles
    }
}
