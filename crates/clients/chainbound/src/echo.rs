use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::{providers::Middleware, signers::Signer};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use tracing::{debug, error};

use artemis_core::types::Executor;

use crate::SendBundleArgs;

/// Possible actions that can be executed by the Echo executor
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum Action {
    SendBundle(SendBundleArgs),
}

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

    /// Returns a reference to the native ethers middleware
    pub fn provider(&self) -> Arc<M> {
        self.inner.clone()
    }
}

#[async_trait]
impl<M, S> Executor<SendBundleArgs> for EchoExecutor<M, S>
where
    M: Middleware + 'static,
    M::Error: 'static,
    S: Signer + 'static,
{
    /// Send a bundle to transactions to the specified builders
    async fn execute(&self, mut action: SendBundleArgs) -> Result<()> {
        if action.unsigned_txs.is_empty() {
            return Err(anyhow!(
                "Bundle must contain at least one transaction. 
                To cancel a bundle, use the `eth_cancelBundle` method."
            ));
        }

        // Sign each transaction in bundle
        for tx in action.unsigned_txs.iter() {
            let signature = self.tx_signer.sign_transaction(&tx.clone().into()).await?;
            let signed = tx.rlp_signed(&signature).to_string();
            action.standard_features.txs.push(signed);
        }

        // Set block number to the next block if not specified
        if action.standard_features.block_number.is_none() {
            let block_number = self.inner.get_block_number().await?;
            let next_block_number_hex = format!("0x{:#x}", block_number.as_u64() + 1);
            action.standard_features.block_number = Some(next_block_number_hex);
        }

        // TODO: Simulate bundle

        // Sign bundle payload (without the Echo-specific features)
        let signable_payload = serde_json::to_string(&action.standard_features)?;
        let flashbots_signature = self.auth_signer.sign_message(&signable_payload).await?;

        // Create the `X-Flashbots-Signature` header
        let flashbots_signature_header: HeaderValue =
            format!("{:#x}:{}", self.auth_signer.address(), flashbots_signature).parse()?;

        // Prepare the full JSON-RPC request body
        let bundle_json = serde_json::to_string(&action)?;

        let request_body = format!(
            r#"{{"id":1,"jsonrpc":"2.0","method":"eth_sendBundle","params":[{}]}}"#,
            bundle_json
        );

        // Send bundle
        let echo_response = self
            .echo_client
            .post(&self.echo_endpoint)
            .body(request_body)
            .header("X-Flashbots-Signature", flashbots_signature_header)
            .send()
            .await;

        match echo_response {
            Ok(send_response) => {
                let status = send_response.status();
                let body = send_response.text().await?;

                dbg!(body.clone());

                if status.is_success() {
                    debug!("Echo bundle response: {:?}", body);
                } else {
                    error!("Error in Echo bundle response: {:?}", body);
                }
            }
            Err(send_error) => error!("Error while sending bundle to Echo: {:?}", send_error),
        }

        Ok(())
    }
}
