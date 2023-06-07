use std::sync::Arc;

use anyhow::Result;
use artemis_core::{
    collectors::mevshare_collector::MevShareCollector,
    engine::Engine,
    executors::mev_share_executor::MevshareExecutor,
    types::{Collector, CollectorMap, ExecutorMap},
};
use clap::Parser;
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{transaction::eip2718::TypedTransaction, Chain, TransactionRequest},
};
use futures::StreamExt;
use mev_share_uni_arb::{
    strategy::MevShareUniArb,
    types::{Action, Event},
};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,
    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,
    /// MEV share signer
    #[arg(long)]
    pub flashbots_signer: String,
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

    //  Set up ethers provider.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);

    let wallet: LocalWallet = args.private_key.parse().unwrap();
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));

    // fb signer
    let fb_signer: LocalWallet = args.flashbots_signer.parse().unwrap();

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    let mevshare_collector = Box::new(MevShareCollector::new(String::from(
        "https://mev-share.flashbots.net",
    )));
    let mevshare_collector = CollectorMap::new(mevshare_collector, |e| Event::MEVShareEvent(e));
    engine.add_collector(Box::new(mevshare_collector));

    let strategy = MevShareUniArb::new(Arc::new(provider.clone()), wallet);
    engine.add_strategy(Box::new(strategy));

    //set up executor
    let mev_share_executor = Box::new(MevshareExecutor::new(fb_signer, Chain::Mainnet));
    let mev_share_executor = ExecutorMap::new(mev_share_executor, |action| match action {
        Action::SubmitBundles(bundles) => Some(bundles),
    });
    engine.add_executor(Box::new(mev_share_executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
