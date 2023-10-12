# Artemis x Chainbound integration

> This crate gives you access to the [Chainbound][chainbound] suite of tools & services for MEV.
> It is built directly into the [Artemis][artemis] framework for seamless integration with your existing
> trading strategies.

This crate offers two main components, which are implemented following the standard Artemis traits:

- [Fiber][fiber] Collector: a low-latency, reliable `mempool` and `new_blocks` stream for Ethereum.
- [Echo][echo] Executor: a feature-rich RPC endpoint to propagate your MEV bundles to block builders.

## Usage

This example assumes you are using a new crate to implement your strategies.

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
artemis-core = { git = "https://github.com/paradigmxyz/artemis.git" }
chainbound-artemis = { git = "https://github.com/paradigmxyz/artemis.git" }

# the following dependencies are also used in this example
ethers = {  version = "2", features = ["ws", "rustls"] }
tokio = { version = "1.18", features = ["full"] }
anyhow = "1.0.70"
```

Then, in your `main.rs`:

```rs
use std::sync::Arc;

use artemis_core::{engine::Engine, types::ExecutorMap};
use chainbound_artemis::{Action, EchoExecutor, Event, FiberCollector, StreamType};
use ethers::{prelude::rand, providers::Provider, signers::LocalWallet};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Join the Chainbound Discord at https://discord.com/invite/J4KNdeCYGX
    // or write to <sales@chainbound.io> to get a free trial.
    let api_key = std::env::var("CHAINBOUND_API_KEY")?;

    // You can select your desired object type to stream here.
    // Please refer to the documentation at https://fiber.chainbound.io/docs/intro for more details.
    //
    // Possible values are:
    // - StreamType::Transactions: new pending transactions in the mempool
    // - StreamType::ExecutionHeaders: new execution headers (blocks) without the transactions
    // - StreamType::ExecutionPayloads: new blocks with header + all transactions included
    // - StreamType::BeaconBlocks: new beacon blocks (ETH2 consensus-layer blocks)
    let stream_type = StreamType::Transactions;

    // Simply create a new collector
    let fiber_collector = Box::new(FiberCollector::new(api_key.clone(), stream_type).await);

    // Now create the Echo Executor to send your bundles to your desired block builders.
    // We also need to instantiate a regular HTTP provider middleware, and two signers
    // (one to actually sign the transactions, one for Flashbots' authentication header)
    //
    // For more info, please refer to the documentation at https://echo.chainbound.io/docs/architecture
    let provider = Arc::new(Provider::connect("wss://eth.llamarpc.com").await.unwrap());
    let tx_signer = LocalWallet::new(&mut rand::thread_rng()); // or any other signer
    let auth_signer = LocalWallet::new(&mut rand::thread_rng()); // or any other signer
    let echo_executor = Box::new(EchoExecutor::new(provider, tx_signer, auth_signer, api_key));

    let executor_map = ExecutorMap::new(echo_executor, |action| match action {
        Action::SendBundle(bundle) => Some(bundle),
    });

    // And add these components to your Artemis engine
    let mut engine: Engine<Event, Action> = Engine::default();
    engine.add_collector(fiber_collector);
    engine.add_executor(Box::new(executor_map));

    // --- bootstrap your trading strategy here ---

    // Finally, run the engine
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            println!("res: {:?}", res);
        }
    }

    Ok(())
}
```

## Useful Links

- [Chainbound website][chainbound]
- [Fiber documentation][fiber-docs]
- [Echo documentation][echo-docs]
- [Chainbound Discord][discord]
- [Chainbound Twitter][twitter]

[artemis]: https://github.com/paradigmxyz/artemis
[chainbound]: https://chainbound.io/
[echo]: https://echo.chainbound.io/
[fiber]: https://fiber.chainbound.io/
[fiber-docs]: https://fiber.chainbound.io/docs/intro
[echo-docs]: https://echo.chainbound.io/docs/architecture
[discord]: https://discord.com/invite/J4KNdeCYGX
[twitter]: https://twitter.com/chainbound_
