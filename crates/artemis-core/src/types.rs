use anyhow::Result;
use async_trait::async_trait;
use ethers::types::Transaction;
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

use crate::collectors::block_collector::NewBlock;
use crate::collectors::opensea_order_collector::OpenseaOrder;
use crate::executors::flashbots_executor::FlashbotsBundle;
use crate::executors::mempool_executor::SubmitTxToMempool;

/// A stream of events emitted by a [Collector](Collector).
pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

/// Collector trait, which defines a source of events.
#[async_trait]
pub trait Collector<E>: Send + Sync {
    /// Returns the core event stream for the collector.
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>>;
}

/// Strategy trait, which defines the core logic for each opportunity.
#[async_trait]
pub trait Strategy<E, A>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data.
    async fn sync_state(&mut self) -> Result<()>;

    /// Process an event, and return an action if needed.
    async fn process_event(&mut self, event: E) -> Vec<A>;
}

/// Executor trait, responsible for executing actions returned by strategies.
#[async_trait]
pub trait Executor<A>: Send + Sync {
    /// Execute an action.
    async fn execute(&self, action: A) -> Result<()>;
}

/// CollectorMap is a wrapper around a [Collector](Collector) that maps outgoing
/// events to a different type.
pub struct CollectorMap<E, F> {
    collector: Box<dyn Collector<E>>,
    f: F,
}
impl<E, F> CollectorMap<E, F> {
    pub fn new(collector: Box<dyn Collector<E>>, f: F) -> Self {
        Self { collector, f }
    }
}

#[async_trait]
impl<E1, E2, F> Collector<E2> for CollectorMap<E1, F>
where
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
    F: Fn(E1) -> E2 + Send + Sync + Clone + 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E2>> {
        let stream = self.collector.get_event_stream().await?;
        let f = self.f.clone();
        let stream = stream.map(f);
        Ok(Box::pin(stream))
    }
}

/// ExecutorMap is a wrapper around an [Executor](Executor) that maps incoming
/// actions to a different type.
pub struct ExecutorMap<A, F> {
    executor: Box<dyn Executor<A>>,
    f: F,
}

impl<A, F> ExecutorMap<A, F> {
    pub fn new(executor: Box<dyn Executor<A>>, f: F) -> Self {
        Self { executor, f }
    }
}

#[async_trait]
impl<A1, A2, F> Executor<A1> for ExecutorMap<A2, F>
where
    A1: Send + Sync + 'static,
    A2: Send + Sync + 'static,
    F: Fn(A1) -> Option<A2> + Send + Sync + Clone + 'static,
{
    async fn execute(&self, action: A1) -> Result<()> {
        let action = (self.f)(action);
        match action {
            Some(action) => self.executor.execute(action).await,
            None => Ok(()),
        }
    }
}

/// Convenience enum containing all the events that can be emitted by collectors.
pub enum Events {
    NewBlock(NewBlock),
    Transaction(Transaction),
    OpenseaOrder(Box<OpenseaOrder>),
}

/// Convenience enum containing all the actions that can be executed by executors.
pub enum Actions {
    FlashbotsBundle(FlashbotsBundle),
    SubmitTxToMempool(SubmitTxToMempool),
}
