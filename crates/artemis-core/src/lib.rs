#![warn(unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! A library for writing MEV bots, designed to be simple, modular, and fast.
//!
//! At its core, Artemis is architected as an event processing pipeline. The
//! library is made up of three main components:
//!
//! 1. [Collectors](types::Collector): *Collectors* take in external events (such as pending txs,
//!  new blocks, marketplace orders, etc. ) and turn them into an internal
//! *event* representation.
//!
//! 2. [Strategies](types::Strategy): *Strategies* contain the core logic required for each MEV
//! opportunity. They take in *events* as inputs, and compute whether any
//! opportunities are available (for example, a strategy might listen to a stream
//! of marketplace orders to see if there are any cross-exchange arbs). *Strategies*
//! produce *actions*.
//!
//! 3. [Executors](types::Executor): *Executors* process *actions*, and are responsible for executing
//! them in different domains (for example, submitting txs, posting off-chain orders, etc.).
//!
//! These components are tied together by the [Engine](engine::Engine), which is responsible for
//! orchestrating the flow of data between them.

/// This module contains [collector](types::Collector) implementations.
pub mod collectors;
/// This module contains the [Engine](engine::Engine) struct, which is responsible
/// for orchestrating data flows between components
pub mod engine;
/// This module contains [executor](types::Executor) implementations.
pub mod executors;
/// This module contains the core type definitions for Artemis.
pub mod types;
/// This module contains utilities for working with Artemis.
pub mod utilities;
