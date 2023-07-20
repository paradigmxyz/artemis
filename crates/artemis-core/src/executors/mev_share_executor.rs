use crate::types::Executor;
use anyhow::Result;
use async_trait::async_trait;
use ethers::{signers::Signer, types::Chain};
use futures::{stream, StreamExt};
use jsonrpsee::http_client::{transport::Error as HttpError, HttpClient, HttpClientBuilder};
use mev_share::rpc::{FlashbotsSignerLayer, MevApiClient, SendBundleRequest};
use tower::ServiceBuilder;
use tracing::{error, info};

/// An executor that sends bundles to the MEV-share Matchmaker.
pub struct MevshareExecutor {
    mev_share_client: AuthedClient,
}

// what type should this actually be?
type AuthedClient = HttpClient;

impl MevshareExecutor {
    pub fn new<S: Signer + Clone + 'static>(signer: S, chain: Chain) -> Self {
        // Set up flashbots-style auth middleware
        let signing_middleware = FlashbotsSignerLayer::new(signer);
        let service_builder = ServiceBuilder::new()
            // map signer errors to http errors
            .map_err(|e| HttpError::Http(e))
            .layer(signing_middleware);

        // Set up the rpc client
        let url = "https://relay.flashbots.net:443";
        let client = HttpClientBuilder::default()
            .set_middleware(service_builder)
            .build(url)
            .expect("Failed to create http client");

        Self {
            mev_share_client: client,
        }
    }
}

#[async_trait]
impl Executor<Vec<SendBundleRequest>> for MevshareExecutor {
    /// Send bundles to the matchmaker.
    async fn execute(&self, action: Vec<SendBundleRequest>) -> Result<()> {
        let bodies = stream::iter(action)
            .map(|bundle| {
                let client = &self.mev_share_client;
                async move { client.send_bundle(bundle).await }
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
