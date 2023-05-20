use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder,
};

use crate::{
    constants::FULFILL_LISTING_URL,
    types::{FulfillListingRequest, FulfillListingResponse, OpenSeaApiError},
};

//. A partial implementation of the OpenSea API v2, supporting the fulfill listing endpoint.
#[derive(Debug, Clone)]
pub struct OpenSeaV2Client {
    client: Client,
}

/// Configuration for the OpenSea API client.
#[derive(Debug, Clone)]
pub struct OpenSeaApiConfig {
    pub api_key: String,
}

impl OpenSeaV2Client {
    /// Create a new client with the given configuration.
    pub fn new(cfg: OpenSeaApiConfig) -> Self {
        let mut builder = ClientBuilder::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&cfg.api_key).unwrap(),
        );
        builder = builder.default_headers(headers);
        let client = builder.build().unwrap();

        Self { client }
    }

    /// Call the fulfill listing endpoint, which returns the arguments necessary
    /// to fulfill an order onchain.
    pub async fn fulfill_listing(
        &self,
        req: FulfillListingRequest,
    ) -> Result<FulfillListingResponse, OpenSeaApiError> {
        let res = self
            .client
            .post(FULFILL_LISTING_URL)
            .json(&req)
            .send()
            .await?
            .json::<FulfillListingResponse>()
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn can_deserialize_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/sample_response.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: FulfillListingResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.protocol, "seaport1.4");
        assert_eq!(res.fulfillment_data.transaction.value, 1780000000000000000);
    }

    #[test]
    fn can_deserialize_seaport_v5_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/sample_response_1.5.json");
        println!("{}", d.display());
        let res = std::fs::read_to_string(d).unwrap();
        let res: FulfillListingResponse = serde_json::from_str(&res).unwrap();
        assert_eq!(res.protocol, "seaport1.5");
        assert_eq!(res.fulfillment_data.transaction.value, 20000000000000000);
    }
}
