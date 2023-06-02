use std::sync::Arc;

use ethers::{signers::Signer, types::Chain};

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Error as RpcError;
use jsonrpsee::http_client::{transport::HttpBackend, HttpClient, HttpClientBuilder};

use tower::ServiceBuilder;

use crate::{
    flashbots_signer::{FlashbotsSigner, FlashbotsSignerLayer},
    types::{BundleRequest, SendBundleResponse},
};

/// Matchmaker client to interact with MEV-share
pub struct Client<S> {
    /// Underlying HTTP client
    pub http_client: HttpClient<FlashbotsSigner<S, HttpBackend>>,
}

impl<S: Signer + Clone + 'static> Client<S> {
    /// Create a new client with the given signer and chain
    pub fn new(signer: S, chain: Chain) -> Self {
        let url = match chain {
            Chain::Mainnet => "https://relay.flashbots.net:443",
            Chain::Goerli => "https://relay-goerli.flashbots.net:443",
            _ => panic!("Unsupported chain"),
        };
        Self::from_url(signer, url)
    }

    /// Create a new client with the given signer and url
    pub fn from_url(signer: S, url: &str) -> Self {
        let signing_middleware = FlashbotsSignerLayer::new(Arc::new(signer));

        let service_builder = ServiceBuilder::new().layer(signing_middleware);

        let http_client = HttpClientBuilder::default()
            .set_middleware(service_builder)
            .build(url)
            .unwrap();

        Self { http_client }
    }

    /// Send a bundle to the matchmaker
    pub async fn send_bundle(
        &self,
        bundle: &BundleRequest,
    ) -> Result<SendBundleResponse, RpcError> {
        self.http_client.request("mev_sendBundle", [bundle]).await
    }
}
