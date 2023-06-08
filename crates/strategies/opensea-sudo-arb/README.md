# Opensea Sudo Arb

A strategy implementing atomic, cross-market NFT arbitrage between Seaport and Sudoswap. At a high level, we listen to a stream of new seaport orders, and compute whether we can atomically fulfill the order and sell the NFT into a sudoswap pool while making a profit. 

## Strategy 

### Sync

The strategy first syncs its initial state, by rebuilding the state of all Sudoswap pools in memory. We do this as follows: 

1. Starting from the Sudoswap factory deployment block, filter for all `NewPair` events emitted to build a full list of pools.
2. We batch read quotes for all pools by using a specialized quoter contract  via eth_call bytecode injection.
3. We update the state of a pair of HashMaps in memory for fast retrival of each NFT collection's best quote. 

### Processing

After the initial sync is done, we stream the following events: 

1. New Blocks: for every new block, we find all sudo pools that were either touched or created, and update their internal state in memory after getting new quotes. 
2. Seaport orders: we stream seaport orders, filtering for sell orders on the collections which have valid sudo quotes. We compute whether an arb is available, and if so, submit a transaction to our atomic arb contract. 

## Contracts 

This strategy relies on two contracts:

1. [`SudoOpenseaArb`](/crates/strategies/opensea-sudo-arb/contracts/src/SudoOpenseaArb.sol): Execute an atomic arb by buying an NFT on seaport by calling `fulfillBasicOrder`, and selling it on Sudoswap by calling `swapNFTsForToken`.

2. [`SudoPairQuoter`](/crates/strategies/opensea-sudo-arb/contracts/src/SudoPairQuoter.sol): Batch read contract that checks whether sudo pools have valid quotes. 

## Build and Test 

In order to run the solidity test, you need access to an alchemy/infura key. You can run tests with the following command: 

```sh
ETH_MAINNET_HTTP=<YOUR_RPC_URL> forge test --root ./contracts
```

You can run the rust tests with the following command: 

```sh
cargo test
```

And if you need to regenerate rust bindings for contracts, you can run 

```sh
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings --overwrite
```