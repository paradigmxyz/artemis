//! Collectors are responsible for collecting data from external sources and
//! turning them into internal events. For example, a collector might listen to
//! a stream of new blocks, and turn them into a stream of `NewBlock` events.

/// This collector listens to a stream of new blocks.
pub mod block_collector;

/// This collector listens to a stream of new event logs.
pub mod log_collector;

/// This collector listens to a stream of new pending transactions.
pub mod mempool_collector;

/// This collector listens to a stream of new Opensea orders.
pub mod opensea_order_collector;

pub mod mevshare_collector;
