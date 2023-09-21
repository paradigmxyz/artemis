use ethers::types::TransactionRequest;
use serde::{Deserialize, Serialize};

/// An UUIDv4 identifier, useful for cancelling/replacing bundles.
pub type ReplacementUuid = String;

/// The list of available MEV builders.
#[derive(Debug, Default, Clone)]
pub enum BlockBuilder {
    /// RPC URL: <https://relay.flashbots.net>
    Flashbots,
    /// RPC URL: <https://rpc.beaverbuild.org/>
    Beaverbuild,
    /// RPC URL: <https://rsync-builder.xyz/>
    Rsync,
    /// RPC URL: <https://builder0x69.io>
    Builder0x69,
    /// RPC URL: <https://rpc.titanbuilder.xyz>
    Titan,
    /// RPC URL: <https://rpc.f1b.io>
    F1b,
    /// RPC URL: <https://api.blocknative.com/v1/auction>
    Blocknative,
    /// RPC URL: <https://rpc.nfactorial.xyz/>
    Nfactorial,
    /// RPC URL: <https://buildai.net/>
    Buildai,

    /// Custom builder name (must be supported by the Echo RPC).
    /// This can be useful if a new Echo version comes out and this
    /// library has not been updated yet.
    Other(String),

    /// Use all available builders. This is the default behavior.
    #[default]
    All,
}

impl ToString for BlockBuilder {
    fn to_string(&self) -> String {
        match self {
            BlockBuilder::Flashbots => "flashbots".to_string(),
            BlockBuilder::Beaverbuild => "beaverbuild".to_string(),
            BlockBuilder::Rsync => "rsync".to_string(),
            BlockBuilder::Builder0x69 => "builder0x69".to_string(),
            BlockBuilder::Titan => "titan".to_string(),
            BlockBuilder::F1b => "f1b".to_string(),
            BlockBuilder::Blocknative => "blocknative".to_string(),
            BlockBuilder::Nfactorial => "nfactorial".to_string(),
            BlockBuilder::Buildai => "buildai".to_string(),
            BlockBuilder::Other(name) => name.to_string(),
            BlockBuilder::All => "all".to_string(),
        }
    }
}

/// A request to send a bundle to the Echo RPC `eth_sendBundle` endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleArgs {
    /// (Internal) Bundle transactions that have yet to be signed.
    /// These are not sent to block builders. they will be replaced by the "txs" field
    /// inside the `standard_features` struct.
    #[serde(skip_serializing, skip_deserializing)]
    pub unsigned_txs: Vec<TransactionRequest>,

    /// Standard bundle features include the basic interface that all builders support.
    #[serde(flatten)]
    pub standard_features: StandardBundleFeatures,

    /// Echo-specific features and bundle options. These are not sent to block builders.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub echo_features: Option<EchoBundleFeatures>,
}

impl SendBundleArgs {
    /// Create a new `SendBundleArgs` with the specified unsigned transactions.
    pub fn with_txs(txs: Vec<TransactionRequest>) -> Self {
        Self {
            unsigned_txs: txs,
            ..Default::default()
        }
    }

    /// Add a transaction to the bundle.
    pub fn add_tx(&mut self, tx: TransactionRequest) {
        self.unsigned_txs.push(tx);
    }

    /// Set the block number at which the bundle should be mined.
    pub fn set_block_number(&mut self, block_number: u64) {
        self.standard_features.block_number = Some(format!("{:#x}", block_number));
    }

    /// Set the minimum timestamp at which the bundle should be mined
    pub fn set_min_timestamp(&mut self, min_timestamp: u64) {
        self.standard_features.min_timestamp = Some(min_timestamp);
    }

    /// Set the maximum timestamp at which the bundle should be mined
    pub fn set_max_timestamp(&mut self, max_timestamp: u64) {
        self.standard_features.max_timestamp = Some(max_timestamp);
    }

    /// Set the transaction hashes of transactions that can revert in the bundle,
    /// without which the rest of the bundle can still be included.
    pub fn set_reverting_tx_hashes(&mut self, reverting_tx_hashes: Vec<String>) {
        self.standard_features.reverting_tx_hashes = Some(reverting_tx_hashes);
    }

    /// Set the UUID of the bundle for later cancellation/replacement.
    pub fn set_replacement_uuid(&mut self, replacement_uuid: ReplacementUuid) {
        self.standard_features.replacement_uuid = Some(replacement_uuid);
    }

    /// Set the percentage of the gas that should be refunded.
    pub fn set_refund_percent(&mut self, refund_percent: u64) {
        self.standard_features.refund_percent = Some(refund_percent);
    }

    /// Set the address to which the refund should be sent.
    pub fn set_refund_recipient(&mut self, refund_recipient: String) {
        self.standard_features.refund_recipient = Some(refund_recipient);
    }

