pub mod fiber;
pub use fiber::{Event, FiberCollector, StreamType};

pub mod echo;
pub use echo::EchoExecutor;

pub mod mev_bundle;
pub use mev_bundle::MevBundle;

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

    use crate::{mev_bundle::Builder, EchoExecutor, Event, FiberCollector, MevBundle, StreamType};

    #[tokio::test]
    pub async fn test_chainbound_client() {
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
            bundle.set_mev_builders(vec![Builder::Flashbots, Builder::Titan]);

            echo_executor.execute(bundle).await.unwrap();
        } else {
            println!("Skipping test_chainbound_clients because FIBER_TEST_KEY is not set");
        }
    }
}
