# Artemis x Chainbound integration

> This crate gives you access to the [Chainbound][chainbound] suite of tools & services for MEV.
> It is built directly into the [Artemis][artemis] framework for seamless integration with your existing
> trading strategies.

This crate offers two main components, which are implemented following the standard Artemis traits:

- [Fiber][fiber] Collector: a low-latency, reliable `mempool` and `new_blocks` stream for Ethereum.
- [Echo][echo] Executor: a feature-rich RPC endpoint to propagate your MEV bundles to block builders.

## Usage

<details>
<summary>Usage as a dependency in a new Cargo project</summary>

This example assumes you are using a clean Cargo project to implement your strategies.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
artemis-core = { git = "https://github.com/paradigmxyz/artemis.git" }
chainbound-artemis = { git = "https://github.com/paradigmxyz/artemis.git" }

tokio = { version = "1.18", features = ["full"] }
anyhow = "1.0.70"
```

Then, in your `main.rs`:

```rs
use artemis_core::engine::Engine;
use fiber_artemis::{Action, Event, FiberCollector, StreamType};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let fiber_endpoint = "beta.fiberapi.io:8080".into();

    // Join the Fiber Discord at https://discord.com/invite/J4KNdeCYGX
    // or write to <sales@chainbound.io> to get a free trial.
    let fiber_api_key = std::env::var("FIBER_API_KEY")?;

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
    let fiber_collector = FiberCollector::new(fiber_endpoint, fiber_api_key, stream_type).await;

    // And add it to your Artemis engine
    let mut engine: Engine<Event, Action> = Engine::default();
    engine.add_collector(Box::new(fiber_collector));

    // ... Add your strategies and executors here ...

    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            println!("res: {:?}", res);
        }
    }

    Ok(())
}
```

</details>

<details>
<summary>Usage inside the Artemis framework (this repository)</summary>

<!-- TODO -->

</details>

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
