// a backtesting framework for mev share
// 1. Parse bundles from a file
// 2. Get block for tx we are trying to backrun
// 3. For chain at given tx hash
// 4. Replay backrun tx

use std::{fs::File, path::Path, sync::Arc};

use ethers::{
    providers::{Http, Middleware, Provider, ProviderExt},
    types::{Transaction, TransactionRequest},
    utils::rlp::{self, Decodable, Rlp},
};
use foundry_evm::{
    executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder},
    revm::primitives::Bytes,
};
use matchmaker::types::{BundleRequest, BundleTx};

#[tokio::main]
async fn main() {
    let bundles_json = r#"
    [
        {
            "version": "beta-1",
            "inclusion": {
                "block": "0x109369C",
                "maxBlock": "0x10936A1"
            },
            "body": [
                {
                    "hash": "0x701bc27a17b2127db7ac14856998c799fa3a52b477c12863218d6ed02dc9ced6"
                },
                {
                    "tx": "0x02f8f2010a851851c1b1a0851851c1b1a083061a809482b114ba3e26f895b3dcb174011a06202c6244d680b88465c8053b0000000000000000000000008982390fe1c4f6a46f612732516b451efd00a9a0000000000000000000000000dce93ed9ae7c53143e19cf799d156b72d1cc2777000000000000000000000000000000000000000000000000000000e8d4a510000000000000000000000000000000000000000000000000000000000000000000c001a0485899ea2471008ab2187e80040157fd6474bd333c8ff6ec94280e04315893aaa04aa2d50cf9ff34ef4f9bad307eae74dca8bc4e536d7e896551eebaa8450df192",
                    "canRevert": false
                }
            ]
        }
    ]"#;

    let bundles = serde_json::from_str::<Vec<BundleRequest>>(bundles_json).unwrap();
    println!("{:?}", bundles);

    let url = " https://eth-mainnet.g.alchemy.com/v2/Pw87LhLIa3Wm1725pmmWvs3J7jWgG6b1".to_string();
    let provider = Arc::new(Provider::<Http>::connect(&url).await);
    simulate_bundle(provider, bundles[0].clone(), url).await;
}

async fn simulate_bundle(provider: Arc<Provider<Http>>, bundle: BundleRequest, fork_url: String) {
    let tx_to_backrun = bundle.body[0].clone();
    let tx_hash = match tx_to_backrun {
        BundleTx::TxHash { hash } => hash,
        _ => panic!("Expected TxHash"),
    };

    let tx = provider.get_transaction(tx_hash).await.unwrap();

    let tx = match tx {
        Some(tx) => tx,
        None => {
            println!("Tx not found");
            return;
        }
    };

    let block_num = tx.block_number.unwrap();

    let evm_opts = EvmOpts {
        fork_url: Some(fork_url.clone()),
        fork_block_number: Some(block_num.as_u64() - 1),
        env: foundry_evm::executor::opts::Env {
            chain_id: None,
            code_size_limit: None,
            gas_price: Some(0),
            gas_limit: 1000000000,
            ..Default::default()
        },
        memory_limit: 1000000000,
        ..Default::default()
    };

    let fork_opts = CreateFork {
        url: fork_url,
        enable_caching: true,
        env: evm_opts.evm_env_blocking().unwrap(),
        evm_opts,
    };

    let db = Backend::spawn(Some(fork_opts.clone()));

    // investigate why we need to explicitly set gas limit to u64::MAX
    let mut executor = ExecutorBuilder::default()
        .with_gas_limit(u64::MAX.into())
        .build(db);

    let bytes = Bytes::copy_from_slice(tx.input.as_ref());

    executor.call_raw_committing(tx.from, tx.to.unwrap_or_default(), bytes, tx.value);

    let tx_bytes = bundle.body[1].clone();
    let tx_bytes = match tx_bytes {
        BundleTx::Tx { tx, can_revert: _ } => tx,
        _ => panic!("Expected Tx"),
    };

    let mut decoded_tx = rlp::decode::<Transaction>(&tx_bytes).unwrap();
    println!("decoded_tx: {:?}", decoded_tx);

    let bytes = Bytes::copy_from_slice(decoded_tx.input.as_ref());

    let res = executor.call_raw(
        decoded_tx.from,
        decoded_tx.to.unwrap(),
        bytes,
        decoded_tx.value,
    );
    println!("res: {:?}", res);
}
