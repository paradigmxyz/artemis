# opensea V2

Partial Rust bindings for the Opensea V2 API, supporting the [fulfill listing endpoint](https://docs.opensea.io/reference/fulfill-a-listing). This endpoint is useful for taker stragegies, as it provides the arguments necessary to fulfill orders onchain. 

## Usage

Instantiate a client with an Opensea API key, and then call `fulfill_listing` with the details of the order you want to fulfill onchain. 


```rs
let client = OpenSeaV2Client::new(OpenSeaApiConfig { api_key });

let req = FulfillListingRequest {
    listing: Listing {
        hash: H256::from_str(
            "0xce83ef67f520d74d081aa4da9588ee6743d3aa64caff98a7dddf214e10469929",
        )
        .unwrap(),
        chain: Chain::Mainnet,
        protocol_version: ProtocolVersion::V1_4,
    },
    fulfiller: Fulfiller {
        address: H160::from_str("0xD77F375A33b1109e82f3C46A30537F1E019708eB").unwrap(),
    },
};
let resp = client.fulfill_listing(req).await;
println!("{:?}", resp);
```


## Building & testing

```
cargo check
cargo test
cargo build [--release]
```

## Acknowledgements

Based on the original [opensea-rs bindings](https://github.com/gakonst/opensea-rs) written by [gakonst](https://twitter.com/gakonst).