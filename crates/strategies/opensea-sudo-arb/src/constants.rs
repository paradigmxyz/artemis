use ethers::{
    contract::EthEvent,
    prelude::Lazy,
    types::{Address, TxHash},
};

/// Block number at which the sudo factory was deployed.
pub const FACTORY_DEPLOYMENT_BLOCK: u64 = 14650730;

/// Address of the sudo pair factory.
pub static LSSVM_PAIR_FACTORY_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0xb16c1342e617a5b6e4b631eb114483fdb289c0a4"
        .parse()
        .unwrap()
});

/// Group of event signatures which are emitted when a pool is touched.
pub static POOL_EVENT_SIGNATURES: Lazy<Vec<TxHash>> = Lazy::new(|| {
    vec![
        bindings::lssvm_pair::SwapNFTInPairFilter::signature(),
        bindings::lssvm_pair::SwapNFTInPairFilter::signature(),
        bindings::lssvm_pair::SpotPriceUpdateFilter::signature(),
        bindings::lssvm_pair::TokenWithdrawalFilter::signature(),
    ]
});
