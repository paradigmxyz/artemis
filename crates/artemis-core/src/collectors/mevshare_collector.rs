use async_trait::async_trait;
use tokio_stream::StreamExt;
use ethers::types::{H256, H160, Log};
use serde::{Deserialize, Serialize};

use eventsource_client::{Client, SSE};

use crate::types::{Collector, CollectorStream};
use anyhow::Result;

/// A collector that listens for new transactions in the mempool, and generates a stream of
/// [events](MevShareEvent) which contain the transaction.
pub struct MevShareCollector {
    mevshare_stream_url: String,
}

// Txs struct for txs object in MevShareEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub call_data: H256,
    pub function_selector: H256,
    pub to: H160
}

// Follows SSE Event Schema - see https://docs.flashbots.net/flashbots-mev-share/searchers/event-stream
/* Schema: {
    hash: string,
    logs?: LogParams[],
    txs: Array<{
        callData?: string,
        functionSelector?: string,
        to?: string,
    }>
}*/
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

/// Implementation of the [Collector](Collector) trait for the
/// [MevShareCollector](MevShareCollector).
/// This implementation uses the Flashbots SSE endpoint to stream events.

#[async_trait]
impl Collector<MevShareEvent> for MevShareCollector
{
    async fn get_event_stream(&self) -> Result<CollectorStream<MevShareEvent>> {
        let client = eventsource_client::ClientBuilder::for_url(&self.mevshare_stream_url).unwrap().build();
        let stream = client.stream();
        let stream = stream.filter_map(|event| match event {
            Ok(SSE::Event(evt)) => { 
                match serde_json::from_str(evt.data.as_str()) {
                    Ok(res) => return res,
                    Err(_) => return None
                };
            },
            Ok(SSE::Comment(_)) => None,
            Err(_) => None
        });
        Ok(Box::pin(stream))
    }
}
