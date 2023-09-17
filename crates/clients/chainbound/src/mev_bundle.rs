use ethers::{
    types::{transaction::eip2718::TypedTransaction, Bytes, TxHash},
    utils::hex,
};

#[derive(Debug, Default)]
pub enum BlockBuilder {
    Flashbots,
    Beaverbuild,
    Rsync,
    Builder0x69,
    Titan,
    F1b,
    Blocknative,
    Nfactorial,
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

/// Complete bundle interface, including Echo-specific features.
/// See the full specs and their meaning here: <https://echo.chainbound.io/docs/usage/api-interface>
#[derive(Debug, Default)]
pub struct MevBundle {
    pub txs: Vec<TypedTransaction>,
    pub signed_txs: Vec<Bytes>,
    pub block_number: Option<u64>,
    pub min_timestamp: Option<u64>,
    pub max_timestamp: Option<u64>,
    pub reverting_tx_hashes: Option<Vec<TxHash>>,
    pub replacement_uuid: Option<String>,
    pub mev_builders: Option<Vec<BlockBuilder>>,
    pub use_public_mempool: Option<bool>,
    pub await_receipt: Option<bool>,
    pub await_receipt_timeout_ms: Option<u64>,
}

impl MevBundle {
    pub fn with_txs(txs: Vec<TypedTransaction>) -> Self {
        Self {
            txs,
            ..Default::default()
        }
    }

    pub fn add_signed_tx(&mut self, tx: Bytes) -> &mut Self {
        self.signed_txs.push(tx);
        self
    }

    pub fn set_block_number(&mut self, block_number: u64) -> &mut Self {
        self.block_number = Some(block_number);
        self
    }

    pub fn set_min_timestamp(&mut self, min_timestamp: u64) -> &mut Self {
        self.min_timestamp = Some(min_timestamp);
        self
    }

    pub fn set_max_timestamp(&mut self, max_timestamp: u64) -> &mut Self {
        self.max_timestamp = Some(max_timestamp);
        self
    }

    pub fn set_reverting_tx_hashes(&mut self, reverting_tx_hashes: Vec<TxHash>) -> &mut Self {
        self.reverting_tx_hashes = Some(reverting_tx_hashes);
        self
    }

    pub fn set_replacement_uuid(&mut self, replacement_uuid: String) -> &mut Self {
        self.replacement_uuid = Some(replacement_uuid);
        self
    }

    pub fn set_mev_builders(&mut self, mev_builders: Vec<BlockBuilder>) -> &mut Self {
        self.mev_builders = Some(mev_builders);
        self
    }

    pub fn set_use_public_mempool(&mut self, use_public_mempool: bool) -> &mut Self {
        self.use_public_mempool = Some(use_public_mempool);
        self
    }

    pub fn set_await_receipt(&mut self, await_receipt: bool) -> &mut Self {
        self.await_receipt = Some(await_receipt);
        self
    }

    pub fn set_await_receipt_timeout_ms(&mut self, await_receipt_timeout_ms: u64) -> &mut Self {
        self.await_receipt_timeout_ms = Some(await_receipt_timeout_ms);
        self
    }

    /// Returns the bundle as a JSON string, ready to be sent to the Echo RPC format.
    /// We don't use the `Serialize` trait in order to have more control over the
    /// final formatting.
    ///
    /// ## Arguments
    /// `include_echo_features`: If true, the resulting String will be formatted according to the
    /// Flashbots API specs. E.g. without the Echo-specific fields. This is useful for creating
    /// the `X-Flashbots-Signature` header for authentication purposes.
    pub fn format_json_body(&self, include_echo_features: bool) -> String {
        let mut json = String::from("{");

        // Check basic requirements
        if self.signed_txs.is_empty() {
            panic!("No signed transactions in bundle");
        }
        if self.block_number.is_none() {
            panic!("No block number in bundle");
        }

        // Add signed transactions
        json.push_str("\"txs\":[");
        for (i, tx) in self.signed_txs.iter().enumerate() {
            json.push_str(&format!("\"0x{}\"", hex::encode(tx)));
            if i < self.signed_txs.len() - 1 {
                json.push(',');
            }
        }

        // Add block number
        json.push_str(&format!(",\"blockNumber\":{}", self.block_number.unwrap()));

        // Add min timestamp if present
        if let Some(min_timestamp) = self.min_timestamp {
            json.push_str(&format!(",\"minTimestamp\":{}", min_timestamp));
        }

        // Add max timestamp if present
        if let Some(max_timestamp) = self.max_timestamp {
            json.push_str(&format!(",\"maxTimestamp\":{}", max_timestamp));
        }

        // Add reverting tx hashes if present
        if let Some(reverting_tx_hashes) = &self.reverting_tx_hashes {
            json.push_str("\"revertingTxHashes\":[");
            for (i, tx_hash) in reverting_tx_hashes.iter().enumerate() {
                json.push_str(&format!("\"0x{}\"", hex::encode(tx_hash)));
                if i < reverting_tx_hashes.len() - 1 {
                    json.push(',');
                }
            }
            json.push(']');
        }

        // Add replacement UUID if present
        if let Some(replacement_uuid) = &self.replacement_uuid {
            json.push_str(&format!(",\"replacementUUID\":\"{}\"", replacement_uuid));
        }

        if !include_echo_features {
            // Skip the Echo-specific fields and return the JSON string
            json.push('}');
            json
        } else {
            // Add MEV builders if present
            if let Some(mev_builders) = &self.mev_builders {
                json.push_str("\"mevBuilders\":[");
                for (i, mev_builder) in mev_builders.iter().enumerate() {
                    json.push_str(&format!("\"{}\"", mev_builder.to_string()));
                    if i < mev_builders.len() - 1 {
                        json.push(',');
                    }
                }
                json.push(']');
            }

            // Add use public mempool if present
            if let Some(use_public_mempool) = &self.use_public_mempool {
                json.push_str(&format!(",\"usePublicMempool\":{}", use_public_mempool));
            }

            // Add await receipt if present
            if let Some(await_receipt) = &self.await_receipt {
                json.push_str(&format!(",\"awaitReceipt\":{}", await_receipt));
            }

            // Add await receipt timeout if present
            if let Some(await_receipt_timeout_ms) = &self.await_receipt_timeout_ms {
                json.push_str(&format!(
                    ",\"awaitReceiptTimeoutMs\":{}",
                    await_receipt_timeout_ms
                ));
            }

            json.push('}');

            json
        }
    }

    /// Returns the bundle as a JSON-RPC request string, ready to be sent to the Echo endpoint.
    pub fn format_json_rpc_request(&self, method: &str, include_echo_features: bool) -> String {
        format!(
            "{{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"{}\",\"params\":[{}]}}",
            method,
            self.format_json_body(include_echo_features)
        )
    }
}
