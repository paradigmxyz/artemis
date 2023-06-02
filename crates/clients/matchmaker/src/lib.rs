#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Client library for Flashbots MEV-share Matchmaker, fulfilling the
//! [MEV-share spec](https://github.com/flashbots/mev-share).

/// Core client implementation
pub mod client;
mod flashbots_signer;
/// Core type definitions for the client
pub mod types;
