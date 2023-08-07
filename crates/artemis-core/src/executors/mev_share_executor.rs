use crate::types::Executor;
use anyhow::Result;
use async_trait::async_trait;
use ethers::signers::Signer;
use jsonrpsee::http_client::{
    transport::{self},
    HttpClientBuilder,
};
use mev_share::rpc::{FlashbotsSignerLayer, MevApiClient, SendBundleRequest};

use tracing::{error, info};

/// An executor that sends bundles to the MEV-share Matchmaker.
pub struct MevshareExecutor {
    mev_share_client: Box<dyn MevApiClient + Send + Sync>,
}

impl MevshareExecutor {
    pub fn new(signer: impl Signer + Clone + 'static) -> Self {
        // Set up flashbots-style auth middleware
        let http = HttpClientBuilder::default()
            .set_middleware(
                tower::ServiceBuilder::new()
                    .map_err(transport::Error::Http)
                    .layer(FlashbotsSignerLayer::new(signer)),
            )
            .build("https://relay.flashbots.net:443")
            .expect("failed to build HTTP client");
        Self {
            mev_share_client: Box::new(http),
        }
    }
}

#[async_trait]
impl Executor<SendBundleRequest> for MevshareExecutor {
    /// Send bundles to the matchmaker.
    async fn execute(&self, action: SendBundleRequest) -> Result<()> {
        let body = self.mev_share_client.send_bundle(action).await;
        match body {
            Ok(body) => info!("Bundle response: {:?}", body),
            Err(e) => error!("Bundle error: {}", e),
        };
        Ok(())
    }
}
