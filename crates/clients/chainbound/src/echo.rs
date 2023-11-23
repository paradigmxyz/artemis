use std::{borrow::BorrowMut, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers::{providers::Middleware, signers::Signer};
use futures::{SinkExt, StreamExt, TryFutureExt, TryStreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error};

use artemis_core::types::Executor;

use crate::{utils, SendBundleArgs, SendPrivateTransactionArgs};

/// Possible actions that can be executed by the Echo executor
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum Action {
    SendBundle(SendBundleArgs),
    SendPrivateTransaction(SendPrivateTransactionArgs),
}

const ECHO_RPC_URL_WS: &str = "wss://echo-rpc.chainbound.io/ws";

/// An Echo executor that sends transactions to the specified block builders
pub struct EchoExecutor<M, S> {
    /// The Echo RPC endpoint
    echo_endpoint: String,
    /// The native ethers middleware
    inner: Arc<M>,
    /// The signer to sign transactions before sending to the builders
    tx_signer: S,
    /// the signer to compute the `X-Flashbots-Signature` of the bundle payload
    auth_signer: S,
    /// Channel to send websocket messages
    api_requests_tx: mpsc::Sender<String>,
    /// Channel to receive websocket messages
    pub api_responses_rx: mpsc::Receiver<String>,
}

impl<M: Middleware, S: Signer> EchoExecutor<M, S> {
    /// Initialize a new Echo executor.
    ///
    /// ## Arguments
    /// - `inner`: The native ethers middleware that can query the blockchain
    /// - `tx_signer`: The actual signer of the bundle transactions
    /// - `auth_signer`: The signer to compute the `X-Flashbots-Signature` of the bundle payload
    /// - `api_key`: The Echo API key to use
    pub async fn new(
        inner: Arc<M>,
        tx_signer: S,
        auth_signer: S,
        api_key: impl Into<String>,
    ) -> Self {
        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(ECHO_RPC_URL_WS)
            .header("x-api-key", api_key.into())
            .header("host", "echo-artemis-client")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
            .header("sec-websocket-version", "13")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .body(())
            .unwrap();

        let (ws_client, _) = tokio_tungstenite::connect_async(request)
            .await
            .expect("Failed to connect to Echo via Websocket.");
        debug!("Echo websocket handshake succeeded.");

        let (api_requests_tx, mut api_requests_rx) = mpsc::channel(1024);
        let (api_responses_tx, api_responses_rx) = mpsc::channel(1024);

        // Spawn a task to manage the websocket connection with request and response channels
        tokio::spawn(async move {
            let (mut outgoing, incoming) = ws_client.split();

            // send all incoming messages to the responses channel in a separate task
            tokio::spawn(async move {
                let responses_tx = api_responses_tx.clone();
                incoming.try_for_each(|msg| {
                    let text = msg.into_text().unwrap_or_else(|e| {
                        error!(error = ?e, "Error converting Echo API response to text");
                        Default::default()
                    });

                    responses_tx.send(text).map_err(|e| {
                        error!(error = ?e, "Error sending Echo API response to the responses channel");
                        tokio_tungstenite::tungstenite::error::Error::ConnectionClosed
                    })
                }).await.ok();
            });

            // send all messages from the intake channel into the websocket
            while let Some(msg) = api_requests_rx.recv().await {
                if let Err(e) = outgoing.send(Message::Text(msg)).await {
                    error!(error = ?e, "Error sending Echo API request to the websocket")
                }
            }

            // websocket has stopped sending messages, alert the user and exit
            error!("Echo API request channel has stopped sending messages");
        });

        Self {
            echo_endpoint: ECHO_RPC_URL_WS.into(),
            inner,
            tx_signer,
            auth_signer,
            api_requests_tx,
            api_responses_rx,
        }
    }

    /// Optionally set the Echo RPC endpoint, overriding the default
    pub fn set_rpc_endpoint(&mut self, endpoint: impl Into<String>) {
        self.echo_endpoint = endpoint.into();
    }

    /// Returns a reference to the native ethers middleware
    pub fn provider(&self) -> Arc<M> {
        Arc::clone(&self.inner)
    }

    /// Returns a reference to the API receipts channel
    pub fn receipts_channel(&mut self) -> &mut mpsc::Receiver<String> {
        self.api_responses_rx.borrow_mut()
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
        let method = "eth_sendBundle";
        let signable_payload = utils::generate_jsonrpc_request(method, &action.standard_features);
        let fb_signature = utils::generate_fb_signature(&self.auth_signer, &signable_payload).await;

        // Websocket usage format:
        // https://echo.chainbound.io/docs/usage/api-interface#flashbots-authentication
        let request_body = format!(
            r#"{{"x-flashbots-signature": "{}","payload": {}}}"#,
            fb_signature,
            utils::generate_jsonrpc_request(method, action)
        );

        // Send bundle request
        self.api_requests_tx.send(request_body).await?;
        debug!("bundle sent to Echo.");

        Ok(())
    }
}

#[async_trait]
impl<M, S> Executor<SendPrivateTransactionArgs> for EchoExecutor<M, S>
where
    M: Middleware + 'static,
    M::Error: 'static,
    S: Signer + 'static,
{
    /// Send a transaction to the specified builders
    async fn execute(&self, mut action: SendPrivateTransactionArgs) -> Result<()> {
        let Some(tx) = action.unsigned_tx.take() else {
            return Err(anyhow!(
                "SendPrivateTransactionArgs must contain a transaction. 
                You can set one with the `SendPrivateTransactionArgs::with_tx()` method."
            ));
        };

        // Sign the transaction
        let signature = self.tx_signer.sign_transaction(&tx.clone().into()).await?;
        let signed = tx.rlp_signed(&signature).to_string();
        action.standard_features.tx = signed;

        // TODO: Simulate transaction

        // Sign payload (without the Echo-specific features)
        let method = "eth_sendPrivateRawTransaction";
        let signable_payload = utils::generate_jsonrpc_request(method, &action.standard_features);
        let fb_signature = utils::generate_fb_signature(&self.auth_signer, &signable_payload).await;

        // Websocket usage format:
        // https://echo.chainbound.io/docs/usage/api-interface#flashbots-authentication
        let request_body = format!(
            r#"{{"x-flashbots-signature": "{}","payload": {}}}"#,
            fb_signature,
            utils::generate_jsonrpc_request(method, action)
        );

        // Send transaction request
        self.api_requests_tx.send(request_body).await?;
        debug!("transaction sent to Echo.");

        Ok(())
    }
}
