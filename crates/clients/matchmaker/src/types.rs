use ethers::types::{Bytes, H256, U64};
use serde::{Deserialize, Serialize};

/// A bundle of transactions to send to the matchmaker.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleRequest {
    /// The version of the MEV-share API to use.
    pub version: ProtocolVersion,
    /// Data used by block builders to check if the bundle should be considered for inclusion.
    pub inclusion: Inclusion,
    /// The transactions to include in the bundle.
    pub body: Vec<BundleTx>,
}

/// Data used by block builders to check if the bundle should be considered for inclusion.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusion {
    /// The first block the bundle is valid for.
    pub block: U64,
    /// The last block the bundle is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block: Option<U64>,
}

/// A bundle tx, which can either be a transaction hash, or a full tx.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum BundleTx {
    /// The hash of the transaction we are trying to backrun.
    TxHash {
        /// Tx hash.
        hash: H256,
    },
    /// A new signed transaction.
    #[serde(rename_all = "camelCase")]
    Tx {
        /// Bytes of the signed transaction.
        tx: Bytes,
        /// If true, the transaction can revert without the bundle being considered invalid.
        can_revert: bool,
    },
}

/// Response from the matchmaker after sending a bundle.
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleResponse {
    /// Hash of the bundle bodies.
    bundle_hash: H256,
}

/// The version of the MEV-share API to use.
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub enum ProtocolVersion {
    #[default]
    #[serde(rename = "beta-1")]
    /// The beta-1 version of the API.
    Beta1,
    /// The 0.1 version of the API.
    #[serde(rename = "v0.1")]
    V0_1,
}

impl BundleRequest {
    /// Create a new bundle request.
    pub fn new(
        block_num: U64,
        max_block: Option<U64>,
        version: ProtocolVersion,
        transactions: Vec<BundleTx>,
    ) -> Self {
        Self {
            version,
            inclusion: Inclusion {
                block: block_num,
                max_block,
            },
            body: transactions,
        }
    }

    /// Helper function to create a simple bundle request with sensible defaults (bundle is valid for the next 5 blocks).
    pub fn make_simple(block_num: U64, transactions: Vec<BundleTx>) -> Self {
        // bundle is valid for 5 blocks
        let max_block = block_num.saturating_add(U64::from(5));
        Self::new(
            block_num,
            Some(max_block),
            ProtocolVersion::Beta1,
            transactions,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::types::BundleRequest;

    #[test]
    fn can_deserialize() {
        let str = r#"
        [{
            "version": "v0.1",
            "inclusion": {
                "block": "0x1"
            },
            "body": [{
                "tx": "0x02f86b0180843b9aca00852ecc889a0082520894c87037874aed04e51c29f582394217a0a2b89d808080c080a0a463985c616dd8ee17d7ef9112af4e6e06a27b071525b42182fe7b0b5c8b4925a00af5ca177ffef2ff28449292505d41be578bebb77110dfc09361d2fb56998260",
                "canRevert": false
            }]
        }]
        "#;
        let res: Result<Vec<BundleRequest>, _> = serde_json::from_str(str);
        assert!(res.is_ok());
    }
}
