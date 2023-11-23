use std::fmt;

use ethers::{signers::Signer, types::H256, utils::keccak256};
use serde::{
    de::{self, Visitor},
    Deserializer, Serialize,
};
use serde_json::to_string;

/// Generate a JSON-RPC request string.
pub fn generate_jsonrpc_request<T>(method: &str, params: T) -> String
where
    T: Serialize,
{
    format!(
        r#"{{"id":1,"jsonrpc":"2.0","method":"{}","params":[{}]}}"#,
        method,
        to_string(&params).unwrap()
    )
}

/// Generate a Flashbots signature for a given payload
pub async fn generate_fb_signature<S, T>(signer: &S, payload: T) -> String
where
    S: Signer,
    T: Serialize,
{
    let msg = format!(
        "0x{:x}",
        H256::from(keccak256(
            serde_json::to_string(&payload).unwrap().as_bytes()
        ))
    );

    let signature = signer.sign_message(msg).await.unwrap();

    format!("{:?}:0x{}", signer.address(), signature)
}

/// Serialize an optional u64 into a hex string starting with 0x.
pub fn serialize_opt_u64_as_hex<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(value) = value {
        serializer.serialize_str(&format!("0x{:x}", value))
    } else {
        serializer.serialize_none()
    }
}

/// Deserialize an optional u64 from a hex string starting with 0x.
pub fn deserialize_opt_u64_or_hex<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptU64OrHex;

    impl<'de> Visitor<'de> for OptU64OrHex {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("u64 or string starting with 0x")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if let Some(hex) = v.strip_prefix("0x") {
                u64::from_str_radix(hex, 16)
                    .map_err(de::Error::custom)
                    .map(Some)
            } else {
                Err(de::Error::custom("Expected string to start with 0x"))
            }
        }
    }

    deserializer.deserialize_any(OptU64OrHex)
}
