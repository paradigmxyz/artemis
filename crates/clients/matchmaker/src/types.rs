use ethers::types::{Bytes, H256, U64};
use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleRequest {
    pub version: ProtocolVersion,
    pub inclusion: Inclusion,
    pub body: Vec<BundleTx>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusion {
    pub block: U64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block: Option<U64>,
}

/// A bundle tx, which can either be a transaction hash, or a full tx
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum BundleTx {
    TxHash {
        hash: H256,
    },
    #[serde(rename_all = "camelCase")]
    Tx {
        tx: Bytes,
        can_revert: bool,
    },
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleResponse {
    bundle_hash: H256,
}

#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub enum ProtocolVersion {
    #[default]
    #[serde(rename = "beta-1")]
    Beta1,
    #[serde(rename = "v0.1")]
    V0_1,
}

impl BundleRequest {
    pub fn new(
        block_num: U64,
        max_block: Option<U64>,
        version: ProtocolVersion,
        transactions: Vec<BundleTx>,
    ) -> Self {
        Self {
            version: version,
            inclusion: Inclusion {
                block: block_num,
                max_block: max_block,
            },
            body: transactions,
        }
    }

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
