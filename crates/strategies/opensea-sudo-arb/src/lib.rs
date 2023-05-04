//! A strategy implementing atomic, cross-market NFT arbitrage between 
//! Seaport and Sudoswap. At a high level, we listen to a stream of new seaport orders, 
//! and compute whether we can atomically fulfill the order and sell the NFT into a 
//! sudoswap pool while making a profit. 

/// This module contains constants used by the strategy.
pub mod constants;

/// This module contains the core strategy implementation.
pub mod strategy;

/// This module contains the core type definitions for the strategy.
pub mod types;
