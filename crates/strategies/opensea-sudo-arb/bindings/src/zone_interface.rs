pub use zone_interface::*;
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
pub mod zone_interface {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOrder\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"validOrderMagicValue\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct AdvancedOrder\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32[]\",\"name\":\"priorOrderHashes\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidOrderIncludingExtraData\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"validOrderMagicValue\",\"type\":\"bytes4\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ZONEINTERFACE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ZoneInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ZoneInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ZoneInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ZoneInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ZoneInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ZoneInterface)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ZoneInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ZONEINTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `isValidOrder` (0x0e1d31dc) function
        pub fn is_valid_order(
            &self,
            order_hash: [u8; 32],
            caller: ::ethers::core::types::Address,
            offerer: ::ethers::core::types::Address,
            zone_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([14, 29, 49, 220], (order_hash, caller, offerer, zone_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidOrderIncludingExtraData` (0x33131570) function
        pub fn is_valid_order_including_extra_data(
            &self,
            order_hash: [u8; 32],
            caller: ::ethers::core::types::Address,
            order: AdvancedOrder,
            prior_order_hashes: ::std::vec::Vec<[u8; 32]>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash(
                    [51, 19, 21, 112],
                    (order_hash, caller, order, prior_order_hashes, criteria_resolvers),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ZoneInterface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isValidOrder` function with signature `isValidOrder(bytes32,address,address,bytes32)` and selector `0x0e1d31dc`
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
        name = "isValidOrder",
        abi = "isValidOrder(bytes32,address,address,bytes32)"
    )]
    pub struct IsValidOrderCall {
        pub order_hash: [u8; 32],
        pub caller: ::ethers::core::types::Address,
        pub offerer: ::ethers::core::types::Address,
        pub zone_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `isValidOrderIncludingExtraData` function with signature `isValidOrderIncludingExtraData(bytes32,address,((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),bytes32[],(uint256,uint8,uint256,uint256,bytes32[])[])` and selector `0x33131570`
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
        name = "isValidOrderIncludingExtraData",
        abi = "isValidOrderIncludingExtraData(bytes32,address,((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),bytes32[],(uint256,uint8,uint256,uint256,bytes32[])[])"
    )]
    pub struct IsValidOrderIncludingExtraDataCall {
        pub order_hash: [u8; 32],
        pub caller: ::ethers::core::types::Address,
        pub order: AdvancedOrder,
        pub prior_order_hashes: ::std::vec::Vec<[u8; 32]>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ZoneInterfaceCalls {
        IsValidOrder(IsValidOrderCall),
        IsValidOrderIncludingExtraData(IsValidOrderIncludingExtraDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for ZoneInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IsValidOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsValidOrder(decoded));
            }
            if let Ok(decoded)
                = <IsValidOrderIncludingExtraDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsValidOrderIncludingExtraData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ZoneInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsValidOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidOrderIncludingExtraData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ZoneInterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsValidOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidOrderIncludingExtraData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsValidOrderCall> for ZoneInterfaceCalls {
        fn from(value: IsValidOrderCall) -> Self {
            Self::IsValidOrder(value)
        }
    }
    impl ::core::convert::From<IsValidOrderIncludingExtraDataCall>
    for ZoneInterfaceCalls {
        fn from(value: IsValidOrderIncludingExtraDataCall) -> Self {
            Self::IsValidOrderIncludingExtraData(value)
        }
    }
    ///Container type for all return fields from the `isValidOrder` function with signature `isValidOrder(bytes32,address,address,bytes32)` and selector `0x0e1d31dc`
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
    pub struct IsValidOrderReturn {
        pub valid_order_magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `isValidOrderIncludingExtraData` function with signature `isValidOrderIncludingExtraData(bytes32,address,((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),bytes32[],(uint256,uint8,uint256,uint256,bytes32[])[])` and selector `0x33131570`
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
    pub struct IsValidOrderIncludingExtraDataReturn {
        pub valid_order_magic_value: [u8; 4],
    }
}
