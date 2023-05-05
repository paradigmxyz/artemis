//! A partial implementation of the Opensea V2 API, supporting the
//! [fulfill listing endpoint](https://docs.opensea.io/reference/fulfill-a-listing).
//! This endpoint is useful for taker stragegies, as it provides the arguments
//! necessary to fulfill orders onchain.

/// This module contains the core client implementation.
pub mod client;

/// This module contains constants used by the client.
mod constants;

/// This module contains the core type definitions for the client.
pub mod types;
