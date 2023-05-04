pub use conduit_interface::*;
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
pub mod conduit_interface {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"ChannelClosed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isOpen\",\"type\":\"bool\",\"components\":[]}],\"type\":\"error\",\"name\":\"ChannelStatusAlreadySet\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidController\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidItemType\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"open\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChannelUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct ConduitTransfer[]\",\"name\":\"transfers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ConduitItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ConduitBatch1155Transfer[]\",\"name\":\"batch1155Transfers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeBatch1155\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ConduitTransfer[]\",\"name\":\"standardTransfers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ConduitItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConduitBatch1155Transfer[]\",\"name\":\"batch1155Transfers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeWithBatch1155\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isOpen\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateChannel\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CONDUITINTERFACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ConduitInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConduitInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConduitInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConduitInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConduitInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ConduitInterface)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConduitInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONDUITINTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `execute` (0x4ce34aa2) function
        pub fn execute(
            &self,
            transfers: ::std::vec::Vec<ConduitTransfer>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([76, 227, 74, 162], transfers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch1155` (0x8df25d92) function
        pub fn execute_batch_1155(
            &self,
            batch_1155_transfers: ::std::vec::Vec<ConduitBatch1155Transfer>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([141, 242, 93, 146], batch_1155_transfers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithBatch1155` (0x899e104c) function
        pub fn execute_with_batch_1155(
            &self,
            standard_transfers: ::std::vec::Vec<ConduitTransfer>,
            batch_1155_transfers: ::std::vec::Vec<ConduitBatch1155Transfer>,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash(
                    [137, 158, 16, 76],
                    (standard_transfers, batch_1155_transfers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateChannel` (0xc4e8fcb5) function
        pub fn update_channel(
            &self,
            channel: ::ethers::core::types::Address,
            is_open: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 232, 252, 181], (channel, is_open))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChannelUpdated` event
        pub fn channel_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChannelUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChannelUpdatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConduitInterface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ChannelClosed` with signature `ChannelClosed(address)` and selector `0x93daadf2`
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
    #[etherror(name = "ChannelClosed", abi = "ChannelClosed(address)")]
    pub struct ChannelClosed {
        pub channel: ::ethers::core::types::Address,
    }
    ///Custom Error type `ChannelStatusAlreadySet` with signature `ChannelStatusAlreadySet(address,bool)` and selector `0x924e341e`
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
    #[etherror(
        name = "ChannelStatusAlreadySet",
        abi = "ChannelStatusAlreadySet(address,bool)"
    )]
    pub struct ChannelStatusAlreadySet {
        pub channel: ::ethers::core::types::Address,
        pub is_open: bool,
    }
    ///Custom Error type `InvalidController` with signature `InvalidController()` and selector `0x6d5769be`
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
    #[etherror(name = "InvalidController", abi = "InvalidController()")]
    pub struct InvalidController;
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConduitInterfaceErrors {
        ChannelClosed(ChannelClosed),
        ChannelStatusAlreadySet(ChannelStatusAlreadySet),
        InvalidController(InvalidController),
        InvalidItemType(InvalidItemType),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConduitInterfaceErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ChannelClosed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChannelClosed(decoded));
            }
            if let Ok(decoded)
                = <ChannelStatusAlreadySet as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ChannelStatusAlreadySet(decoded));
            }
            if let Ok(decoded)
                = <InvalidController as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidController(decoded));
            }
            if let Ok(decoded)
                = <InvalidItemType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidItemType(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConduitInterfaceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ChannelClosed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChannelStatusAlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidItemType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ConduitInterfaceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ChannelClosed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ChannelStatusAlreadySet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidItemType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConduitInterfaceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChannelClosed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChannelStatusAlreadySet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidController(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidItemType(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ConduitInterfaceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ChannelClosed> for ConduitInterfaceErrors {
        fn from(value: ChannelClosed) -> Self {
            Self::ChannelClosed(value)
        }
    }
    impl ::core::convert::From<ChannelStatusAlreadySet> for ConduitInterfaceErrors {
        fn from(value: ChannelStatusAlreadySet) -> Self {
            Self::ChannelStatusAlreadySet(value)
        }
    }
    impl ::core::convert::From<InvalidController> for ConduitInterfaceErrors {
        fn from(value: InvalidController) -> Self {
            Self::InvalidController(value)
        }
    }
    impl ::core::convert::From<InvalidItemType> for ConduitInterfaceErrors {
        fn from(value: InvalidItemType) -> Self {
            Self::InvalidItemType(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ChannelUpdated", abi = "ChannelUpdated(address,bool)")]
    pub struct ChannelUpdatedFilter {
        #[ethevent(indexed)]
        pub channel: ::ethers::core::types::Address,
        pub open: bool,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute((uint8,address,address,address,uint256,uint256)[])` and selector `0x4ce34aa2`
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
        name = "execute",
        abi = "execute((uint8,address,address,address,uint256,uint256)[])"
    )]
    pub struct ExecuteCall {
        pub transfers: ::std::vec::Vec<ConduitTransfer>,
    }
    ///Container type for all input parameters for the `executeBatch1155` function with signature `executeBatch1155((address,address,address,uint256[],uint256[])[])` and selector `0x8df25d92`
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
        name = "executeBatch1155",
        abi = "executeBatch1155((address,address,address,uint256[],uint256[])[])"
    )]
    pub struct ExecuteBatch1155Call {
        pub batch_1155_transfers: ::std::vec::Vec<ConduitBatch1155Transfer>,
    }
    ///Container type for all input parameters for the `executeWithBatch1155` function with signature `executeWithBatch1155((uint8,address,address,address,uint256,uint256)[],(address,address,address,uint256[],uint256[])[])` and selector `0x899e104c`
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
        name = "executeWithBatch1155",
        abi = "executeWithBatch1155((uint8,address,address,address,uint256,uint256)[],(address,address,address,uint256[],uint256[])[])"
    )]
    pub struct ExecuteWithBatch1155Call {
        pub standard_transfers: ::std::vec::Vec<ConduitTransfer>,
        pub batch_1155_transfers: ::std::vec::Vec<ConduitBatch1155Transfer>,
    }
    ///Container type for all input parameters for the `updateChannel` function with signature `updateChannel(address,bool)` and selector `0xc4e8fcb5`
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
    #[ethcall(name = "updateChannel", abi = "updateChannel(address,bool)")]
    pub struct UpdateChannelCall {
        pub channel: ::ethers::core::types::Address,
        pub is_open: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConduitInterfaceCalls {
        Execute(ExecuteCall),
        ExecuteBatch1155(ExecuteBatch1155Call),
        ExecuteWithBatch1155(ExecuteWithBatch1155Call),
        UpdateChannel(UpdateChannelCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConduitInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <ExecuteBatch1155Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteBatch1155(decoded));
            }
            if let Ok(decoded)
                = <ExecuteWithBatch1155Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteWithBatch1155(decoded));
            }
            if let Ok(decoded)
                = <UpdateChannelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateChannel(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConduitInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch1155(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithBatch1155(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateChannel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConduitInterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithBatch1155(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateChannel(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for ConduitInterfaceCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatch1155Call> for ConduitInterfaceCalls {
        fn from(value: ExecuteBatch1155Call) -> Self {
            Self::ExecuteBatch1155(value)
        }
    }
    impl ::core::convert::From<ExecuteWithBatch1155Call> for ConduitInterfaceCalls {
        fn from(value: ExecuteWithBatch1155Call) -> Self {
            Self::ExecuteWithBatch1155(value)
        }
    }
    impl ::core::convert::From<UpdateChannelCall> for ConduitInterfaceCalls {
        fn from(value: UpdateChannelCall) -> Self {
            Self::UpdateChannel(value)
        }
    }
    ///Container type for all return fields from the `execute` function with signature `execute((uint8,address,address,address,uint256,uint256)[])` and selector `0x4ce34aa2`
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
    pub struct ExecuteReturn {
        pub magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `executeBatch1155` function with signature `executeBatch1155((address,address,address,uint256[],uint256[])[])` and selector `0x8df25d92`
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
    pub struct ExecuteBatch1155Return {
        pub magic_value: [u8; 4],
    }
    ///Container type for all return fields from the `executeWithBatch1155` function with signature `executeWithBatch1155((uint8,address,address,address,uint256,uint256)[],(address,address,address,uint256[],uint256[])[])` and selector `0x899e104c`
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
    pub struct ExecuteWithBatch1155Return {
        pub magic_value: [u8; 4],
    }
}
