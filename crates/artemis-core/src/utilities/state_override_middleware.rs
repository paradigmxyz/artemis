use async_trait::async_trait;
use ethers::{
    core::types::{transaction::eip2718::TypedTransaction, BlockId},
    providers::{spoof, CallBuilder, Middleware, MiddlewareError, RawCall},
    types::{Address, Bytes},
};
use thiserror::Error;

/// This custom middleware performs an ephemeral state override prior to executoring calls.
#[derive(Debug)]
pub struct StateOverrideMiddleware<M> {
    /// The inner middleware
    inner: M,
    /// The state override set we use for calls
    state: spoof::State,
}

impl<M> StateOverrideMiddleware<M>
where
    M: Middleware,
{
    /// Creates an instance of StateOverrideMiddleware
    /// `ìnner` the inner Middleware
    pub fn new(inner: M) -> StateOverrideMiddleware<M> {
        Self {
            inner,
            state: spoof::state(),
        }
    }
}

#[async_trait]
impl<M> Middleware for StateOverrideMiddleware<M>
where
    M: Middleware,
{
    type Error = StateOverrideMiddlewareError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }

    /// Performs a call with the state override.
    async fn call(
        &self,
        tx: &TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<Bytes, Self::Error> {
        let call_builder = CallBuilder::new(self.inner.provider(), tx);
        let call_builder = match block {
            Some(block) => call_builder.block(block),
            None => call_builder,
        };
        let call_builder = call_builder.state(&self.state);
        call_builder
            .await
            .map_err(StateOverrideMiddlewareError::from_provider_err)
    }
}

impl<M> StateOverrideMiddleware<M> {
    /// Adds a code override at a given address.
    pub fn add_code_to_address(&mut self, address: Address, code: Bytes) {
        self.state.account(address).code(code);
    }

    /// Adds a code override at a random address, returning the address.
    pub fn add_code(&mut self, code: Bytes) -> Address {
        let address = Address::random();
        self.state.account(address).code(code);
        address
    }
}

#[derive(Error, Debug)]
pub enum StateOverrideMiddlewareError<M: Middleware> {
    /// Thrown when the internal middleware errors
    #[error("{0}")]
    MiddlewareError(M::Error),
}

impl<M: Middleware> MiddlewareError for StateOverrideMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> Self {
        StateOverrideMiddlewareError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            StateOverrideMiddlewareError::MiddlewareError(e) => Some(e),
        }
    }
}
