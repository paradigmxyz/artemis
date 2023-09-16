use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::{providers::Middleware, signers::Signer};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use tracing::{error, info};

use artemis_core::types::Executor;

use crate::MevBundle;

const ECHO_RPC_URL: &str = "https://echo-rpc.chainbound.io";

/// An Echo executor that sends transactions to the specified block builders
pub struct EchoExecutor<M, S> {
    /// The Echo RPC endpoint
    echo_endpoint: String,
    /// The HTTP client to send requests to the Echo RPC
    echo_client: Client,
    /// The native ethers middleware
    inner: Arc<M>,
    /// The signer to sign transactions before sending to the builders
    tx_signer: S,
    /// the signer to compute the `X-Flashbots-Signature` of the bundle payload
    auth_signer: S,
}

impl<M: Middleware, S: Signer> EchoExecutor<M, S> {
    /// Initialize a new Echo executor.
    ///
    /// ## Arguments
    /// - `inner`: The native ethers middleware that can query the blockchain
    /// - `tx_signer`: The actual signer of the bundle transactions
    /// - `auth_signer`: The signer to compute the `X-Flashbots-Signature` of the bundle payload
    /// - `api_key`: The Echo API key to use
    pub fn new(inner: Arc<M>, tx_signer: S, auth_signer: S, api_key: impl Into<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("X-Api-Key", api_key.into().parse().expect("Broken API key"));

        let echo_client = Client::builder()
            .timeout(Duration::from_secs(300))
            .default_headers(headers)
            .build()
            .expect("Could not instantiate HTTP client");

        Self {
            echo_endpoint: ECHO_RPC_URL.into(),
            echo_client,
            inner,
            tx_signer,
            auth_signer,
        }
    }

    /// Optionally set the Echo RPC endpoint, overriding the default
    pub fn set_rpc_endpoint(&mut self, endpoint: impl Into<String>) {
        self.echo_endpoint = endpoint.into();
    }
}

#[async_trait]
impl<M, S> Executor<MevBundle> for EchoExecutor<M, S>
where
    M: Middleware + 'static,
    M::Error: 'static,
    S: Signer + 'static,
{
    /// Send a bundle to transactions to the specified builders
    async fn execute(&self, mut action: MevBundle) -> Result<()> {
        if action.txs.is_empty() {
            return Err(anyhow!(
                "Bundle must contain at least one transaction. 
                To cancel a bundle, use the `eth_cancelBundle` method."
            ));
        }

        // Sign each transaction in bundle
        for tx in &action.txs.clone() {
            let signature = self.tx_signer.sign_transaction(tx).await?;
            action.add_signed_tx(tx.rlp_signed(&signature));
        }

        // Set block number to the next block if not specified
        if action.block_number.is_none() {
            let block_number = self.inner.get_block_number().await?;
            action.set_block_number(block_number.as_u64() + 1);
        }

        // TODO: Simulate bundle

        // Sign bundle payload (without the Echo-specific features)
        let signable_payload = action.format_json_rpc_request("eth_sendBundle", false);
        let flashbots_signature = self.auth_signer.sign_message(&signable_payload).await?;

        // Create the `X-Flashbots-Signature` header
        let flashbots_signature_header: HeaderValue =
            format!("{}:{}", self.auth_signer.address(), flashbots_signature).parse()?;

        // Prepare the full JSON-RPC request body
        let request_body = action.format_json_rpc_request("eth_sendBundle", true);

        // Send bundle
        let echo_response = self
            .echo_client
            .post(ECHO_RPC_URL)
            .body(request_body)
            .header("X-Flashbots-Signature", flashbots_signature_header)
            .send()
            .await;

        match echo_response {
            Ok(send_response) => {
                let status = send_response.status();
                let body = send_response.text().await?;

                if status.is_success() {
                    info!("Echo bundle response: {:?}", body);
                } else {
                    error!("Error in Echo bundle response: {:?}", body);
                }
            }
            Err(send_error) => error!("Error while sending bundle to Echo: {:?}", send_error),
        }

        Ok(())
    }
}
