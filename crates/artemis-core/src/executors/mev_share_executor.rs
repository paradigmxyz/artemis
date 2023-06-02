use crate::types::Executor;
use anyhow::Result;
use async_trait::async_trait;
use ethers::{signers::Signer, types::Chain};
use futures::{stream, StreamExt};
use matchmaker::{client::Client, types::BundleRequest};
use tracing::{error, info};

/// An executor that sends bundles to the MEV-share Matchmaker.
pub struct MevshareExecutor<S> {
    matchmaker_client: Client<S>,
}

/// List of bundles to send to the Matchmaker.
pub type Bundles = Vec<BundleRequest>;

impl<S: Signer + Clone + 'static> MevshareExecutor<S> {
    pub fn new(signer: S, chain: Chain) -> Self {
        Self {
            matchmaker_client: Client::new(signer, chain),
        }
    }
}

#[async_trait]
impl<S: Signer + Clone + 'static> Executor<Bundles> for MevshareExecutor<S> {
    /// Send bundles to the matchmaker.
    async fn execute(&self, action: Bundles) -> Result<()> {
        let bodies = stream::iter(action)
            .map(|bundle| {
                let client = &self.matchmaker_client;
                async move { client.send_bundle(&bundle).await }
            })
            .buffer_unordered(5);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => info!("Bundle response: {:?}", b),
                    Err(e) => error!("Bundle error: {}", e),
                }
            })
            .await;
        Ok(())
    }
}
