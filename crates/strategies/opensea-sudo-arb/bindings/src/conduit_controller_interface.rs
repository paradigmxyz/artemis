pub use conduit_controller_interface::*;
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
pub mod conduit_controller_interface {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"CallerIsNotNewPotentialOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"CallerIsNotOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"ChannelOutOfRange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"ConduitAlreadyExists\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidCreator\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidInitialOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newPotentialOwner\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"NewPotentialOwnerAlreadySet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"NewPotentialOwnerIsZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoConduit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"NoPotentialOwnerCurrentlySet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewConduit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPotentialOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PotentialOwnerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelOwnershipTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initialOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createConduit\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"channelIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChannel\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChannelStatus\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isOpen\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChannels\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"channels\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConduit\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"exists\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConduitCodeHashes\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"creationCodeHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"runtimeCodeHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getKey\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPotentialOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"potentialOwner\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalChannels\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalChannels\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newPotentialOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"channel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isOpen\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateChannel\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CONDUITCONTROLLERINTERFACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ConduitControllerInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConduitControllerInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConduitControllerInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConduitControllerInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConduitControllerInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ConduitControllerInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConduitControllerInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONDUITCONTROLLERINTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acceptOwnership` (0x51710e45) function
        pub fn accept_ownership(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 113, 14, 69], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOwnershipTransfer` (0x7b37e561) function
        pub fn cancel_ownership_transfer(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 55, 229, 97], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createConduit` (0x794593bc) function
        pub fn create_conduit(
            &self,
            conduit_key: [u8; 32],
            initial_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([121, 69, 147, 188], (conduit_key, initial_owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChannel` (0x027cc764) function
        pub fn get_channel(
            &self,
            conduit: ::ethers::core::types::Address,
            channel_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 124, 199, 100], (conduit, channel_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChannelStatus` (0x33bc8572) function
        pub fn get_channel_status(
            &self,
            conduit: ::ethers::core::types::Address,
            channel: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([51, 188, 133, 114], (conduit, channel))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChannels` (0x8b9e028b) function
        pub fn get_channels(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([139, 158, 2, 139], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConduit` (0x6e9bfd9f) function
        pub fn get_conduit(
            &self,
            conduit_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, bool),
        > {
            self.0
                .method_hash([110, 155, 253, 159], conduit_key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConduitCodeHashes` (0x0a96ad39) function
        pub fn get_conduit_code_hashes(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([10, 150, 173, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getKey` (0x93790f44) function
        pub fn get_key(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([147, 121, 15, 68], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPotentialOwner` (0x906c87cc) function
        pub fn get_potential_owner(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 108, 135, 204], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalChannels` (0x4e3f9580) function
        pub fn get_total_channels(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 63, 149, 128], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x14afd79e) function
        pub fn owner_of(
            &self,
            conduit: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([20, 175, 215, 158], conduit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0x6d435421) function
        pub fn transfer_ownership(
            &self,
            conduit: ::ethers::core::types::Address,
            new_potential_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 67, 84, 33], (conduit, new_potential_owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateChannel` (0x13ad9cab) function
        pub fn update_channel(
            &self,
            conduit: ::ethers::core::types::Address,
            channel: ::ethers::core::types::Address,
            is_open: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 173, 156, 171], (conduit, channel, is_open))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewConduit` event
        pub fn new_conduit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewConduitFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PotentialOwnerUpdated` event
        pub fn potential_owner_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PotentialOwnerUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ConduitControllerInterfaceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConduitControllerInterface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerIsNotNewPotentialOwner` with signature `CallerIsNotNewPotentialOwner(address)` and selector `0x88c3a115`
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
        name = "CallerIsNotNewPotentialOwner",
        abi = "CallerIsNotNewPotentialOwner(address)"
    )]
    pub struct CallerIsNotNewPotentialOwner {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `CallerIsNotOwner` with signature `CallerIsNotOwner(address)` and selector `0xd4ed9a17`
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
    #[etherror(name = "CallerIsNotOwner", abi = "CallerIsNotOwner(address)")]
    pub struct CallerIsNotOwner {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `ChannelOutOfRange` with signature `ChannelOutOfRange(address)` and selector `0x6ceb340b`
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
    #[etherror(name = "ChannelOutOfRange", abi = "ChannelOutOfRange(address)")]
    pub struct ChannelOutOfRange {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `ConduitAlreadyExists` with signature `ConduitAlreadyExists(address)` and selector `0x6328ccb2`
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
    #[etherror(name = "ConduitAlreadyExists", abi = "ConduitAlreadyExists(address)")]
    pub struct ConduitAlreadyExists {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidCreator` with signature `InvalidCreator()` and selector `0xcb6e5344`
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
    #[etherror(name = "InvalidCreator", abi = "InvalidCreator()")]
    pub struct InvalidCreator;
    ///Custom Error type `InvalidInitialOwner` with signature `InvalidInitialOwner()` and selector `0x99faaa04`
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
    #[etherror(name = "InvalidInitialOwner", abi = "InvalidInitialOwner()")]
    pub struct InvalidInitialOwner;
    ///Custom Error type `NewPotentialOwnerAlreadySet` with signature `NewPotentialOwnerAlreadySet(address,address)` and selector `0xcbc080ca`
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
        name = "NewPotentialOwnerAlreadySet",
        abi = "NewPotentialOwnerAlreadySet(address,address)"
    )]
    pub struct NewPotentialOwnerAlreadySet {
        pub conduit: ::ethers::core::types::Address,
        pub new_potential_owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `NewPotentialOwnerIsZeroAddress` with signature `NewPotentialOwnerIsZeroAddress(address)` and selector `0xa388d263`
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
        name = "NewPotentialOwnerIsZeroAddress",
        abi = "NewPotentialOwnerIsZeroAddress(address)"
    )]
    pub struct NewPotentialOwnerIsZeroAddress {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `NoConduit` with signature `NoConduit()` and selector `0x4ca82090`
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
    #[etherror(name = "NoConduit", abi = "NoConduit()")]
    pub struct NoConduit;
    ///Custom Error type `NoPotentialOwnerCurrentlySet` with signature `NoPotentialOwnerCurrentlySet(address)` and selector `0x6b013616`
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
        name = "NoPotentialOwnerCurrentlySet",
        abi = "NoPotentialOwnerCurrentlySet(address)"
    )]
    pub struct NoPotentialOwnerCurrentlySet {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConduitControllerInterfaceErrors {
        CallerIsNotNewPotentialOwner(CallerIsNotNewPotentialOwner),
        CallerIsNotOwner(CallerIsNotOwner),
        ChannelOutOfRange(ChannelOutOfRange),
        ConduitAlreadyExists(ConduitAlreadyExists),
        InvalidCreator(InvalidCreator),
        InvalidInitialOwner(InvalidInitialOwner),
        NewPotentialOwnerAlreadySet(NewPotentialOwnerAlreadySet),
        NewPotentialOwnerIsZeroAddress(NewPotentialOwnerIsZeroAddress),
        NoConduit(NoConduit),
        NoPotentialOwnerCurrentlySet(NoPotentialOwnerCurrentlySet),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConduitControllerInterfaceErrors {
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
                = <CallerIsNotNewPotentialOwner as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerIsNotNewPotentialOwner(decoded));
            }
            if let Ok(decoded)
                = <CallerIsNotOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerIsNotOwner(decoded));
            }
            if let Ok(decoded)
                = <ChannelOutOfRange as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChannelOutOfRange(decoded));
            }
            if let Ok(decoded)
                = <ConduitAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConduitAlreadyExists(decoded));
            }
            if let Ok(decoded)
                = <InvalidCreator as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidCreator(decoded));
            }
            if let Ok(decoded)
                = <InvalidInitialOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidInitialOwner(decoded));
            }
            if let Ok(decoded)
                = <NewPotentialOwnerAlreadySet as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NewPotentialOwnerAlreadySet(decoded));
            }
            if let Ok(decoded)
                = <NewPotentialOwnerIsZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NewPotentialOwnerIsZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <NoConduit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoConduit(decoded));
            }
            if let Ok(decoded)
                = <NoPotentialOwnerCurrentlySet as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoPotentialOwnerCurrentlySet(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConduitControllerInterfaceErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerIsNotNewPotentialOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerIsNotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChannelOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConduitAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCreator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewPotentialOwnerAlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewPotentialOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoConduit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoPotentialOwnerCurrentlySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ConduitControllerInterfaceErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerIsNotNewPotentialOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerIsNotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ChannelOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ConduitAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCreator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidInitialOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewPotentialOwnerAlreadySet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NewPotentialOwnerIsZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoConduit as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NoPotentialOwnerCurrentlySet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConduitControllerInterfaceErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerIsNotNewPotentialOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallerIsNotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChannelOutOfRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConduitAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCreator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPotentialOwnerAlreadySet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewPotentialOwnerIsZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoConduit(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoPotentialOwnerCurrentlySet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for ConduitControllerInterfaceErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerIsNotNewPotentialOwner>
    for ConduitControllerInterfaceErrors {
        fn from(value: CallerIsNotNewPotentialOwner) -> Self {
            Self::CallerIsNotNewPotentialOwner(value)
        }
    }
    impl ::core::convert::From<CallerIsNotOwner> for ConduitControllerInterfaceErrors {
        fn from(value: CallerIsNotOwner) -> Self {
            Self::CallerIsNotOwner(value)
        }
    }
    impl ::core::convert::From<ChannelOutOfRange> for ConduitControllerInterfaceErrors {
        fn from(value: ChannelOutOfRange) -> Self {
            Self::ChannelOutOfRange(value)
        }
    }
    impl ::core::convert::From<ConduitAlreadyExists>
    for ConduitControllerInterfaceErrors {
        fn from(value: ConduitAlreadyExists) -> Self {
            Self::ConduitAlreadyExists(value)
        }
    }
    impl ::core::convert::From<InvalidCreator> for ConduitControllerInterfaceErrors {
        fn from(value: InvalidCreator) -> Self {
            Self::InvalidCreator(value)
        }
    }
    impl ::core::convert::From<InvalidInitialOwner>
    for ConduitControllerInterfaceErrors {
        fn from(value: InvalidInitialOwner) -> Self {
            Self::InvalidInitialOwner(value)
        }
    }
    impl ::core::convert::From<NewPotentialOwnerAlreadySet>
    for ConduitControllerInterfaceErrors {
        fn from(value: NewPotentialOwnerAlreadySet) -> Self {
            Self::NewPotentialOwnerAlreadySet(value)
        }
    }
    impl ::core::convert::From<NewPotentialOwnerIsZeroAddress>
    for ConduitControllerInterfaceErrors {
        fn from(value: NewPotentialOwnerIsZeroAddress) -> Self {
            Self::NewPotentialOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoConduit> for ConduitControllerInterfaceErrors {
        fn from(value: NoConduit) -> Self {
            Self::NoConduit(value)
        }
    }
    impl ::core::convert::From<NoPotentialOwnerCurrentlySet>
    for ConduitControllerInterfaceErrors {
        fn from(value: NoPotentialOwnerCurrentlySet) -> Self {
            Self::NoPotentialOwnerCurrentlySet(value)
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
    #[ethevent(name = "NewConduit", abi = "NewConduit(address,bytes32)")]
    pub struct NewConduitFilter {
        pub conduit: ::ethers::core::types::Address,
        pub conduit_key: [u8; 32],
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub conduit: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "PotentialOwnerUpdated", abi = "PotentialOwnerUpdated(address)")]
    pub struct PotentialOwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub new_potential_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConduitControllerInterfaceEvents {
        NewConduitFilter(NewConduitFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PotentialOwnerUpdatedFilter(PotentialOwnerUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ConduitControllerInterfaceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewConduitFilter::decode_log(log) {
                return Ok(ConduitControllerInterfaceEvents::NewConduitFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    ConduitControllerInterfaceEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PotentialOwnerUpdatedFilter::decode_log(log) {
                return Ok(
                    ConduitControllerInterfaceEvents::PotentialOwnerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ConduitControllerInterfaceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewConduitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PotentialOwnerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<NewConduitFilter> for ConduitControllerInterfaceEvents {
        fn from(value: NewConduitFilter) -> Self {
            Self::NewConduitFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for ConduitControllerInterfaceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PotentialOwnerUpdatedFilter>
    for ConduitControllerInterfaceEvents {
        fn from(value: PotentialOwnerUpdatedFilter) -> Self {
            Self::PotentialOwnerUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership(address)` and selector `0x51710e45`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership(address)")]
    pub struct AcceptOwnershipCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cancelOwnershipTransfer` function with signature `cancelOwnershipTransfer(address)` and selector `0x7b37e561`
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
        name = "cancelOwnershipTransfer",
        abi = "cancelOwnershipTransfer(address)"
    )]
    pub struct CancelOwnershipTransferCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createConduit` function with signature `createConduit(bytes32,address)` and selector `0x794593bc`
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
    #[ethcall(name = "createConduit", abi = "createConduit(bytes32,address)")]
    pub struct CreateConduitCall {
        pub conduit_key: [u8; 32],
        pub initial_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getChannel` function with signature `getChannel(address,uint256)` and selector `0x027cc764`
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
    #[ethcall(name = "getChannel", abi = "getChannel(address,uint256)")]
    pub struct GetChannelCall {
        pub conduit: ::ethers::core::types::Address,
        pub channel_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChannelStatus` function with signature `getChannelStatus(address,address)` and selector `0x33bc8572`
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
    #[ethcall(name = "getChannelStatus", abi = "getChannelStatus(address,address)")]
    pub struct GetChannelStatusCall {
        pub conduit: ::ethers::core::types::Address,
        pub channel: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getChannels` function with signature `getChannels(address)` and selector `0x8b9e028b`
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
    #[ethcall(name = "getChannels", abi = "getChannels(address)")]
    pub struct GetChannelsCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getConduit` function with signature `getConduit(bytes32)` and selector `0x6e9bfd9f`
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
    #[ethcall(name = "getConduit", abi = "getConduit(bytes32)")]
    pub struct GetConduitCall {
        pub conduit_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getConduitCodeHashes` function with signature `getConduitCodeHashes()` and selector `0x0a96ad39`
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
    #[ethcall(name = "getConduitCodeHashes", abi = "getConduitCodeHashes()")]
    pub struct GetConduitCodeHashesCall;
    ///Container type for all input parameters for the `getKey` function with signature `getKey(address)` and selector `0x93790f44`
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
    #[ethcall(name = "getKey", abi = "getKey(address)")]
    pub struct GetKeyCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPotentialOwner` function with signature `getPotentialOwner(address)` and selector `0x906c87cc`
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
    #[ethcall(name = "getPotentialOwner", abi = "getPotentialOwner(address)")]
    pub struct GetPotentialOwnerCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTotalChannels` function with signature `getTotalChannels(address)` and selector `0x4e3f9580`
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
    #[ethcall(name = "getTotalChannels", abi = "getTotalChannels(address)")]
    pub struct GetTotalChannelsCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(address)` and selector `0x14afd79e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(address)")]
    pub struct OwnerOfCall {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address,address)` and selector `0x6d435421`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address,address)")]
    pub struct TransferOwnershipCall {
        pub conduit: ::ethers::core::types::Address,
        pub new_potential_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateChannel` function with signature `updateChannel(address,address,bool)` and selector `0x13ad9cab`
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
    #[ethcall(name = "updateChannel", abi = "updateChannel(address,address,bool)")]
    pub struct UpdateChannelCall {
        pub conduit: ::ethers::core::types::Address,
        pub channel: ::ethers::core::types::Address,
        pub is_open: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConduitControllerInterfaceCalls {
        AcceptOwnership(AcceptOwnershipCall),
        CancelOwnershipTransfer(CancelOwnershipTransferCall),
        CreateConduit(CreateConduitCall),
        GetChannel(GetChannelCall),
        GetChannelStatus(GetChannelStatusCall),
        GetChannels(GetChannelsCall),
        GetConduit(GetConduitCall),
        GetConduitCodeHashes(GetConduitCodeHashesCall),
        GetKey(GetKeyCall),
        GetPotentialOwner(GetPotentialOwnerCall),
        GetTotalChannels(GetTotalChannelsCall),
        OwnerOf(OwnerOfCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateChannel(UpdateChannelCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConduitControllerInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded)
                = <CancelOwnershipTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelOwnershipTransfer(decoded));
            }
            if let Ok(decoded)
                = <CreateConduitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateConduit(decoded));
            }
            if let Ok(decoded)
                = <GetChannelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChannel(decoded));
            }
            if let Ok(decoded)
                = <GetChannelStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetChannelStatus(decoded));
            }
            if let Ok(decoded)
                = <GetChannelsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChannels(decoded));
            }
            if let Ok(decoded)
                = <GetConduitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConduit(decoded));
            }
            if let Ok(decoded)
                = <GetConduitCodeHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetConduitCodeHashes(decoded));
            }
            if let Ok(decoded)
                = <GetKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetKey(decoded));
            }
            if let Ok(decoded)
                = <GetPotentialOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPotentialOwner(decoded));
            }
            if let Ok(decoded)
                = <GetTotalChannelsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTotalChannels(decoded));
            }
            if let Ok(decoded)
                = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UpdateChannelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateChannel(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConduitControllerInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelOwnershipTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateConduit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChannel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChannelStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChannels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConduit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConduitCodeHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPotentialOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalChannels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateChannel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConduitControllerInterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOwnershipTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateConduit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChannel(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChannelStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChannels(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConduit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConduitCodeHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPotentialOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalChannels(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateChannel(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for ConduitControllerInterfaceCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<CancelOwnershipTransferCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: CancelOwnershipTransferCall) -> Self {
            Self::CancelOwnershipTransfer(value)
        }
    }
    impl ::core::convert::From<CreateConduitCall> for ConduitControllerInterfaceCalls {
        fn from(value: CreateConduitCall) -> Self {
            Self::CreateConduit(value)
        }
    }
    impl ::core::convert::From<GetChannelCall> for ConduitControllerInterfaceCalls {
        fn from(value: GetChannelCall) -> Self {
            Self::GetChannel(value)
        }
    }
    impl ::core::convert::From<GetChannelStatusCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: GetChannelStatusCall) -> Self {
            Self::GetChannelStatus(value)
        }
    }
    impl ::core::convert::From<GetChannelsCall> for ConduitControllerInterfaceCalls {
        fn from(value: GetChannelsCall) -> Self {
            Self::GetChannels(value)
        }
    }
    impl ::core::convert::From<GetConduitCall> for ConduitControllerInterfaceCalls {
        fn from(value: GetConduitCall) -> Self {
            Self::GetConduit(value)
        }
    }
    impl ::core::convert::From<GetConduitCodeHashesCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: GetConduitCodeHashesCall) -> Self {
            Self::GetConduitCodeHashes(value)
        }
    }
    impl ::core::convert::From<GetKeyCall> for ConduitControllerInterfaceCalls {
        fn from(value: GetKeyCall) -> Self {
            Self::GetKey(value)
        }
    }
    impl ::core::convert::From<GetPotentialOwnerCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: GetPotentialOwnerCall) -> Self {
            Self::GetPotentialOwner(value)
        }
    }
    impl ::core::convert::From<GetTotalChannelsCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: GetTotalChannelsCall) -> Self {
            Self::GetTotalChannels(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ConduitControllerInterfaceCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for ConduitControllerInterfaceCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateChannelCall> for ConduitControllerInterfaceCalls {
        fn from(value: UpdateChannelCall) -> Self {
            Self::UpdateChannel(value)
        }
    }
    ///Container type for all return fields from the `createConduit` function with signature `createConduit(bytes32,address)` and selector `0x794593bc`
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
    pub struct CreateConduitReturn {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getChannel` function with signature `getChannel(address,uint256)` and selector `0x027cc764`
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
    pub struct GetChannelReturn {
        pub channel: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getChannelStatus` function with signature `getChannelStatus(address,address)` and selector `0x33bc8572`
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
    pub struct GetChannelStatusReturn {
        pub is_open: bool,
    }
    ///Container type for all return fields from the `getChannels` function with signature `getChannels(address)` and selector `0x8b9e028b`
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
    pub struct GetChannelsReturn {
        pub channels: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getConduit` function with signature `getConduit(bytes32)` and selector `0x6e9bfd9f`
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
    pub struct GetConduitReturn {
        pub conduit: ::ethers::core::types::Address,
        pub exists: bool,
    }
    ///Container type for all return fields from the `getConduitCodeHashes` function with signature `getConduitCodeHashes()` and selector `0x0a96ad39`
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
    pub struct GetConduitCodeHashesReturn {
        pub creation_code_hash: [u8; 32],
        pub runtime_code_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getKey` function with signature `getKey(address)` and selector `0x93790f44`
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
    pub struct GetKeyReturn {
        pub conduit_key: [u8; 32],
    }
    ///Container type for all return fields from the `getPotentialOwner` function with signature `getPotentialOwner(address)` and selector `0x906c87cc`
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
    pub struct GetPotentialOwnerReturn {
        pub potential_owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getTotalChannels` function with signature `getTotalChannels(address)` and selector `0x4e3f9580`
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
    pub struct GetTotalChannelsReturn {
        pub total_channels: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(address)` and selector `0x14afd79e`
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
    pub struct OwnerOfReturn {
        pub owner: ::ethers::core::types::Address,
    }
}