    /// Set the index of the transaction of which the refund should be calculated.
    pub fn set_refund_index(&mut self, refund_index: u64) {
        self.standard_features.refund_index = Some(refund_index);
    }

    /// Set the block builders to forward the bundle to. If not specified, the bundle
    /// will be forwarded to all block builders configured with Echo
    pub fn set_mev_builders(&mut self, mev_builders: Vec<BlockBuilder>) {
        self.echo_features
            .get_or_insert_with(Default::default)
            .mev_builders = Some(mev_builders.into_iter().map(|b| b.to_string()).collect());
    }

    /// Set the boolean flag indicating if the bundle should also be propagated to the public
    /// mempool by using Fiber's internal network (default: false)
    pub fn set_use_public_mempool(&mut self, use_public_mempool: bool) {
        self.echo_features
            .get_or_insert_with(Default::default)
            .use_public_mempool = use_public_mempool;
    }

    /// Set the boolean flag indicating if the HTTP request should hang until the bundle is either
    /// included, or the timeout is reached (default: false)
    pub fn set_await_receipt(&mut self, await_receipt: bool) {
        self.echo_features
            .get_or_insert_with(Default::default)
            .await_receipt = await_receipt;
    }

    /// Set the timeout in milliseconds for the HTTP request to hang until the bundle is either
    /// included, or the timeout is reached
    pub fn set_await_receipt_timeout_ms(&mut self, await_receipt_timeout_ms: u64) {
        self.echo_features
            .get_or_insert_with(Default::default)
            .await_receipt_timeout_ms = await_receipt_timeout_ms;
    }
}

/// Standard bundle features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StandardBundleFeatures {
    /// (Required) Raw bundle transactions as RLP-encoded hex strings.
    pub txs: Vec<String>,

    /// (Required) The block number at which the bundle should be mined.
    /// Encoded as hex string.
    pub block_number: Option<String>,

    /// (Optional) The minimum timestamp at which the bundle should be mined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<u64>,

    /// (Optional) The maximum timestamp at which the bundle should be mined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<u64>,

    /// (Optional) The transaction hashes of transactions that can revert in the bundle,
    /// without which the rest of the bundle can still be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverting_tx_hashes: Option<Vec<String>>,

    /// (Optional) The UUID of the bundle to be replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_uuid: Option<ReplacementUuid>,

    /// (Optional) The percentage of the gas that should be refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_percent: Option<u64>,

    /// (Optional) The address to which the refund should be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_recipient: Option<String>,

    /// (Optional) The index of the transaction of which the refund should be calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_index: Option<u64>,

    /// (Optional) The transaction hashes of which the refund should be calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_tx_hashes: Option<Vec<String>>,
}

/// Echo-specific features and bundle options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EchoBundleFeatures {
    /// The block builders to forward the bundle to. If not specified, the bundle
    /// will be forwarded to all block builders configured with Echo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mev_builders: Option<Vec<String>>,

    /// Boolean flag indicating if the bundle should also be propagated to the public
    /// mempool by using Fiber's internal network (default: false)
    #[serde(default = "bool::default")]
    pub use_public_mempool: bool,

    /// Boolean flag indicating if the HTTP request should hang until the bundle is either
    /// included, or the timeout is reached (default: false)
    #[serde(default = "bool::default")]
    pub await_receipt: bool,

    /// Timeout in milliseconds for the HTTP request to hang until the bundle is either
    /// included, or the timeout is reached
    #[serde(default = "default_await_receipt_timeout_ms")]
    pub await_receipt_timeout_ms: u64,
}

/// A response from the Echo RPC `eth_sendBundle` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleResponse {
    /// The bundle hash that was generated from the request body. Each block builder *can*
    /// generate a different hash for the same bundle, so this is only used for identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_hash: Option<String>,

    /// The receipt notification that can be used to track the bundle's inclusion status (included / timed out)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_notification: Option<BundleNotification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
/// A request to cancel a bundle using the Echo RPC `eth_cancelBundle` endpoint
pub struct CancelBundleArgs {
    /// The UUID of the bundle to be cancelled.
    pub replacement_uuid: ReplacementUuid,

    /// The block builders to which the cancellation request should be forwarded.
    /// If not specified, these will be inferred by existing sendBundle requests with the same
    /// `replacementUuid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mev_builders: Option<Vec<String>>,
}

fn default_await_receipt_timeout_ms() -> u64 {
    30000
}

/// A notification sent from the echo response handler
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status", content = "data")]
#[allow(unused)]
#[allow(missing_docs)]
pub enum BundleNotification {
    Included {
        block_number: u64,
        elapsed_ms: u128,
        block_builder: Option<String>,
    },
    TimedOut {
        elapsed_ms: u128,
    },
}
