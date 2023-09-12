//! A strategy implementing probabilistic uniswap v3 / v2 arbitrage on MEV share. At a
//! a high level, we listen to the stream of mev share events, and filter for trades
//! that touch a v3 pool that we have a v2 pool for. We then submit a series of backruns
//! of varying sizes, hoping that one of them will be profitable.

/// This module contains the core strategy implementation.
pub mod strategy;

/// This module contains the core type definitions for the strategy.
pub mod types;
