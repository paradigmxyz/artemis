use crate::types::{Collector, CollectorStream};
use anyhow::Result;
use async_trait::async_trait;
use opensea_stream::{
    client,
    schema::{self, ItemListedData},
    subscribe_to, Collection, Network,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

/// A collector that listens for new orders on OpenSea, and generates a stream of
/// [events](OpenseaOrder) which contain the order.
#[derive(Default)]
pub struct OpenseaOrderCollector {
    api_key: String,
}

impl OpenseaOrderCollector {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

/// A new order event, containing the internal order.
#[derive(Debug, Clone)]
pub struct OpenseaOrder {
    pub listing: ItemListedData,
}

/// Implementation of the [Collector](Collector) trait for the [OpenseaOrderCollector](OpenseaOrderCollector).
#[async_trait]
impl Collector<OpenseaOrder> for OpenseaOrderCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, OpenseaOrder>> {
        let mut client = client(Network::Mainnet, &self.api_key).await;

        let collection = Collection::All;

        let (_, subscription) = subscribe_to(&mut client, collection).await?;

        let stream = BroadcastStream::new(subscription);

        let stream = stream.filter_map(|event| {
            let event = event.ok()?.into_custom_payload()?;
            if let schema::Payload::ItemListed(listing) = event.payload {
                Some(OpenseaOrder { listing })
            } else {
                None
            }
        });

        Ok(Box::pin(stream))
    }
}
