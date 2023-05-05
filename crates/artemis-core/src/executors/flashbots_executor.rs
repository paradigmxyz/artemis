use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    providers::Middleware, signers::Signer, types::transaction::eip2718::TypedTransaction,
};
use ethers_flashbots::{BundleRequest, FlashbotsMiddleware};
use reqwest::Url;
use tracing::error;

use crate::types::Executor;

/// A Flashbots executor that sends transactions to the Flashbots relay.
pub struct FlashbotsExecutor<M, S> {
    /// The Flashbots middleware.
    fb_client: FlashbotsMiddleware<Arc<M>, S>,

    /// The signer to sign transactions before sending to the relay.
    tx_signer: S,
}

/// A bundle of transactions to send to the Flashbots relay.
pub type FlashbotsBundle = Vec<TypedTransaction>;

impl<M: Middleware, S: Signer> FlashbotsExecutor<M, S> {
    pub fn new(client: Arc<M>, tx_signer: S, relay_signer: S, relay_url: impl Into<Url>) -> Self {
        let fb_client = FlashbotsMiddleware::new(client, relay_url, relay_signer);
        Self {
            fb_client,
            tx_signer,
        }
    }
}

#[async_trait]
impl<M, S> Executor<FlashbotsBundle> for FlashbotsExecutor<M, S>
where
    M: Middleware + 'static,
    M::Error: 'static,
    S: Signer + 'static,
{
    /// Send a bundle to transactions to the Flashbots relay.
    async fn execute(&self, action: FlashbotsBundle) -> Result<()> {
        // Add txs to bundle.
        let mut bundle = BundleRequest::new();

        // Sign each transaction in bundle.
        for tx in action {
            let signature = self.tx_signer.sign_transaction(&tx).await?;
            bundle.add_transaction(tx.rlp_signed(&signature));
        }

        // Simulate bundle.
        let block_number = self.fb_client.get_block_number().await?;
        let bundle = bundle
            .set_block(block_number + 1)
            .set_simulation_block(block_number)
            .set_simulation_timestamp(0);

        let simulated_bundle = self.fb_client.simulate_bundle(&bundle).await;

        if let Err(simulate_error) = simulated_bundle {
            error!("Error simulating bundle: {:?}", simulate_error);
        }

        // Send bundle.
        let pending_bundle = self.fb_client.send_bundle(&bundle).await;

        if let Err(send_error) = pending_bundle {
            error!("Error sending bundle: {:?}", send_error);
        }

        Ok(())
    }
}
