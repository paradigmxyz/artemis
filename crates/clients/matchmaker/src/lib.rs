#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

//! Client library for Flashbots MEV-share Matchmaker, fulfilling the
//! [MEV-share spec](https://github.com/flashbots/mev-share).
//!
//! # Example
//! ```no_run
//!
//! use ethers::signers::Signer;
//! use ethers::{core::rand::thread_rng, signers::LocalWallet};
//! use ethers::types::{Chain, TransactionRequest, H256, U64};
//! use matchmaker::client::Client;
//! use matchmaker::types::{BundleTx, BundleRequest};
//!
//! # tokio_test::block_on(async {    
//! // The signer used to authenticate bundles
//! let fb_signer = LocalWallet::new(&mut thread_rng());
//!
//! // The signer used for our own transactions
//! let tx_signer = LocalWallet::new(&mut thread_rng());
//!
//! // Set up the client
//! let matchmaker_client = Client::new(fb_signer, Chain::Mainnet);
//!
//! // Hash of the transaction we are trying to backrun
//! let tx_hash = H256::random();
//!
//! // Our own tx that we want to include in the bundle
//! let tx = TransactionRequest::pay("vitalik.eth", 100);
//! let signature = tx_signer.sign_transaction(&tx.clone().into()).await.unwrap();
//! let bytes = tx.rlp_signed(&signature);
//!
//! // Build bundle
//! let mut txs = Vec::new();
//! txs.push(BundleTx::TxHash { hash: tx_hash });
//! txs.push(BundleTx::Tx {
//!     tx: bytes,
//!     can_revert: false,
//! });
//! // block number that we are targeting
//! let block_num = U64::from(100000000);
//! let bundle = BundleRequest::make_simple(block_num, txs);
//!
//! // Send bundle
//! let resp = matchmaker_client.send_bundle(&bundle).await;
//!  println!("Got a bundle response: {:?}", resp);
//! # });
//! ```

/// Core client implementation
pub mod client;
mod flashbots_signer;
/// Core type definitions for the client
pub mod types;
