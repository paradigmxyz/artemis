use anyhow::Result;
use clap::Parser;
use ethers::types::H160;
use opensea_v2::client::{OpenSeaApiConfig, OpenSeaV2Client};

use ethers::prelude::MiddlewareBuilder;
use ethers::providers::{Provider, Ws};

use artemis_core::collectors::block_collector::BlockCollector;
use artemis_core::collectors::opensea_order_collector::OpenseaOrderCollector;
use artemis_core::executors::mempool_executor::MempoolExecutor;
use ethers::signers::{LocalWallet, Signer};
use opensea_sudo_arb::strategy::OpenseaSudoArb;
use opensea_sudo_arb::types::{Action, Config, Event};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

use std::str::FromStr;
use std::sync::Arc;

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,

    /// Key for the OpenSea API.
    #[arg(long)]
    pub opensea_api_key: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    /// Address of the arb contract.
    #[arg(long)]
    pub arb_contract_address: String,

    /// Percentage of profit to pay in gas.
    #[arg(long)]
    pub bid_percentage: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("opensea_sudo_arb", Level::INFO)
        .with_target("artemis_core", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);

    let wallet: LocalWallet = args.private_key.parse().unwrap();
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet));

    // Set up opensea client.
    let opensea_client = OpenSeaV2Client::new(OpenSeaApiConfig {
        api_key: args.opensea_api_key.clone(),
    });

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up block collector.
    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    // Set up opensea collector.
    let opensea_collector = Box::new(OpenseaOrderCollector::new(args.opensea_api_key));
    let opensea_collector =
        CollectorMap::new(opensea_collector, |e| Event::OpenseaOrder(Box::new(e)));
    engine.add_collector(Box::new(opensea_collector));

    // Set up opensea sudo arb strategy.
    let config = Config {
        arb_contract_address: H160::from_str(&args.arb_contract_address)?,
        bid_percentage: args.bid_percentage,
    };
    let strategy = OpenseaSudoArb::new(Arc::new(provider.clone()), opensea_client, config);
    engine.add_strategy(Box::new(strategy));

    // Set up flashbots executor.
    let executor = Box::new(MempoolExecutor::new(provider.clone()));
    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });
    engine.add_executor(Box::new(executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }
    Ok(())
}
