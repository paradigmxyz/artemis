#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! # Chainbound Artemis
//!
//! This crate gives you access to the [Chainbound][chainbound] suite of tools & services for MEV.
//! It is built directly into the [Artemis][artemis] framework for seamless integration with your existing
//! trading strategies.
//!
//! This crate offers two main components, which are implemented following the standard Artemis traits:
//!
//! - Fiber Collector: a low-latency, reliable `mempool` and `new_blocks` stream for Ethereum.
//! - Echo Executor: a feature-rich RPC endpoint to propagate your MEV bundles to block builders.
//!
//! Please refer to the crate README file for an example on how to use these components.

/// Fiber Network client module
pub mod fiber;
pub use fiber::{Event, FiberCollector, StreamType};

/// Echo RPC client module
pub mod echo;
pub use echo::{Action, EchoExecutor};

/// MEV bundle helper types
pub mod mev_bundle;
pub use mev_bundle::{BlockBuilder, MevBundle};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use artemis_core::types::Executor;
    use ethers::{
        prelude::rand,
        providers::Provider,
        signers::{LocalWallet, Signer},
        types::{TransactionRequest, U256},
    };
    use futures::StreamExt;

    use crate::{BlockBuilder, EchoExecutor, Event, FiberCollector, MevBundle, StreamType};

    #[tokio::test]
    async fn test_chainbound_client() {
        if let Ok(api_key) = std::env::var("FIBER_TEST_KEY") {
            let ty = StreamType::Transactions;
            let fiber_tx_collector = FiberCollector::new(api_key.clone(), ty).await;
            let fiber_tx_stream = fiber_tx_collector.get_event_stream().await.unwrap();
            let fiber_tx = fiber_tx_stream.into_future().await.0.unwrap();
            assert!(matches!(fiber_tx, Event::Transaction(_)));

            let provider = Arc::new(Provider::connect("https://eth.llamarpc.com").await.unwrap());
            let tx_signer = LocalWallet::new(&mut rand::thread_rng());
            let auth_signer = LocalWallet::new(&mut rand::thread_rng());
            let account = tx_signer.address();
            let echo_executor = EchoExecutor::new(provider, tx_signer, auth_signer, api_key);

            let tx = TransactionRequest::new()
                .to(account)
                .from(account)
                .value(42)
                .gas_price(U256::from_dec_str("100000000000000000").unwrap());

            let mut bundle = MevBundle::with_txs(vec![tx.into()]);
            bundle.set_mev_builders(vec![BlockBuilder::Flashbots, BlockBuilder::Titan]);

            echo_executor.execute(bundle).await.unwrap();
        } else {
            println!("Skipping test_chainbound_clients because FIBER_TEST_KEY is not set");
        }
    }
}
