use async_trait::async_trait;
use tokio_stream::StreamExt;
use std::sync::Arc;
use ethers::types::{H256, H160, Log};
use serde::{Deserialize, Serialize};

use eventsource_client::{Client, SSE};

use crate::types::{Collector, CollectorStream};
use anyhow::Result;

/// A collector that listens for new transactions in the mempool, and generates a stream of
/// [events](Transaction) which contain the transaction.
pub struct MevShareCollector {
    mevshare_stream_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub call_data: H256,
    pub function_selector: H256,
    pub to: H160
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MevShareEvent {
    pub hash: H256,
    pub logs: Option<Vec<Log>>,
    pub txs: Option<Vec<Transactions>>,
}

impl MevShareCollector {
    pub fn new(mevshare_stream_url: String) -> Self {
        Self { mevshare_stream_url }
    }
}

/// Implementation of the [Collector](Collector) trait for the [MempoolCollector](MempoolCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new transactions.
// https://mev-share.flashbots.net

#[async_trait]
impl Collector<MevShareEvent> for MevShareCollector
{
    async fn get_event_stream(&self) -> Result<CollectorStream<MevShareEvent>> {
        let client = eventsource_client::ClientBuilder::for_url(&self.mevshare_stream_url).unwrap().build();
        let stream = client.stream();
        let stream = stream.filter_map(|event| {
            match event.unwrap() {
                SSE::Event(evt) => { 
                    let mev_share_event: MevShareEvent = serde_json::from_str(evt.data.as_str()).unwrap();
                    Some(mev_share_event)
                },
                SSE::Comment(_) => None
            }
        });
        Ok(Box::pin(stream))
    }
}
