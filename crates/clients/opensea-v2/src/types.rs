use std::str::FromStr;

use ethers::types::{Bytes, Chain, H160, H256, U256};
use serde::{de, Deserialize, Serialize, Serializer};
use thiserror::Error;

use super::constants::{SEAPORT_V1, SEAPORT_V4, SEAPORT_V5};

/// Request to fulfill a listing on OpenSea.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillListingRequest {
    pub listing: Listing,
    pub fulfiller: Fulfiller,
}

/// Listing we want to fulfill on OpenSea.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub hash: H256,
    #[serde(serialize_with = "chain_to_str")]
    pub chain: Chain,
    #[serde(
        rename = "protocol_address",
        serialize_with = "protocol_version_to_str"
    )]
    pub protocol_version: ProtocolVersion,
}

/// Address which will fulfill the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fulfiller {
    pub address: H160,
}

/// Response from OpenSea fulfill listing endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillListingResponse {
    pub protocol: String,
    pub fulfillment_data: FulfillmentData,
}

/// Protocol version for the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProtocolVersion {
    V1_1,
    V1_4,
    V1_5,
}

/// Information needed to fulfill the listing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FulfillmentData {
    pub transaction: Transaction,
}

/// Transaction data for onchain fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub function: String,
    pub chain: u64,
    pub to: String,
    pub value: u64,
    pub input_data: InputData,
}

/// Additional input data for the transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputData {
    pub parameters: Parameters,
}

/// Parameters for onchain transaction fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    pub consideration_token: H160,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub consideration_identifier: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub consideration_amount: U256,
    pub offerer: H160,
    pub zone: H160,
    pub offer_token: H160,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub offer_identifier: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub offer_amount: U256,
    pub basic_order_type: u8,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub start_time: U256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub end_time: U256,
    pub zone_hash: H256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub salt: U256,
    pub offerer_conduit_key: H256,
    pub fulfiller_conduit_key: H256,
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub total_original_additional_recipients: U256,
    pub additional_recipients: Vec<AdditionalRecipient>,
    #[serde(deserialize_with = "bytes_from_str")]
    pub signature: Bytes,
}

/// Additional recipient for onchain transaction fulfillment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdditionalRecipient {
    #[serde(deserialize_with = "u256_from_dec_str")]
    pub amount: U256,
    pub recipient: H160,
}

/// Error returned by the OpenSea API.
#[derive(Debug, Error)]
pub enum OpenSeaApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// Helper function to convert a chain to a string.
fn chain_to_str<S: Serializer>(chain: &Chain, serializer: S) -> Result<S::Ok, S::Error> {
    let chain_str = match chain {
        Chain::Mainnet => "ethereum",
        _ => Err(serde::ser::Error::custom("Unsupported chain"))?,
    };
    serializer.serialize_str(chain_str)
}

/// Helper function to convert a protocol version to a string.
fn protocol_version_to_str<S: Serializer>(
    protocol_version: &ProtocolVersion,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let protocol_version_str = match protocol_version {
        ProtocolVersion::V1_1 => SEAPORT_V1,
        ProtocolVersion::V1_4 => SEAPORT_V4,
        ProtocolVersion::V1_5 => SEAPORT_V5,
    };
    serializer.serialize_str(protocol_version_str)
}

/// Helper function to convert a string to bytes.
fn bytes_from_str<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: de::Deserializer<'de>,
{
    let val = String::deserialize(deserializer)?;
    Bytes::from_str(&val).map_err(de::Error::custom)
}

/// Helper function to convert a decimal string to a U256.
fn u256_from_dec_str<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: de::Deserializer<'de>,
{
    let val = String::deserialize(deserializer)?;
    U256::from_dec_str(&val).map_err(de::Error::custom)
}
