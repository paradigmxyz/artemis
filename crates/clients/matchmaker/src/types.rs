use std::ops::Add;

use ethers::types::{Bytes, H160, H256, U64};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleRequest {
    pub version: String,
    pub inclusion: Inclusion,
    pub body: Vec<TxReq>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inclusion {
    pub block: U64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxBlock: Option<U64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TxReq {
    TxHash { hash: H256 },
    Tx { tx: Bytes, canRevert: bool },
}

impl BundleRequest {
    pub fn new(blockNum: U64, transactions: Vec<TxReq>) -> Self {
        Self {
            version: "beta-1".to_string(),
            inclusion: Inclusion {
                block: blockNum,
                maxBlock: Some(blockNum.add(5)),
            },
            body: transactions,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendBundleResponse {
    bundleHash: H256,
}
