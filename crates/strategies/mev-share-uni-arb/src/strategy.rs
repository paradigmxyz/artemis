use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;

use anyhow::Result;
use artemis_core::types::Strategy;

use ethers::signers::Signer;
use ethers::types::transaction::eip2718::TypedTransaction;
use matchmaker::types::{BundleRequest, BundleTx};

use ethers::providers::Middleware;
use ethers::types::{Filter, TransactionRequest, H256};
use ethers::types::{H160, U256};

use crate::constants::ARB_CONTRACT_ADDRESS;
use crate::types::{PoolRecord, V2V3PoolRecord};

use super::types::{Action, Event};

use mev_share_bindings::blind_arb::BlindArb;

#[derive(Debug, Clone)]
pub struct V2PoolInfo {
    pub v2_pool: H160,
    pub is_weth_token0: bool,
}

#[derive(Debug, Clone)]
pub struct MevShareUniArb<M, S> {
    /// Ethers client.
    client: Arc<M>,
    /// Maps Uni V3 pools to Uni V2 pool information.
    pool_map: HashMap<H160, V2PoolInfo>,
    /// Tx Signer
    tx_signer: S,
    /// Arb Contract
    arb_contract: BlindArb<M>,
}

impl<M: Middleware + 'static, S: Signer> MevShareUniArb<M, S> {
    pub fn new(client: Arc<M>, signer: S) -> Self {
        Self {
            client: client.clone(),
            pool_map: HashMap::new(),
            tx_signer: signer,
            arb_contract: BlindArb::new(*ARB_CONTRACT_ADDRESS, client.clone()),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static, S: Signer + 'static> Strategy<Event, Action>
    for MevShareUniArb<M, S>
{
    async fn sync_state(&mut self) -> Result<()> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/v3_v2_pools.csv");
        let mut reader = csv::Reader::from_path(path)?;

        for record in reader.deserialize() {
            // Check that the record exists, and deserialize it to the `Record` type
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

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::MEVShareEvent(event) => {
                println!("Got event: {:?}", event);
                // skip if event has no logs
                if event.logs.len() == 0 {
                    return None;
                }
                let address = event.logs[0].address;
                // skip if address is not a v3 pool
                if !self.pool_map.contains_key(&address) {
                    return None;
                }

                // generate bundles
                println!("FOUND A V3 MATCH");
                let bundles = self.generate_bundles(address, event.hash).await;
                return Some(Action::SubmitBundles(bundles));
            }
        }
    }
}

impl<M: Middleware + 'static, S: Signer + 'static> MevShareUniArb<M, S> {
    pub async fn generate_bundles(&self, v3_address: H160, tx_hash: H256) -> Vec<BundleRequest> {
        let mut bundles = Vec::new();
        let v2_info = self.pool_map.get(&v3_address).unwrap();

        // let sizes = vec![
        //     U256::from(1000000000000000000_u128),
        //     U256::from(2000000000000000000_u128),
        //     U256::from(3000000000000000000_u128),
        //     U256::from(4000000000000000000_u128),
        //     U256::from(5000000000000000000_u128),
        //     U256::from(6000000000000000000_u128),
        //     U256::from(7000000000000000000_u128),
        //     U256::from(8000000000000000000_u128),
        //     U256::from(9000000000000000000_u128),
        // ];

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

        // TODO: CHANGE THIS
        let payment_percentage = U256::from(0);

        let bid_gas_price = self.client.get_gas_price().await.unwrap();

        let block_num = self.client.get_block_number().await.unwrap();

        for size in sizes {
            let mut tx = {
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
                inner.set_gas(400000);
                inner.set_gas_price(bid_gas_price * 2);
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
            print!("TX: {:?}", tx);
            let signature = self.tx_signer.sign_transaction(&tx).await.unwrap();
            let bytes = tx.rlp_signed(&signature);
            let mut txs = Vec::new();
            txs.push(BundleTx::TxHash { hash: tx_hash });
            txs.push(BundleTx::Tx {
                tx: bytes,
                can_revert: false,
            });
            let bundle = BundleRequest::make_simple(block_num.add(1), txs);
            println!("BUNDLE: {:?}", bundle);
            bundles.push(bundle);
        }
        bundles
    }
}
