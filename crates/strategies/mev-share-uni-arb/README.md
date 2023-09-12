# MEV Share Uniswap V2 / V3 Arbitrage

A strategy implementing probabilistic Uniswap V2 / V3 arbitrage on Mev Share. At a high level, we listen to the stream of mev share events, and filter for trades that touch a v3 pool. We then submit a series of backruns of varying sizes, hoping that one of them will be profitable.

## Strategy 

### Sync

The strategy first syncs its initial state, by loading the set of valid pools into memory. These are pools where one asset in the pair is WETH, and which exist on both uniswap v2 and v3. 

### Processing

After the initial sync is done, we stream MEV-Share events, listening for transactions that touch one of the revelant pools. When we find these transactions, we submit a series of backruns, blindly guessing the trade size.  


## Contracts 

This strategy relies on an atomic arb contract which can be found [here](./contracts/src/BlindArb.sol)

## Build and Test 

In order to run the solidity test, you need access to an alchemy/infura key. You can run tests with the following command: 

```sh
ETH_MAINNET_HTTP=<YOUR_KEY> forge test --root ./contracts
```

You can run the rust tests with the following command: 

```sh
cargo test
```

And if you need to regenerate rust bindings for contracts, you can run 

```sh
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings
```