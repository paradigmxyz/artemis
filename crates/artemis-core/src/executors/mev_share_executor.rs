use std::error::Error;

use crate::types::Executor;
use anyhow::Result;
use async_trait::async_trait;
use ethers::{signers::Signer};
use futures::{stream, StreamExt};
use jsonrpsee::http_client::{
    transport::{Error as HttpError, HttpBackend},
    HttpClient, HttpClientBuilder,
};
use mev_share::rpc::{FlashbotsSigner, FlashbotsSignerLayer, MevApiClient, SendBundleRequest};
use tower::{util::MapErr, ServiceBuilder};
use tracing::{error, info};

/// An executor that sends bundles to the MEV-share Matchmaker.
pub struct MevshareExecutor<S> {
    mev_share_client: AuthedClient<S>,
}

type MapErrorFn = fn(Box<dyn Error + Send + Sync + 'static>) -> HttpError;
type AuthedClient<S> = HttpClient<MapErr<FlashbotsSigner<S, HttpBackend>, MapErrorFn>>;

impl<S: Signer + Clone + 'static> MevshareExecutor<S> {
    pub fn new(signer: S) -> Self {
        // Set up flashbots-style auth middleware
        let signing_middleware = FlashbotsSignerLayer::new(signer);
        let service_builder = ServiceBuilder::new()
            // map signer errors to http errors
            .map_err(Self::map_box_to_http_error as MapErrorFn)
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

    pub fn map_box_to_http_error(err: Box<dyn Error + Send + Sync>) -> HttpError {
        HttpError::Http(err)
    }
}

#[async_trait]
impl<S: Signer + Clone + 'static> Executor<Vec<SendBundleRequest>> for MevshareExecutor<S> {
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
