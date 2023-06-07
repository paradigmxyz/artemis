use ethers::{
    contract::EthEvent,
    prelude::Lazy,
    types::{Address, TxHash},
};

/// Block number at which the sudo factory was deployed.
pub const FACTORY_DEPLOYMENT_BLOCK: u64 = 14650730;

/// Address of the sudo pair factory.
pub static ARB_CONTRACT_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x82b114ba3e26f895b3dcb174011a06202c6244d6"
        .parse()
        .unwrap()
});
