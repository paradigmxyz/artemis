pub use transfer_helper_interface::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod transfer_helper_interface {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidItemType\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct TransferHelperItem[]\",\"name\":\"items\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ConduitItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"bulkTransfer\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static TRANSFERHELPERINTERFACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct TransferHelperInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TransferHelperInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TransferHelperInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TransferHelperInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TransferHelperInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(TransferHelperInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TransferHelperInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TRANSFERHELPERINTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `bulkTransfer` (0x73b69d03) function
        pub fn bulk_transfer(
            &self,
            items: ::std::vec::Vec<TransferHelperItem>,
            recipient: ::ethers::core::types::Address,
            conduit_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([115, 182, 157, 3], (items, recipient, conduit_key))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TransferHelperInterface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidItemType` with signature `InvalidItemType()` and selector `0x7932f1fc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidItemType", abi = "InvalidItemType()")]
    pub struct InvalidItemType;
    ///Container type for all input parameters for the `bulkTransfer` function with signature `bulkTransfer((uint8,address,uint256,uint256)[],address,bytes32)` and selector `0x73b69d03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "bulkTransfer",
        abi = "bulkTransfer((uint8,address,uint256,uint256)[],address,bytes32)"
    )]
    pub struct BulkTransferCall {
        pub items: ::std::vec::Vec<TransferHelperItem>,
        pub recipient: ::ethers::core::types::Address,
        pub conduit_key: [u8; 32],
    }
    ///Container type for all return fields from the `bulkTransfer` function with signature `bulkTransfer((uint8,address,uint256,uint256)[],address,bytes32)` and selector `0x73b69d03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BulkTransferReturn(pub [u8; 4]);
}
