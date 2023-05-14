use artemis_core::{
    collectors::{block_collector::BlockCollector, mempool_collector::MempoolCollector},
    executors::mempool_executor::{MempoolExecutor, SubmitTxToMempool},
    types::{Collector, Executor},
};
use ethers::providers::StreamExt;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{BlockNumber, TransactionRequest, U256},
    utils::{Anvil, AnvilInstance},
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

/// Spawns Anvil and instantiates an Http provider.
pub async fn spawn_anvil() -> (Provider<Ws>, AnvilInstance) {
    let anvil = Anvil::new().block_time(1u64).spawn();
    let provider = Provider::<Ws>::connect(anvil.ws_endpoint())
        .await
        .unwrap()
        .interval(Duration::from_millis(50u64));
    (provider, anvil)
}

/// Test that block collector correctly emits blocks.
#[tokio::test]
async fn test_block_collector_sends_blocks() {
    let (provider, _anvil) = spawn_anvil().await;
    let provider = Arc::new(provider);
    let block_collector = BlockCollector::new(provider.clone());
    let block_stream = block_collector.get_event_stream().await.unwrap();
    let block_a = block_stream.into_future().await.0.unwrap();
    let block_b = provider
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(block_a.hash, block_b.hash.unwrap());
}

/// Test that mempool collector correctly emits blocks.
#[tokio::test]
async fn test_mempool_collector_sends_txs() {
    let (provider, _anvil) = spawn_anvil().await;
    let provider = Arc::new(provider);
    let mempool_collector = MempoolCollector::new(provider.clone());
    let mempool_stream = mempool_collector.get_event_stream().await.unwrap();

    let account = provider.get_accounts().await.unwrap()[0];
    let value: u64 = 42;
    let gas_price = U256::from_dec_str("100000000000000000").unwrap();
    let tx = TransactionRequest::new()
        .to(account)
        .from(account)
        .value(value)
        .gas_price(gas_price);

    provider.send_transaction(tx, None).await.unwrap();
    let tx = mempool_stream.into_future().await.0.unwrap();
    assert_eq!(tx.value, value.into());
}

/// Test that the mempool executor correctly sends txs
#[tokio::test]
async fn test_mempool_executor_sends_tx_simple() {
    let (provider, _anvil) = spawn_anvil().await;
    let provider = Arc::new(provider);
    let mempool_executor = MempoolExecutor::new(provider.clone());

    let account = provider.get_accounts().await.unwrap()[0];
    let value: u64 = 42;
    let gas_price = U256::from_dec_str("100000000000000000").unwrap();
    let tx = TransactionRequest::new()
        .to(account)
        .from(account)
        .value(value)
        .gas_price(gas_price);
    let action = SubmitTxToMempool {
        tx: tx.into(),
        gas_bid_info: None,
    };
    mempool_executor.execute(action).await.unwrap();
    //Sleep to seconds so that the tx has time to be mined
    sleep(Duration::from_secs(2)).await;
    let tx = provider.get_transaction_count(account, None).await.unwrap();
    assert_eq!(tx, 1.into());
}
