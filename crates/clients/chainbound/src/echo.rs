use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    providers::Middleware,
    signers::Signer,
    types::transaction::eip2718::TypedTransaction,
    types::{Bytes, TxHash},
};
use reqwest::{header::HeaderMap, Client};
use serde::Serialize;
use tracing::error;

use artemis_core::types::Executor;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Builder {
    Flashbots,
    Beaverbuild,
    Rsync,
    Builder0x69,
    Titan,
    F1b,
    Blocknative,
    Nfactorial,
    Buildai,

    /// Custom builder name (must be supported by the Echo RPC).
    /// This can be useful if a new Echo version comes out and this
    /// library has not been updated yet.
    Other(String),

    /// Use all available builders. This is the default behavior.
    #[default]
    All,
}

/// Complete bundle interface, including Echo-specific features.
/// See the full specs and their meaning here: <https://echo.chainbound.io/docs/usage/api-interface>
#[derive(Debug, Default)]
pub struct MevBundle {
    pub txs: Vec<TypedTransaction>,
    pub signed_txs: Vec<Bytes>,
    pub block_number: Option<u64>,
    pub min_timestamp: Option<u64>,
    pub max_timestamp: Option<u64>,
    pub reverting_tx_hashes: Option<Vec<TxHash>>,
    pub replacement_uuid: Option<String>,
    pub mev_builders: Option<Vec<Builder>>,
    pub use_public_mempool: Option<bool>,
    pub await_receipt: Option<bool>,
    pub await_receipt_timeout_ms: Option<u64>,
}

impl MevBundle {
    pub fn with_txs(txs: Vec<TypedTransaction>) -> Self {
        Self {
            txs,
            ..Default::default()
        }
    }

    pub fn add_signed_tx(&mut self, tx: Bytes) -> &mut Self {
        self.signed_txs.push(tx);
        self
    }

    pub fn set_block_number(&mut self, block_number: u64) -> &mut Self {
        self.block_number = Some(block_number);
        self
    }

    pub fn set_min_timestamp(&mut self, min_timestamp: u64) -> &mut Self {
        self.min_timestamp = Some(min_timestamp);
        self
    }

    pub fn set_max_timestamp(&mut self, max_timestamp: u64) -> &mut Self {
        self.max_timestamp = Some(max_timestamp);
        self
    }

    pub fn set_reverting_tx_hashes(&mut self, reverting_tx_hashes: Vec<TxHash>) -> &mut Self {
        self.reverting_tx_hashes = Some(reverting_tx_hashes);
        self
    }

    pub fn set_replacement_uuid(&mut self, replacement_uuid: String) -> &mut Self {
        self.replacement_uuid = Some(replacement_uuid);
        self
    }

    pub fn set_mev_builders(&mut self, mev_builders: Vec<Builder>) -> &mut Self {
        self.mev_builders = Some(mev_builders);
        self
    }

    pub fn set_use_public_mempool(&mut self, use_public_mempool: bool) -> &mut Self {
        self.use_public_mempool = Some(use_public_mempool);
        self
    }

    pub fn set_await_receipt(&mut self, await_receipt: bool) -> &mut Self {
        self.await_receipt = Some(await_receipt);
        self
    }

    pub fn set_await_receipt_timeout_ms(&mut self, await_receipt_timeout_ms: u64) -> &mut Self {
        self.await_receipt_timeout_ms = Some(await_receipt_timeout_ms);
        self
    }
}

/// An Echo executor that sends transactions to the specified block builders.
pub struct EchoExecutor<M, S> {
    /// The HTTP client to send requests to the Echo RPC
    echo_client: Client,
    /// The native ethers middleware
    inner: Arc<M>,
    /// The signer to sign transactions before sending to the builders
    tx_signer: S,
    /// the signer to compute the `X-Flashbots-Signature` of the bundle payload
    auth_signer: S,
    /// The list of builders to send the bundles to
    mev_builders: Vec<Builder>,
}

impl<M: Middleware, S: Signer> EchoExecutor<M, S> {
    pub fn new(
        inner: Arc<M>,
        tx_signer: S,
        auth_signer: S,
        api_key: impl Into<String>,
        mev_builders: Vec<Builder>,
    ) -> Self {
        let headers = HeaderMap::new();
        let mut echo_client = Client::builder()
            .timeout(Duration::from_secs(300))
            .default_headers(headers)
            .build()
            .expect("Could not instantiate HTTP client");

        Self {
            echo_client,
            inner,
            tx_signer,
            auth_signer,
            mev_builders,
        }
    }
}

#[async_trait]
impl<M, S> Executor<MevBundle> for EchoExecutor<M, S>
where
    M: Middleware + 'static,
    M::Error: 'static,
    S: Signer + 'static,
{
    /// Send a bundle to transactions to the specified builders.
    async fn execute(&self, mut action: MevBundle) -> Result<()> {
        // Sign each transaction in bundle.
        for tx in action.txs {
            let signature = self.tx_signer.sign_transaction(&tx).await?;
            action.add_signed_tx(tx.rlp_signed(&signature));
        }

        // // Simulate bundle.
        // let block_number = self.echo_client.get_block_number().await?;
        // let bundle = bundle
        //     .set_block(block_number + 1)
        //     .set_simulation_block(block_number)
        //     .set_simulation_timestamp(0);
        // let simulated_bundle = self.echo_client.simulate_bundle(&bundle).await;
        // if let Err(simulate_error) = simulated_bundle {
        //     error!("Error simulating bundle: {:?}", simulate_error);
        // }

        // Send bundle.
        // TODO
        let pending_bundle = self.echo_client.send_bundle(&bundle).await;

        if let Err(send_error) = pending_bundle {
            error!("Error sending bundle: {:?}", send_error);
        }

        Ok(())
    }
}
